use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::responses;
use crate::presentation::http::v1::dto::GetArtifactIngestionPath;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use shared::application::inputs::artifact_ingestion::GetModelIngestionInput;
use shared::application::services::artifact_service::ArtifactServiceError;
use shared::logging::SharedLogger;
use serde_json::to_value;
use uuid::Uuid;

#[get("models-api/ingestions/{ingestion_id}")]
async fn get_model_ingestion(
    _req: HttpRequest,
    path: web::Path<GetArtifactIngestionPath>,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("List aritfacts operation");
    
    let artifact_service = match artifact_service_factory(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let ingestion_id = match Uuid::parse_str(&path.ingestion_id) {
        Ok(id) => id,
        Err(err) => return build_error_response(400, err.to_string())
    };

    let input = GetModelIngestionInput {
        ingestion_id
    };

    let maybe_ingestion = match artifact_service.get_model_ingestion(input).await {
        Ok(a) => a,
        Err(err) => {
            match err {
                ArtifactServiceError::IncorrectArtifactType(_) => return build_error_response(404, format!("Model Artifact Ingestion not for id '{}'", &ingestion_id)),
                _ => return build_error_response(500, err.to_string())
            }
        }
    };

    let ingestion = match maybe_ingestion {
        Some(i) => i,
        None => return build_error_response(404, format!("ArtifactIngestion with id {} not found", &ingestion_id))
    };

    let dto = match to_value(responses::ArtifactIngestion::from(ingestion)) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };
    
    build_success_response(Some(dto), None, None)
}
