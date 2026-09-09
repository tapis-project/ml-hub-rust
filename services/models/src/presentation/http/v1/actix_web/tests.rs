use super::openapi::ApiDoc;
use utoipa::OpenApi;

#[test]
fn openapi_exposes_model_contracts_and_association_route() -> Result<(), Box<dyn std::error::Error>>
{
    let document = serde_json::to_value(ApiDoc::openapi())?;

    assert!(document
        .pointer("/paths/~1models-api~1artifacts~1{artifact_id}~1model/post")
        .is_some());
    assert!(document
        .pointer("/paths/~1models-api~1artifacts~1{artifact_id}~1metadata")
        .is_none());

    for schema in [
        "Model",
        "CreateModelBody",
        "CreateModelResponse",
        "AssociateModelBody",
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
