use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::responses;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, Responder};
use shared::application::inputs::artifact_publication::ListModelPublicationsInput;
use shared::logging::SharedLogger;
use serde_json::{to_value, Value};

#[get("models-api/publications")]
async fn list_model_publications(
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("List publications operation");
    
    let artifact_service = match artifact_service_factory(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let publications = match artifact_service.list_model_publications(ListModelPublicationsInput {}).await {
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
