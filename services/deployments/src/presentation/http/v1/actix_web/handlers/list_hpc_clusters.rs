use actix_web::{get, web, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::services::hpc_cluster_query_service::HpcClusterQueryService,
    presentation::http::v1::{
        contracts::responses::ListHpcClustersResponse,
        requests::hpc_clusters::{DataCenter, ListHpcClustersPath, ListHpcClustersQuery},
        responses::hpc_clusters::HpcClusterSummary,
    },
    shared_kernel::context::RequestContext,
};

use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/deployments-api/data-centers/{data_center}/hpc-clusters",
    tag = "Targets",
    summary = "List HPC clusters in a data center",
    params(
        ListHpcClustersPath,
        ListHpcClustersQuery
    ),
    responses(
        (
            status = 200,
            description = "HPC clusters listed",
            body = ListHpcClustersResponse
        ),
        (
            status = 400,
            description = "Invalid data center or pagination parameters"
        ),
        (
            status = 500,
            description = "Unable to list HPC clusters"
        ),
    ),
)]
#[get("deployments-api/data-centers/{data_center}/hpc-clusters")]
pub async fn list_hpc_clusters(
    path: web::Path<String>,
    query: web::Query<ListHpcClustersQuery>,
    ctx: RequestContext,
    service: web::Data<HpcClusterQueryService>,
) -> impl Responder {
    let data_center = match DataCenter::try_from(path.as_str()) {
        Ok(data_center) => data_center,
        Err(error) => return build_error_response(400, error),
    };

    let input = query.into_inner().into_input(data_center);

    let output = match service.list_hpc_clusters(&ctx, &input).await {
        Ok(output) => output,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(
        output
            .hpc_clusters
            .into_iter()
            .map(HpcClusterSummary::from)
            .collect::<Vec<_>>(),
    ) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let mut metadata = Map::new();

    if let Some(cursor) = output.cursor {
        metadata.insert("cursor".into(), Value::String(cursor));
    }

    if let Some(count) = output.count {
        metadata.insert("count".into(), Value::Number(count.into()));
    }

    build_success_response(
        Some(result),
        Some("Successfully listed HPC clusters".into()),
        Some(Value::Object(metadata)),
    )
}
