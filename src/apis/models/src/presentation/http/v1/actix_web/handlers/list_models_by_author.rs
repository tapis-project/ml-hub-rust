use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{
    ModelMetadata,
    CreateModelMetadataPath,
    AssociateModelMetadata as AssociateModelMetadataDto
};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_metadata_service_factory;
use crate::application::model_metadata_inputs::AssociateModelMetadata as AssociateModelMetadataInput;
use actix_web::{
    get,
    web, 
    // HttpRequest, 
    Responder
};
use shared::logging::SharedLogger;
use shared::presentation::http::v1::contracts::responses;

#[utoipa::path(
    get,
    path="/models-api/models/{author}/{name}",
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
#[get("models-api/models/{author}/{name}")]
async fn create_model_metadata(
    // req: HttpRequest,
    path: web::Path<CreateModelMetadataPath>,
    // query: web::Query<HashMap<String, String>>,
    body: web::Json<ModelMetadata>,
    data: web::Data<AppState>,
) -> impl Responder {
    // let logger = SharedLogger::new();

    // logger.debug("Start create model metadata operation");

    // let artifact_id = path.into_inner().artifact_id;

    // let requests = AssociateModelMetadataDto {
    //     artifact_id: artifact_id.clone(),
    //     metadata: body.into_inner()
    // };

    // let input = match AssociateModelMetadataInput::try_from(requests) {
    //     Ok(i) => i,
    //     Err(err) => return build_error_response(500, err.to_string())
    // };

    // let model_metadata_service = match model_metadata_service_factory(&data.db).await {
    //     Ok(s) => s,
    //     Err(err) => return build_error_response(500, err.to_string())
    // };

    // match model_metadata_service.associate_metadata_with_artifact(input).await {
    //     Ok(_) => (),
    //     Err(err) => return build_error_response(500, err.to_string())
    // };

    // build_success_response(None, Some(format!("Successfully created metadata for artifact {}", artifact_id)), None)
    build_error_response(501, "Not Implemnted".into())
}
