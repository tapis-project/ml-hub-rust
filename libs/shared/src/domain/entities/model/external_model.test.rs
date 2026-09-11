use serde_json::{Map, Value};

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
