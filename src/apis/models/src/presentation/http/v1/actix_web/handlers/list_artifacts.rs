use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::responses;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use shared::logging::SharedLogger;
use serde_json::{to_value, Value};

#[get("models-api/artifacts")]
async fn list_artifacts(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("List aritfacts operation");
    let artifact_service = match artifact_service_factory(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let artifacts = match artifact_service.get_all_dataset_artifacts().await {
        Ok(a) => a,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let response_dtos: Vec<responses::Artifact>  = artifacts.into_iter()
        .map(|a| responses::Artifact::from(a))
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
