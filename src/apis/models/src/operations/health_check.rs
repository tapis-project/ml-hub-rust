use crate::config::VERSION;
use actix_web::{
    get,
    HttpResponse,
    Responder as ActixResponder
};
use shared::responses::JsonResponse;
use shared::logging::SharedLogger;

#[get("/models-api/health-check")]
pub async fn health_check() -> impl ActixResponder {
    let logger = SharedLogger::new();

    logger.debug("Health check operation");

    HttpResponse::Ok()
        .content_type("application/json")
        .json(JsonResponse {
            status: Some(200),
            message: Some(String::from("success")),
            result: None,
            metadata: None,
            version: Some(VERSION.to_string())
        })
}