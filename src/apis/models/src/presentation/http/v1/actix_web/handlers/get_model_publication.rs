use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::responses;
use crate::presentation::http::v1::requests::GetArtifactPublicationPath;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use shared::application::inputs::artifact_publication::GetModelPublicationInput;
use shared::application::services::artifact_service::ArtifactServiceError;
use shared::logging::SharedLogger;
use serde_json::to_value;
use shared::presentation::http::v1::contracts;
use uuid::Uuid;

#[utoipa::path(
    get,
    path="/models-api/publications/{publication_id}",
    tag="Publications",
    description="Fetch a publication by id",
    params(
        ("publication_id" = String, Path, description = "The ID of the model publication")
    ),
    responses(
        (status=200, description="Discovered models", body=contracts::responses::GetModelPublicationResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("models-api/publications/{publication_id}")]
async fn get_model_publication(
    _req: HttpRequest,
    path: web::Path<GetArtifactPublicationPath>,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("List aritfacts operation");
    
    let artifact_service = match artifact_service_factory(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let publication_id = match Uuid::parse_str(&path.publication_id) {
        Ok(id) => id,
        Err(err) => return build_error_response(400, err.to_string())
    };

    let input = GetModelPublicationInput {
        publication_id
    };

    let maybe_publication = match artifact_service.get_model_publication(input).await {
        Ok(a) => a,
        Err(err) => {
            match err {
                ArtifactServiceError::IncorrectArtifactType(_) => return build_error_response(404, format!("Model Artifact Publication not for id '{}'", &publication_id)),
                _ => return build_error_response(500, err.to_string())
            }
        }
    };

    let publication = match maybe_publication {
        Some(i) => i,
        None => return build_error_response(404, format!("ArtifactPublication with id {} not found", &publication_id))
    };

    let requests = match to_value(responses::ArtifactPublication::from(publication)) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };
    
    build_success_response(Some(requests), None, None)
}
