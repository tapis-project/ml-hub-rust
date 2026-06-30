use shared::shared_kernal::identity::IdentityContext;
use shared::application::services::model_metadata_service::ModelMetadataService;
use validator::Validate;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::create_model_metadata::body::CreateModelMetadataBody;
use crate::application::model_metadata_inputs::UpsertModelMetadata as UpsertModelMetadataInput;
use actix_web::{
    post,
    web, 
    Responder
};
use shared::presentation::http::v1::contracts::responses;

#[utoipa::path(
    post,
    path="/models-api/models",
    tag="Models",
    description="Create a model metadata",
    request_body=CreateModelMetadataBody,
    responses(
        (status=200, description="Discovered models", body=responses::CreateModelMetadataResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[post("models-api/models")]
async fn create_model_metadata(
    body: web::Json<CreateModelMetadataBody>,
    ctx: IdentityContext,
    model_metadata_service: web::Data<ModelMetadataService>,
) -> impl Responder {
    let request_body = body.into_inner();

    if let Err(err) = request_body.validate() {
        return build_error_response(500, err.to_string())
    };

    let input = match UpsertModelMetadataInput::try_from((request_body, &ctx)) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string())
    };

    match model_metadata_service.register_model_metadata(input, &ctx).await {
        Ok(_) => (),
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(None, Some("Successfully created metadata".into()), None)
}
