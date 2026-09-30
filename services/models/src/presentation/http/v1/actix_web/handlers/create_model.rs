use actix_web::{post, web, HttpResponse, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::services::model_creation_service::{
        ModelCreationService, ModelCreationServiceError,
    },
    presentation::http::v1::{
        contracts::responses,
        requests::create_model::body::CreateModelBody,
        responses::{models::Model, JsonResponse},
    },
    shared_kernel::context::RequestContext,
};
use validator::Validate;

use crate::{
    config::VERSION, presentation::http::v1::actix_web::response_helpers::build_error_response,
};

#[utoipa::path(
    post,
    path = "/models-api/models",
    tag = "Models",
    summary = "Add an external model to the user's collection",
    request_body = CreateModelBody,
    responses(
        (
            status = 201,
            description = "Model created",
            body = responses::CreateModelResponse
        ),
        (
            status = 400,
            description = "Invalid model",
            body = responses::BadRequestResponse
        ),
        (
            status = 404,
            description = "ExternalModel not found",
            body = responses::NotFoundResponse
        ),
        (
            status = 409,
            description = "Model already in your collection",
            body = responses::ConflictResponse
        ),
        (
            status = 500,
            description = "Unable to create model",
            body = responses::ServerErrorResponse
        ),
    ),
)]
#[post("models-api/models")]
pub async fn create_model(
    body: web::Json<CreateModelBody>,
    ctx: RequestContext,
    service: web::Data<ModelCreationService>,
) -> impl Responder {
    let body = body.into_inner();

    if let Err(error) = body.validate() {
        return build_error_response(400, error.to_string());
    }

    let output = match service.create_model(&ctx, body.into()).await {
        Ok(output) => output,
        Err(ModelCreationServiceError::ExternalModelNotFound) => {
            return build_error_response(404, "ExternalModel not found".into())
        }
        Err(ModelCreationServiceError::ModelAlreadyInCollection) => {
            return build_error_response(409, "Model already in your collection".into())
        }
        Err(ModelCreationServiceError::Domain(error)) => {
            return build_error_response(400, error.to_string())
        }
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(Model::from(output)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    HttpResponse::Created().json(JsonResponse {
        status: Some(201),
        message: Some("Successfully created model".into()),
        result: Some(result),
        metadata: Some(Value::Object(Map::new())),
        version: Some(VERSION.into()),
    })
}
