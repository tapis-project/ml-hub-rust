use actix_web::{get, Responder};
use crate::presentation::http::v1::actix_web::response_helpers::build_success_response;

#[get("/models-api/health-check")]
pub async fn health_check() -> impl Responder {
    return build_success_response(None, Some(String::from("success")), None);
}


// Handler tests
#[cfg(test)]
#[path = "health_check.test.rs"]
mod health_check_test;