use clients::ModelConversionClient;
use serde_json::json;
use shared::domain::entities::model::external_model::{
    ModelLocator, ModelProvider,
};

use super::HuggingFaceClient;

fn source_model() -> serde_json::Value {
    json!({
        "author": "author",
        "id": "author/model",
        "library_name": "Transformers",
        "pipeline_tag": "unknown-future-task",
        "tags": ["transformers", "ONNX", "license:apache-2.0", "text-generation"],
        "gated": false,
        "private": false,
        "likes": 12,
        "downloads": 34,
        "sha": "abc123",
        "used_storage": 30,
        "source_only": {"preserved": true}
    })
}

#[test]
fn converts_hugging_face_metadata_to_external_model() {
    let source = source_model();

    let model =
        match HuggingFaceClient::new().from_platform_metadata(source.clone()) {
            Ok(model) => model,
            Err(error) => panic!("conversion should succeed: {error}"),
        };

    assert!(matches!(model.provider(), ModelProvider::HuggingFace));

    let locator = match model.locator() {
        ModelLocator::HuggingFace(locator) => locator,
        ModelLocator::Tapis(_) => panic!("expected Hugging Face locator"),
    };

    assert_eq!(locator.id(), "author/model");
    assert_eq!(locator.sha(), "abc123");
    assert_eq!(model.metadata().derived().size(), 30);
    assert_eq!(model.metadata().derived().name(), Some("model"));
    assert_eq!(model.metadata().derived().author(), Some("author"));
    assert_eq!(model.metadata().derived().license(), Some("apache-2.0"));
    assert_eq!(
        model.metadata().derived().inference_runtimes(),
        ["transformers", "onnx"]
    );
    assert_eq!(
        model.metadata().canonical().get("source_only"),
        source.get("source_only")
    );
}

#[test]
fn skips_restricted_models() {
    let mut source = source_model();
    source["gated"] = json!(true);

    let result = HuggingFaceClient::new().from_platform_metadata(source);

    assert!(matches!(
        result,
        Err(clients::ClientError::Forbidden { .. })
    ));
}

#[test]
fn skips_models_without_storage_size() {
    let mut source = source_model();
    source["used_storage"] = serde_json::Value::Null;

    let result = HuggingFaceClient::new().from_platform_metadata(source);

    assert!(matches!(
        result,
        Err(clients::ClientError::BadRequest { .. })
    ));
}
