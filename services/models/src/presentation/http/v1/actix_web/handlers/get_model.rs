use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::model_query_service::{ModelQueryService, ModelQueryServiceError},
    presentation::http::v1::{
        contracts::responses::GetModelResponse, requests::models::GetModelPath,
        responses::models::Model,
    },
    shared_kernel::context::RequestContext,
};

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/models-api/models/{model_id}",
    tag = "Models",
    summary = "Get a model from a tenant collection",
    params(GetModelPath),
    responses(
        (
            status = 200,
            description = "Model found",
            body = GetModelResponse
        ),
        (
            status = 404,
            description = "Model not found"
        ),
        (
            status = 500,
            description = "Unable to get model"
        ),
    ),
)]
#[get("models-api/models/{model_id}")]
pub async fn get_model(
    path: web::Path<GetModelPath>,
    ctx: RequestContext,
    service: web::Data<ModelQueryService>,
) -> impl Responder {
    let output = match service.get_model(&ctx, path.model_id).await {
        Ok(output) => output,
        Err(ModelQueryServiceError::NotFound) => {
            return build_error_response(404, "Model not found".into())
        }
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(Model::from(output)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(result),
        Some("Successfully retrieved model".into()),
        None,
    )
}
