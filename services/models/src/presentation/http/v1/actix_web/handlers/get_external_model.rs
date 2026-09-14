use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::external_model_discovery_service::{
        ExternalModelDiscoveryService, ExternalModelDiscoveryServiceError,
    },
    presentation::http::v1::{
        contracts::responses::GetExternalModelResponse, requests::models::GetExternalModelPath,
        responses::models::ExternalModel,
    },
    shared_kernel::identifiers::ExternalModelId,
};

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/models-api/external-models/{external_model_id}",
    tag = "ExternalModels",
    summary = "Get an external model",
    params(GetExternalModelPath),
    responses(
        (
            status = 200,
            description = "ExternalModel found",
            body = GetExternalModelResponse
        ),
        (
            status = 404,
            description = "ExternalModel not found"
        ),
        (
            status = 500,
            description = "Unable to get ExternalModel"
        ),
    ),
)]
#[get("models-api/external-models/{external_model_id}")]
pub async fn get_external_model(
    path: web::Path<GetExternalModelPath>,
    service: web::Data<ExternalModelDiscoveryService>,
) -> impl Responder {
    let id = ExternalModelId::reconstitute(path.external_model_id);

    let model = match service.get_external_model(&id).await {
        Ok(model) => model,
        Err(ExternalModelDiscoveryServiceError::NotFound) => {
            return build_error_response(404, "ExternalModel not found".into())
        }
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(ExternalModel::from(model)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(result),
        Some("Successfully retrieved ExternalModel".into()),
        None,
    )
}
