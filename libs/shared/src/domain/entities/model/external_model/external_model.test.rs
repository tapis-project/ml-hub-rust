use evaluations::{FieldPath, ResolveValue};
use serde_json::{json, Map, Value};

use super::{
    DerivedMetadata, ExternalModel, ExternalModelError, HuggingFaceRepoLocator, ModelLocator,
    ModelMetadata, ModelProvider, TapisSystemLocator,
};
use crate::shared_kernel::{constants::GLOBAL_TENANT, identifiers::traits::UrnGenerator};

fn metadata() -> ModelMetadata {
    let derived = match DerivedMetadata::new(
        Some("model".into()),
        Some("author".into()),
        vec!["transformers".into()],
        vec!["tag".into()],
        Vec::new(),
        None,
        12,
        false,
        false,
        None,
        None,
    ) {
        Ok(derived) => derived,
        Err(error) => panic!("metadata should be valid: {error}"),
    };

    let mut canonical = Map::new();

    canonical.insert("private_source_field".into(), Value::Bool(true));

    ModelMetadata::new(derived, canonical)
}

#[test]
fn creates_global_external_model_and_preserves_metadata() {
    let locator = match HuggingFaceRepoLocator::new("owner/repo".into(), "sha".into()) {
        Ok(locator) => locator,
        Err(error) => panic!("locator should be valid: {error}"),
    };

    let model = match ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::HuggingFace(locator),
        metadata(),
    ) {
        Ok(model) => model,
        Err(error) => panic!("ExternalModel should be valid: {error}"),
    };

    assert_eq!(model.metadata().derived().size(), 12);
    assert_eq!(model.updated_at(), model.created_at());
    assert_eq!(
        model.urn().to_string(),
        format!("urn:mlhub:v1:{GLOBAL_TENANT}:external_model:{}", model.id())
    );
}

#[test]
fn rejects_provider_locator_mismatch() {
    let locator = match TapisSystemLocator::new(
        "site".into(),
        "tenant".into(),
        "system".into(),
        "/path".into(),
    ) {
        Ok(locator) => locator,
        Err(error) => panic!("locator should be valid: {error}"),
    };

    let result = ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::Tapis(locator),
        metadata(),
    );

    assert!(matches!(
        result,
        Err(ExternalModelError::ProviderLocatorMismatch)
    ));
}

#[test]
fn rejects_empty_provider_locator_fields() {
    let result = HuggingFaceRepoLocator::new(String::new(), "sha".into());

    assert!(result.is_err());
}

#[test]
fn resolves_canonical_values_and_absent_values_as_null() -> Result<(), Box<dyn std::error::Error>> {
    let locator = HuggingFaceRepoLocator::new("owner/repo".into(), "sha".into())?;
    let mut metadata = metadata();

    metadata.canonical.insert("id".into(), json!("owner/repo"));
    metadata
        .canonical
        .insert("config".into(), json!({"quantization_config": {"bits": 4}}));
    metadata
        .canonical
        .insert("siblings".into(), json!([{"name": "weights.safetensors"}]));

    let model = ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::HuggingFace(locator),
        metadata,
    )?;

    let id = model.resolve_value(Some(FieldPath::new(vec![
        "metadata".into(),
        "canonical".into(),
        "id".into(),
    ])))?;

    let quantization = model.resolve_value(Some(FieldPath::new(vec![
        "metadata".into(),
        "canonical".into(),
        "config".into(),
        "quantization_config".into(),
    ])))?;

    let sibling_name = model.resolve_value(Some(FieldPath::new(vec![
        "metadata".into(),
        "canonical".into(),
        "siblings".into(),
        "0".into(),
        "name".into(),
    ])))?;

    let missing = model.resolve_value(Some(FieldPath::new(vec![
        "metadata".into(),
        "canonical".into(),
        "gguf".into(),
    ])))?;

    assert_eq!(Value::from(id), json!("owner/repo"));
    assert_eq!(Value::from(quantization), json!({"bits": 4}));
    assert_eq!(Value::from(sibling_name), json!("weights.safetensors"));
    assert_eq!(Value::from(missing), Value::Null);

    Ok(())
}
