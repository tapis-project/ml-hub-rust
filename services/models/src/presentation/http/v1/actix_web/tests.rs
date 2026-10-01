use std::sync::Arc;

use actix_web::{http::StatusCode, test as actix_test, web, App, HttpMessage};
use async_trait::async_trait;
use shared::domain::entities::model::external_model::ExternalModelId;
use shared::{
    application::{
        inputs::{
            deployment_option::ListDeploymentOptionsInput,
            discover_models::SearchExternalModelsInput, hpc_cluster::ListHpcClustersInput,
        },
        outputs::hpc_cluster::HpcClusterListOutput,
        ports::{
            deployment_option::{
                DeploymentOptionPage, DeploymentOptionRepository, DeploymentOptionRepositoryError,
            },
            errors::InfrastructureError,
            hpc_cluster::{HpcClusterRepository, HpcClusterRepositoryError},
            model::{ExternalModelPage, ExternalModelRepository, ExternalModelRepositoryError},
        },
        services::deployment_option_query_service::DeploymentOptionQueryService,
    },
    domain::entities::{
        deployment_option::DeploymentOption,
        hpc_cluster::{DataCenter, HpcCluster, HpcClusterId},
        model::external_model::{
            DerivedMetadata, ExternalModel, HuggingFaceRepoLocator, ModelLocator, ModelMetadata,
            ModelProvider,
        },
    },
    shared_kernel::context::RequestContext,
};
use utoipa::OpenApi;

use super::{
    handlers::list_external_model_deployment_options::list_external_model_deployment_options,
    openapi::ApiDoc,
};

struct EmptyDeploymentOptionRepository;

#[async_trait]
impl DeploymentOptionRepository for EmptyDeploymentOptionRepository {
    async fn find_by_id(
        &self,
        _id: &shared::domain::entities::deployment_option::DeploymentOptionId,
    ) -> Result<Option<DeploymentOption>, DeploymentOptionRepositoryError> {
        Ok(None)
    }

    async fn find_by_external_model_id(
        &self,
        _external_model_id: &ExternalModelId,
    ) -> Result<Vec<DeploymentOption>, DeploymentOptionRepositoryError> {
        Ok(Vec::new())
    }

    async fn list_by_external_model_id(
        &self,
        _external_model_id: &ExternalModelId,
        input: &ListDeploymentOptionsInput,
    ) -> Result<DeploymentOptionPage, DeploymentOptionRepositoryError> {
        if input.cursor() == Some("invalid") {
            return Err(DeploymentOptionRepositoryError::InvalidCursor);
        }

        if input.cursor() == Some("error") {
            return Err(DeploymentOptionRepositoryError::Persistence(
                InfrastructureError::Transient {
                    error_id: uuid::Uuid::now_v7(),
                    reason: "repository unavailable".into(),
                    retry_after: None,
                },
            ));
        }

        Ok(DeploymentOptionPage {
            deployment_options: Vec::new(),
            count: input.include_count().then_some(0),
            cursor: None,
        })
    }

    async fn replace_for_external_model(
        &self,
        _external_model_id: &ExternalModelId,
        _deployment_options: &[DeploymentOption],
    ) -> Result<(), DeploymentOptionRepositoryError> {
        Ok(())
    }
}

struct TestExternalModelRepository {
    external_model: Option<ExternalModel>,
}

#[async_trait]
impl ExternalModelRepository for TestExternalModelRepository {
    async fn save(&self, _model: &ExternalModel) -> Result<(), ExternalModelRepositoryError> {
        Ok(())
    }

    async fn update(&self, _model: &ExternalModel) -> Result<(), ExternalModelRepositoryError> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        _id: &ExternalModelId,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(self.external_model.clone())
    }

    async fn find_by_ids(
        &self,
        _ids: &[ExternalModelId],
    ) -> Result<Vec<ExternalModel>, ExternalModelRepositoryError> {
        Ok(Vec::new())
    }

    async fn find_by_provider_and_locator(
        &self,
        _provider: &ModelProvider,
        _locator: &ModelLocator,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(None)
    }

    async fn search(
        &self,
        _input: &SearchExternalModelsInput,
    ) -> Result<ExternalModelPage, ExternalModelRepositoryError> {
        Ok(ExternalModelPage {
            external_models: Vec::new(),
            count: None,
            cursor: None,
        })
    }
}

struct EmptyHpcClusterRepository;

#[async_trait]
impl HpcClusterRepository for EmptyHpcClusterRepository {
    async fn list_all(&self) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError> {
        Ok(Vec::new())
    }

    async fn find_by_id(
        &self,
        _id: &HpcClusterId,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(None)
    }

    async fn find_by_id_and_data_center(
        &self,
        _id: &HpcClusterId,
        _data_center: &DataCenter,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(None)
    }

    async fn find_by_ids(
        &self,
        _ids: &[HpcClusterId],
    ) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError> {
        Ok(Vec::new())
    }

    async fn list(
        &self,
        _input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError> {
        Ok(HpcClusterListOutput {
            hpc_clusters: Vec::new(),
            cursor: None,
            count: None,
        })
    }
}

fn deployment_option_query_service(
    external_model: Option<ExternalModel>,
) -> web::Data<DeploymentOptionQueryService> {
    web::Data::new(DeploymentOptionQueryService::new(
        Arc::new(EmptyDeploymentOptionRepository),
        Arc::new(TestExternalModelRepository { external_model }),
        Arc::new(EmptyHpcClusterRepository),
    ))
}

fn external_model() -> Result<ExternalModel, Box<dyn std::error::Error>> {
    let derived = DerivedMetadata::new(
        Some("model".into()),
        Some("author".into()),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
        1,
        false,
        false,
        None,
        None,
    )?;

    let locator = HuggingFaceRepoLocator::new("author/model".into(), "sha".into())?;

    Ok(ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::HuggingFace(locator),
        ModelMetadata::new(derived, serde_json::Map::new()),
    )?)
}

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

