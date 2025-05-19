use actix_web::{get, Responder};
use shared::logging::SharedLogger;
use crate::presentation::http::v1::actix_web::helpers::build_success_response;

#[get("/models-api")]
pub async fn index() -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Index operation");
    build_success_response(None, Some(String::from("success")), None)
}