use crate::presentation::http::v1::actix_web::helpers::build_success_response;
use client_provider::Platform;
use actix_web::{
    get, Responder
};
use serde_json::Value;

#[get("models-api/platforms")]
async fn list_platforms() -> impl Responder {
    let mut platforms = Vec::new();
    for platform in Platform::list_all() {
        platforms.push(Value::String(platform.to_string()))
    }
    build_success_response(Some(Value::Array(platforms)), None, None)
}