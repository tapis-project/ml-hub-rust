use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::responses;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use shared::logging::SharedLogger;
use serde_json::{to_value, Value};
use shared::presentation::http::v1::contracts;

#[utoipa::path(
    get,
    tag="Artifacts",
    path = "/models-api/artifacts",
    description="List all model artifacts",
    responses(
        (status=200, description="Listed model artifacts", body=contracts::responses::ListModelArtifactResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("models-api/artifacts")]
async fn list_model_artifacts(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("List aritfacts operation");
    
    let artifact_service = artifact_service_factory(&data.client, data.db_name.clone(), data.channel.clone());

    let artifacts = match artifact_service.list_model_artifacts().await {
        Ok(a) => a,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let response_dtos: Vec<responses::Artifact>  = artifacts.into_iter()
        .map(|a| responses::Artifact::from(a))
        .collect();

    let mut result: Vec<Value> = Vec::with_capacity(response_dtos.len());
    for requests in response_dtos {
        match to_value(requests) {
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
