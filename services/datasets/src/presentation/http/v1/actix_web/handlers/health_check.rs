use crate::presentation::http::v1::actix_web::helpers::{
    build_success_response,
};
use actix_web::{
    get,
    Responder
};
use shared::logging::SharedLogger;
use shared::presentation::http::v1::contracts::responses;

#[utoipa::path(
    get,
    path="/datasets-api/health-check",
    tag="Health check",
    description="Health check",
    responses(
        (status=200, description="Successfully check health", body=responses::AssociateDatasetMetadataResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[get("/datasets-api/health-check")]
async fn health_check() -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Health check operation");
    return build_success_response(None, Some(String::from("success")), None);
}


// Handler tests
#[cfg(test)]
#[path = "health_check.test.rs"]
mod health_check_test;