use shared::application::services::model_service::ModelService;
use shared::shared_kernel::context::RequestContext;
use validator::Validate;

use crate::application::model_inputs::RegisterModelInput;
use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::requests::create_model::body::CreateModelBody;
use actix_web::{post, web, Responder};
use shared::presentation::http::v1::contracts::responses;

#[utoipa::path(
    post,
    path="/models-api/models",
    tag="Models",
    description="Create a model",
    request_body=CreateModelBody,
    responses(
        (status=200, description="Model created", body=responses::CreateModelResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[post("models-api/models")]
async fn create_model(
    body: web::Json<CreateModelBody>,
    ctx: RequestContext,
    model_service: web::Data<ModelService>,
) -> impl Responder {
    let request_body = body.into_inner();

    if let Err(err) = request_body.validate() {
        return build_error_response(500, err.to_string());
    };

    let input = match RegisterModelInput::try_from(request_body) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    match model_service.register_model(input, &ctx).await {
        Ok(_) => (),
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(None, Some("Successfully created model".into()), None)
}
