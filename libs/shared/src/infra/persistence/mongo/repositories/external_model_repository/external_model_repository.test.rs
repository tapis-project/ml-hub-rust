use platforms::Platform;

use super::*;
use crate::application::inputs::discover_models::DeploymentStrategyCriterion;

#[test]
fn empty_criterion_matches_the_global_catalog() -> Result<(), ExternalModelRepositoryError> {
    let filter = criterion_filter(&SearchCriterion::default())?;

    assert!(filter.is_empty());

    Ok(())
}

#[test]
fn criterion_combines_fields_and_uses_case_insensitive_exact_array_matches(
) -> Result<(), Box<dyn std::error::Error>> {
    let criterion = SearchCriterion {
        provider: Some(ModelProvider::HuggingFace),
        name: Some("Model.*".into()),
        inference_runtimes: vec!["Transformers".into(), "vLLM".into()],
        size: NumericRange {
            min: Some(10),
            max: Some(20),
        },
        deployment_strategies: vec![DeploymentStrategyCriterion {
            name: "GPU.*".into(),
            platform: Platform::TapisJobs,
        }],
        has_deployment_strategies: Some(true),
        ..Default::default()
    };

    let filter = criterion_filter(&criterion)?;

    let name = match filter.get("metadata.derived.name") {
        Some(Bson::RegularExpression(regex)) => regex,
        other => panic!("expected name regex, found {other:?}"),
    };

    let runtimes = filter
        .get_document("metadata.derived.inference_runtimes")?
        .get_array("$in")?;

    let size = filter.get_document("metadata.derived.size")?;

    let strategy_conditions = filter.get_array("$and")?;

    assert_eq!(filter.get_str("provider")?, "HuggingFace");
    assert_eq!(name.pattern, "^Model\\.\\*$");
    assert_eq!(name.options, "i");
    assert_eq!(runtimes.len(), 2);
    assert!(runtimes.iter().all(|runtime| matches!(
        runtime,
        Bson::RegularExpression(regex) if regex.options == "i"
    )));
    assert_eq!(size.get_i64("$gte")?, 10);
    assert_eq!(size.get_i64("$lte")?, 20);
    assert_eq!(strategy_conditions.len(), 2);

    Ok(())
}
