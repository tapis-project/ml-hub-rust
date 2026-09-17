use crate::presentation::http::v1::actix_web::helpers::build_success_response;
use actix_web::{get, Responder};

#[get("/deployments-api")]
pub async fn index() -> impl Responder {
    build_success_response(None, Some(String::from("success")), None)
}
