use serde_json::{Map, Value};

use crate::{
    domain::entities::model::{
        external_model::{
            DerivedMetadata, ExternalModel, HuggingFaceRepoLocator, ModelLocator, ModelMetadata,
            ModelProvider,
        },
        Model,
    },
    shared_kernel::{
        enums::{Task, Visibility},
        identifiers::ExternalModelId,
    },
};

pub fn full_external_model() -> ExternalModel {
    let derived = match DerivedMetadata::new(
        Some("foo".into()),
        Some("bar".into()),
        vec!["transformers".into(), "diffusers".into()],
        vec!["text-generation".into(), "transformers".into()],
        vec![Task::ImageClassification],
        Some("MIT".into()),
        1024,
        false,
        true,
        Some(2),
        Some(3),
    ) {
        Ok(derived) => derived,
        Err(error) => panic!("fixture metadata should be valid: {error}"),
    };

    let mut canonical = Map::new();

    canonical.insert("source".into(), Value::String("hugging-face".into()));

    let locator = match HuggingFaceRepoLocator::new("bar/foo".into(), "abc123".into()) {
        Ok(locator) => locator,
        Err(error) => panic!("fixture locator should be valid: {error}"),
    };

    match ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::HuggingFace(locator),
        ModelMetadata::new(derived, canonical),
    ) {
        Ok(model) => model,
        Err(error) => panic!("fixture ExternalModel should be valid: {error}"),
    }
}

pub fn full_model() -> Model {
    match Model::create(
        "tenant".into(),
        "owner".into(),
        "foo".into(),
        Some("description".into()),
        ExternalModelId::new(),
        Visibility::Private,
    ) {
        Ok(model) => model,
        Err(error) => panic!("fixture Model should be valid: {error}"),
    }
}
