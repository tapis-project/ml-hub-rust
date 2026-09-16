use std::sync::Arc;

use actix_web::{http::StatusCode, test as actix_test, web, App, HttpMessage};
use async_trait::async_trait;
use shared::{
    application::{
        inputs::hpc_cluster::ListHpcClustersInput,
        outputs::hpc_cluster::HpcClusterListOutput,
        ports::hpc_cluster::{HpcClusterRepository, HpcClusterRepositoryError},
        services::hpc_cluster_query_service::HpcClusterQueryService,
    },
    domain::entities::hpc_cluster::{DataCenter, HpcCluster, HpcClusterId},
    shared_kernel::context::RequestContext,
};
use utoipa::OpenApi;

use super::{
    handlers::{get_hpc_cluster::get_hpc_cluster, list_hpc_clusters::list_hpc_clusters},
    openapi::ApiDoc,
};

struct EmptyHpcClusterRepository;

#[async_trait]
impl HpcClusterRepository for EmptyHpcClusterRepository {
    async fn find_by_id(
        &self,
        _data_center: &DataCenter,
        _id: &HpcClusterId,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(None)
    }

    async fn list(
        &self,
        _input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError> {
        Ok(HpcClusterListOutput {
            hpc_clusters: Vec::new(),
            cursor: None,
            count: Some(0),
        })
    }
}

fn hpc_cluster_query_service() -> web::Data<HpcClusterQueryService> {
    web::Data::new(HpcClusterQueryService::new(Arc::new(
        EmptyHpcClusterRepository,
    )))
}

#[test]
fn openapi_contains_hpc_cluster_routes_and_schemas() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    assert!(document
        .pointer("/paths/~1deployments-api~1data-centers~1{data_center}~1hpc-clusters/get")
        .is_some());
    assert!(document
        .pointer("/paths/~1deployments-api~1data-centers~1{data_center}~1hpc-clusters~1{hpc_cluster_id}/get")
        .is_some());
    assert!(document.pointer("/components/schemas/HpcCluster").is_some());
    assert!(document
        .pointer("/components/schemas/HpcClusterSummary")
        .is_some());

    Ok(())
}

#[test]
fn openapi_inlines_data_center_path_enum() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;
    let parameters = document
        .pointer(
            "/paths/~1deployments-api~1data-centers~1{data_center}~1hpc-clusters/get/parameters",
        )
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| std::io::Error::other("List operation should define parameters"))?;
    let data_center = parameters
        .iter()
        .find(|parameter| parameter.get("name") == Some(&serde_json::json!("data_center")))
        .ok_or_else(|| std::io::Error::other("Data center parameter should be documented"))?;

    assert_eq!(
        data_center.pointer("/schema/enum"),
        Some(&serde_json::json!(["Tacc"]))
    );

    Ok(())
}

#[test]
fn openapi_documents_hpc_cluster_pagination_limits() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;
    let parameters = document
        .pointer(
            "/paths/~1deployments-api~1data-centers~1{data_center}~1hpc-clusters/get/parameters",
        )
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| std::io::Error::other("List operation should define parameters"))?;
    let limit = parameters
        .iter()
        .find(|parameter| parameter.get("name") == Some(&serde_json::json!("limit")))
        .ok_or_else(|| std::io::Error::other("Limit parameter should be documented"))?;

    assert_eq!(
        limit.pointer("/schema/minimum"),
        Some(&serde_json::json!(1))
    );
    assert_eq!(
        limit.pointer("/schema/maximum"),
        Some(&serde_json::json!(100))
    );

    Ok(())
}

#[test]
fn openapi_uses_explicit_accelerator_fields() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    assert!(document
        .pointer("/components/schemas/HardwareProfile/properties/accelerator_type")
        .is_some());
    assert!(document
        .pointer("/components/schemas/HardwareProfile/properties/gpu")
        .is_some());
    assert!(document
        .pointer("/components/schemas/HardwareProfile/properties/accelerator")
        .is_none());
    assert!(document
        .pointer("/components/schemas/AcceleratorProfile")
        .is_none());
    assert_eq!(
        document.pointer("/components/schemas/AcceleratorType/enum"),
        Some(&serde_json::json!(["Gpu"]))
    );
    assert_eq!(
        document.pointer("/components/schemas/GpuProfile/properties/count_per_node/type"),
        Some(&serde_json::json!("integer"))
    );

    Ok(())
}

#[test]
fn openapi_uses_hpc_cluster_operation_ids() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    assert_eq!(
        document.pointer(
            "/paths/~1deployments-api~1data-centers~1{data_center}~1hpc-clusters/get/operationId"
        ),
        Some(&serde_json::json!("list_hpc_clusters"))
    );
    assert_eq!(
        document.pointer(
            "/paths/~1deployments-api~1data-centers~1{data_center}~1hpc-clusters~1{hpc_cluster_id}/get/operationId"
        ),
        Some(&serde_json::json!("get_hpc_cluster"))
    );

    Ok(())
}

#[actix_web::test]
async fn list_rejects_an_unsupported_data_center() {
    let app = actix_test::init_service(
        App::new()
            .app_data(hpc_cluster_query_service())
            .service(list_hpc_clusters),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri("/deployments-api/data-centers/tacc/hpc-clusters")
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn detail_rejects_a_malformed_cluster_id() {
    let app = actix_test::init_service(
        App::new()
            .app_data(hpc_cluster_query_service())
            .service(get_hpc_cluster),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri("/deployments-api/data-centers/Tacc/hpc-clusters/not-a-uuid")
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn detail_returns_not_found_within_the_data_center() {
    let app = actix_test::init_service(
        App::new()
            .app_data(hpc_cluster_query_service())
            .service(get_hpc_cluster),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri(&format!(
            "/deployments-api/data-centers/Tacc/hpc-clusters/{}",
            HpcClusterId::new()
        ))
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn list_returns_scoped_count_metadata() {
    let app = actix_test::init_service(
        App::new()
            .app_data(hpc_cluster_query_service())
            .service(list_hpc_clusters),
    )
    .await;

    let request = actix_test::TestRequest::get()
        .uri("/deployments-api/data-centers/Tacc/hpc-clusters?include_count=true")
        .to_request();

    request
        .extensions_mut()
        .insert(RequestContext::system(None));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = actix_test::read_body_json(response).await;

    assert_eq!(body.pointer("/metadata/count"), Some(&serde_json::json!(0)));
}
