use actix_web::{get, Responder};
use shared::logging::SharedLogger;
use crate::helpers::build_success_response;

#[get("/models-api/health-check")]
pub async fn health_check() -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Health check operation");
    return build_success_response(None, Some(String::from("success")), None);
}