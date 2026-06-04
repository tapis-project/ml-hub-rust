use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{
    ModelMetadata,
    CreateModelMetadata,
};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_metadata_service_factory;
use crate::application::model_metadata_inputs::UpsertModelMetadata as UpsertModelMetadataInput;
use actix_web::{
    post,
    web, 
    Responder
};
use shared::presentation::http::v1::contracts::responses;
use shared::presentation::http::v1::requests::errors::PresentationError;

#[utoipa::path(
    post,
    path="/models-api/models",
    tag="Models",
    description="Create a model metadata",
    request_body=ModelMetadata,
    responses(
        (status=200, description="Discovered models", body=responses::CreateModelMetadataResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[post("models-api/models")]
async fn create_model_metadata(
    body: web::Json<ModelMetadata>,
    data: web::Data<AppState>,
) -> impl Responder {
    let dto = match CreateModelMetadata::new(body.into_inner()) {
        Ok(r) => r,
        Err(err) => {
            match err {
                PresentationError::ValidationError(_) => return build_error_response(400, err.to_string())
            }
        }
    };

    let input = match UpsertModelMetadataInput::try_from(dto) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let model_metadata_service = match model_metadata_service_factory(&data.client, data.db_name.clone(), data.client_strategy_sets.clone()).await {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    match model_metadata_service.register_model_metadata(input).await {
        Ok(_) => (),
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(None, Some("Successfully created metadata".into()), None)
}
