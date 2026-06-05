use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{
    ModelMetadata,
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
    path="/models-api/models/{author}",
    tag="Models",
    description="List models by author",
    params(
        ("author" = String, Path, description = "The author of the models")
    ),
    responses(
        (status=200, description="Listed models by author", body=responses::ListModelsResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[get("models-api/models/{author}/{name}")]
async fn list_models_by_author(
    // query: web::Query<HashMap<String, String>>,
    body: web::Json<ModelMetadata>,
    data: web::Data<AppState>,
) -> impl Responder {
   build_error_response(501, "Not Implemnted".into())
}
