use super::openapi::ApiDoc;
use utoipa::OpenApi;

#[test]
fn openapi_exposes_model_contracts_and_association_route() -> Result<(), Box<dyn std::error::Error>>
{
    let document = serde_json::to_value(ApiDoc::openapi())?;

    for path in [
        "/paths/~1models-api~1artifacts~1{artifact_id}~1model/post",
        "/paths/~1models-api~1external-models~1search/post",
        "/paths/~1models-api~1external-models~1{external_model_id}/get",
        "/paths/~1models-api~1models/post",
        "/paths/~1models-api~1models/get",
        "/paths/~1models-api~1models~1{model_id}/get",
    ] {
        assert!(document.pointer(path).is_some(), "missing path {path}");
    }

    for path in [
        "/paths/~1models-api~1models~1fork~1{author}~1{name}",
        "/paths/~1models-api~1models~1search",
        "/paths/~1models-api~1models~1{author}",
        "/paths/~1models-api~1models~1{author}~1{name}",
        "/paths/~1models-api~1platforms~1{platform}~1models",
        "/paths/~1models-api~1platforms~1{platform}~1models~1{model_id}",
    ] {
        assert!(
            document.pointer(path).is_none(),
            "legacy path remains: {path}"
        );
    }

    for schema in [
        "Model",
        "CreateModelBody",
        "CreateModelResponse",
        "AssociateModelBody",
        "ExternalModel",
        "DerivedModelMetadata",
    ] {
        assert!(document
            .pointer(&format!("/components/schemas/{schema}"))
            .is_some());
    }

    for legacy_schema in [
        "ModelMetadata",
        "CreateModelMetadataBody",
        "CreateModelMetadataResponse",
        "AssociateModelMetadataBody",
        "ForkModelResponse",
    ] {
        assert!(document
            .pointer(&format!("/components/schemas/{legacy_schema}"))
            .is_none());
    }

    assert_eq!(
        document.pointer("/paths/~1models-api~1models/post/operationId"),
        Some(&serde_json::json!("create_model"))
    );
    assert_eq!(
        document.pointer("/paths/~1models-api~1models/post/responses/201/description"),
        Some(&serde_json::json!("Model created"))
    );
    assert!(document
        .pointer("/components/schemas/ExternalModel/properties/metadata/properties/canonical")
        .is_none());
    assert_eq!(
        document.pointer("/paths/~1models-api~1artifacts~1{artifact_id}~1model/post/operationId"),
        Some(&serde_json::json!("associate_model_with_artifact"))
    );

    Ok(())
}

#[test]
fn openapi_uses_model_publication_status_names() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    let statuses = document
        .pointer("/components/schemas/ArtifactPublicationStatus/enum")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| std::io::Error::other("publication statuses should be documented"))?;

    assert!(statuses.contains(&serde_json::json!("PublishingModel")));
    assert!(statuses.contains(&serde_json::json!("PublishedModel")));
    assert!(!statuses.contains(&serde_json::json!("PublishingMetadata")));
    assert!(!statuses.contains(&serde_json::json!("PublishedMetadata")));

    Ok(())
}
