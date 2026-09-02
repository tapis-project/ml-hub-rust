use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    contracts::responses::ListDatasetsResponse,
    requests::{ListDatasetsQueryParams, Scope},
    responses::Dataset,
};
use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::dataset_service::DatasetService, shared_kernel::context::RequestContext,
};

#[utoipa::path(get, path = "/datasets-api/datasets", tag = "Datasets", summary = "List datasets with at most the first 50 items from each", params(ListDatasetsQueryParams), responses((status = 200, description = "Datasets listed", body = ListDatasetsResponse), (status = 500, description = "Unable to list datasets")))]
#[get("datasets-api/datasets")]
pub async fn list_datasets(
    query: web::Query<ListDatasetsQueryParams>,
    ctx: RequestContext,
    service: web::Data<DatasetService>,
) -> impl Responder {
    let datasets = match query.scope {
        Scope::Owned => service.list_for_user(&ctx).await,
        Scope::Shared => service.list_shared_with_user(&ctx).await,
        Scope::Global => service.list_global(&ctx).await,
    };

    let datasets = match datasets {
        Ok(v) => v,
        Err(e) => return build_error_response(500, e.to_string()),
    };

    let response = match to_value(datasets.into_iter().map(Dataset::from).collect::<Vec<_>>()) {
        Ok(v) => v,
        Err(e) => return build_error_response(500, e.to_string()),
    };

    build_success_response(
        Some(response),
        Some("Successfully listed datasets".into()),
        None,
    )
}
