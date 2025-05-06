use actix_web::{get, Responder};
use shared::logging::SharedLogger;
use crate::helpers::build_success_response;

#[get("/models-api")]
pub async fn index() -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Index operation");
    build_success_response(None, Some(200), Some(String::from("success")))
}