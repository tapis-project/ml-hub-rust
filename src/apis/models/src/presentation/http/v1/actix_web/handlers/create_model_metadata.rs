use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::dto::{
    ModelMetadata,
    CreateModelMetadataPath,
    CreateModelMetadata as CreateModelMetadataDto
};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_metadata_service_factory;
use crate::application::model_metadata_inputs::CreateModelMetadata as CreateModelMetadataInput;
use actix_web::{
    post,
    web, 
    // HttpRequest, 
    Responder
};
use shared::logging::SharedLogger;
// use std::collections::HashMap;

#[post("models-api/artifacts/{artifact_id}/metadata")]
async fn create_model_metadata(
    // req: HttpRequest,
    path: web::Path<CreateModelMetadataPath>,
    // query: web::Query<HashMap<String, String>>,
    body: web::Json<ModelMetadata>,
    data: web::Data<AppState>,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start create model metadata operation");

    let artifact_id = path.into_inner().artifact_id;

    let dto = CreateModelMetadataDto {
        artifact_id: artifact_id.clone(),
        metadata: body.into_inner()
    };

    let input = match CreateModelMetadataInput::try_from(dto) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let model_metadata_service = match model_metadata_service_factory(&data.db).await {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    match model_metadata_service.create_metadata(input).await {
        Ok(_) => (),
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(None, Some(format!("Successfully created metadata for artifact {}", artifact_id)), None)
}
