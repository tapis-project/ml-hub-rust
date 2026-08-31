use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    contracts::responses::GetDatasetResponse,
    requests::GetDatasetPath,
    responses::Dataset,
};
use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::dataset_service::{DatasetService, DatasetServiceError},
    shared_kernel::context::RequestContext,
};

#[utoipa::path(get, path = "/datasets-api/datasets/{dataset_id}", tag = "Datasets", summary = "Get a dataset", params(GetDatasetPath), responses((status = 200, description = "Dataset found", body = GetDatasetResponse), (status = 404, description = "Dataset not found"), (status = 500, description = "Unable to get dataset")))]
#[get("datasets-api/datasets/{dataset_id}")]
pub async fn get_dataset(
    path: web::Path<GetDatasetPath>,
    ctx: RequestContext,
    service: web::Data<DatasetService>,
) -> impl Responder {
    let dataset = match service.get_dataset(&ctx, path.dataset_id).await {
        Ok(v) => v,
        Err(DatasetServiceError::NotFound) => {
            return build_error_response(404, "Dataset not found".into())
        }
        Err(e) => return build_error_response(500, e.to_string()),
    };

    let response = match to_value(Dataset::from(dataset)) {
        Ok(v) => v,
        Err(e) => return build_error_response(500, e.to_string()),
    };

    build_success_response(
        Some(response),
        Some("Successfully retrieved dataset".into()),
        None,
    )
}
