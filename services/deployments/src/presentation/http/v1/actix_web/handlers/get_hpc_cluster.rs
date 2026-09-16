use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::hpc_cluster_query_service::{
        HpcClusterQueryService, HpcClusterQueryServiceError,
    },
    domain::entities::hpc_cluster::HpcClusterId,
    presentation::http::v1::{
        contracts::responses::GetHpcClusterResponse,
        requests::hpc_clusters::{DataCenter, GetHpcClusterPath},
        responses::hpc_clusters::HpcCluster,
    },
    shared_kernel::context::RequestContext,
};
use uuid::Uuid;

use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/deployments-api/data-centers/{data_center}/hpc-clusters/{hpc_cluster_id}",
    tag = "Targets",
    summary = "Get an HPC cluster",
    params(GetHpcClusterPath),
    responses(
        (
            status = 200,
            description = "HPC cluster found",
            body = GetHpcClusterResponse
        ),
        (
            status = 400,
            description = "Invalid data center or HPC cluster ID"
        ),
        (
            status = 404,
            description = "HPC cluster not found"
        ),
        (
            status = 500,
            description = "Unable to get HPC cluster"
        ),
    ),
)]
#[get("deployments-api/data-centers/{data_center}/hpc-clusters/{hpc_cluster_id}")]
pub async fn get_hpc_cluster(
    path: web::Path<(String, String)>,
    ctx: RequestContext,
    service: web::Data<HpcClusterQueryService>,
) -> impl Responder {
    let (data_center, hpc_cluster_id) = path.into_inner();

    let data_center = match DataCenter::try_from(data_center.as_str()) {
        Ok(data_center) => data_center.into(),
        Err(error) => return build_error_response(400, error),
    };

    let hpc_cluster_id = match Uuid::parse_str(&hpc_cluster_id) {
        Ok(id) => HpcClusterId::reconstitute(id),
        Err(error) => return build_error_response(400, error.to_string()),
    };

    let output = match service
        .get_hpc_cluster(&ctx, &data_center, &hpc_cluster_id)
        .await
    {
        Ok(output) => output,
        Err(HpcClusterQueryServiceError::NotFound) => {
            return build_error_response(404, "HPC cluster not found".into())
        }
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(HpcCluster::from(output.hpc_cluster)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(result),
        Some("Successfully retrieved HPC cluster".into()),
        None,
    )
}
