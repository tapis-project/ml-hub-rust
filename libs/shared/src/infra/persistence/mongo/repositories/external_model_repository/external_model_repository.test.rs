use super::*;
use crate::domain::entities::deployment_option::ServingRuntime;
use crate::shared_kernel::enums::DeploymentModality;

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

    Ok(())
}

#[test]
fn deployment_option_fields_match_the_same_option() -> Result<(), Box<dyn std::error::Error>> {
    let hpc_cluster_id = uuid::Uuid::now_v7();

    let batch_scheduler_queue_id = uuid::Uuid::now_v7();

    let criterion = SearchCriterion {
        serving_runtimes: vec![ServingRuntime::FlexServ],
        hpc_cluster_ids: vec![hpc_cluster_id],
        batch_scheduler_queue_ids: vec![batch_scheduler_queue_id],
        supported_deployment_modalities: vec![DeploymentModality::Batch],
        has_deployment_options: Some(true),
        ..Default::default()
    };

    let filter = criterion_filter(&criterion)?;

    let conditions = filter.get_array("$and")?;

    let option_condition = conditions[0].as_document().ok_or("option condition")?;

    let option_filter = option_condition
        .get_document("deployment_options")?
        .get_document("$elemMatch")?;

    assert_eq!(
        option_filter
            .get_document("serving_runtime")?
            .get_array("$in")?
            .len(),
        1
    );
    assert_eq!(
        option_filter
            .get_document("hpc_cluster_queue.hpc_cluster_id")?
            .get_array("$in")?
            .len(),
        1
    );
    assert_eq!(
        option_filter
            .get_document("hpc_cluster_queue.batch_scheduler_queue_id")?
            .get_array("$in")?
            .len(),
        1
    );
    assert_eq!(conditions.len(), 2);

    Ok(())
}

#[test]
fn has_no_deployment_options_uses_empty_array_match() -> Result<(), Box<dyn std::error::Error>> {
    let criterion = SearchCriterion {
        has_deployment_options: Some(false),
        ..Default::default()
    };

    let filter = criterion_filter(&criterion)?;

    let conditions = filter.get_array("$and")?;

    let condition = conditions[0].as_document().ok_or("option condition")?;

    assert_eq!(
        condition
            .get_document("deployment_options")?
            .get_i32("$size")?,
        0
    );

    Ok(())
}
