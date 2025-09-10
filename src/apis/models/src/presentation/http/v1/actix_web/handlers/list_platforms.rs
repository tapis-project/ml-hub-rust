use crate::presentation::http::v1::actix_web::helpers::build_success_response;
use client_provider::Platform;
use actix_web::{
    get, Responder
};
use serde_json::Value;
use shared::presentation::http::v1::contracts;

#[utoipa::path(
    get,
    path="/models-api/platforms",
    tag="Platforms",
    description="List all external platforms integrated with this deployment of MLHub",
    responses(
        (status=200, description="Listed platforms", body=contracts::responses::ListPlatformsResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("models-api/platforms")]
async fn list_platforms() -> impl Responder {
    let mut platforms = Vec::new();
    for platform in Platform::list_all() {
        platforms.push(Value::String(platform.to_string()))
    }
    build_success_response(Some(Value::Array(platforms)), None, None)
}