#[test]
fn openapi_exposes_deployment_option_search_filters() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    for property in [
        "serving_runtimes",
        "hpc_cluster_ids",
        "batch_scheduler_queue_ids",
        "supported_deployment_modalities",
        "has_deployment_options",
    ] {
        assert!(
            document
                .pointer(&format!(
                    "/components/schemas/DiscoveryCriterion/properties/{property}"
                ))
                .is_some(),
            "missing deployment option filter {property}"
        );
    }

    assert_eq!(
        document.pointer("/components/schemas/ServingRuntime/enum"),
        Some(&serde_json::json!(["FlexServ"]))
    );

    Ok(())
}

#[test]
fn openapi_exposes_external_model_deployment_options() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    let operation = document
        .pointer(
            "/paths/~1models-api~1external-models~1{external_model_id}~1deployment-options/get",
        )
        .ok_or_else(|| {
            std::io::Error::other("deployment options operation should be documented")
        })?;

    assert_eq!(
        operation.pointer("/operationId"),
        Some(&serde_json::json!("list_external_model_deployment_options"))
    );

    for parameter in ["external_model_id", "limit", "cursor", "include_count"] {
        assert!(
            operation
                .pointer("/parameters")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|parameters| parameters.iter().any(|value| {
                    value.pointer("/name") == Some(&serde_json::json!(parameter))
                })),
            "missing deployment option parameter {parameter}"
        );
    }

    assert_eq!(
        document.pointer("/components/schemas/DeploymentTargetType/enum"),
        Some(&serde_json::json!(["HpcClusterQueue"]))
    );
    assert!(document
        .pointer("/components/schemas/DeploymentOption/properties/hpc_cluster_queue")
        .is_some());
    assert!(document
        .pointer("/components/schemas/DeploymentOption/properties/available")
        .is_some());

    Ok(())
}

#[test]
fn openapi_exposes_deployment_option_details_and_parameters(
) -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    let operation = document
        .pointer(
            "/paths/~1models-api~1external-models~1{external_model_id}~1deployment-options~1{deployment_option_id}/get",
        )
        .ok_or_else(|| {
            std::io::Error::other("deployment option detail operation should be documented")
        })?;

    assert_eq!(
        operation.pointer("/operationId"),
        Some(&serde_json::json!("get_external_model_deployment_option"))
    );
    assert!(document
        .pointer("/components/schemas/DeploymentOptionDetail/allOf/1/properties/parameters")
        .is_some());
    assert!(document
        .pointer("/components/schemas/DeploymentParameter/properties/default")
        .is_some());
    assert!(document
        .pointer("/components/schemas/DeploymentParameterChoice/properties/enabled")
        .is_some());

    Ok(())
}

#[actix_web::test]
async fn deployment_options_reject_a_malformed_external_model_id(
) -> Result<(), Box<dyn std::error::Error>> {
    let external_model = external_model()?;

    let app = actix_test::init_service(
        App::new()
            .app_data(deployment_option_query_service(Some(external_model)))
            .service(list_external_model_deployment_options),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri("/models-api/external-models/not-a-uuid/deployment-options")
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[actix_web::test]
async fn deployment_options_return_not_found_for_a_missing_external_model() {
    let app = actix_test::init_service(
        App::new()
            .app_data(deployment_option_query_service(None))
            .service(list_external_model_deployment_options),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri(&format!(
            "/models-api/external-models/{}/deployment-options",
            ExternalModelId::new()
        ))
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn deployment_options_reject_an_invalid_cursor() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = external_model()?;

    let app = actix_test::init_service(
        App::new()
            .app_data(deployment_option_query_service(Some(external_model)))
            .service(list_external_model_deployment_options),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri(&format!(
            "/models-api/external-models/{}/deployment-options?cursor=invalid",
            ExternalModelId::new()
        ))
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[actix_web::test]
async fn deployment_options_return_an_empty_page_with_count_metadata(
) -> Result<(), Box<dyn std::error::Error>> {
    let external_model = external_model()?;

    let app = actix_test::init_service(
        App::new()
            .app_data(deployment_option_query_service(Some(external_model)))
            .service(list_external_model_deployment_options),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri(&format!(
            "/models-api/external-models/{}/deployment-options?include_count=true",
            ExternalModelId::new()
        ))
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = actix_test::read_body_json(response).await;

    assert_eq!(body.pointer("/result"), Some(&serde_json::json!([])));
    assert_eq!(body.pointer("/metadata/count"), Some(&serde_json::json!(0)));

    Ok(())
}

#[actix_web::test]
async fn deployment_options_return_server_error_for_repository_failure(
) -> Result<(), Box<dyn std::error::Error>> {
    let external_model = external_model()?;

    let app = actix_test::init_service(
        App::new()
            .app_data(deployment_option_query_service(Some(external_model)))
            .service(list_external_model_deployment_options),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri(&format!(
            "/models-api/external-models/{}/deployment-options?cursor=error",
            ExternalModelId::new()
        ))
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    Ok(())
}
