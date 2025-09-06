use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::dto::ListArtifactPublicationsPath;
use crate::presentation::http::v1::responses;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, Responder};
use shared::application::inputs::artifacts::ListPublicationsByArtifactIdInput;
use shared::logging::SharedLogger;
use serde_json::{to_value, Value};
use shared::presentation::http::v1::contracts::responses::ListModelPublicationsForArtifactResponse;
use uuid::Uuid;

#[utoipa::path(
    get,
    path="/models-api/artifacts/{artifact_id}/publications",
    tag="Publications",
    description="List all publications for an artifact",
    params(
        ("artifact_id" = String, Path, description = "The ID of the artifact")
    ),
    responses(
        (status=200, description="Listed model publications for artifact", body=ListModelPublicationsForArtifactResponse)
    )
)]
#[get("models-api/artifacts/{artifact_id}/publications")]
async fn list_publications_for_artifact(
    path: web::Path<ListArtifactPublicationsPath>,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Get publications for artifact");
    
    let artifact_service = match artifact_service_factory(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let artifact_id = match Uuid::parse_str(path.artifact_id.clone().as_str()) {
        Ok(id) => id,
        Err(err) => return build_error_response(400, err.to_string())
    };

    let input = ListPublicationsByArtifactIdInput {
        artifact_id 
    };

    let publications = match artifact_service.list_publications_by_artifact_id(input).await {
        Ok(p) => p,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let response_dtos: Vec<responses::ArtifactPublication>  = publications.into_iter()
        .map(|a| responses::ArtifactPublication::from(a))
        .collect();

    let mut result: Vec<Value> = Vec::with_capacity(response_dtos.len());
    for dto in response_dtos {
        match to_value(dto) {
            Ok(v) => result.push(v),
            Err(err) => return build_error_response(500, err.to_string())
        };
    };

    let response = match to_value(result) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };
    
    build_success_response(Some(response), None, None)
}
