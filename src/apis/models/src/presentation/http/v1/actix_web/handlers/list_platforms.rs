use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use client_provider::ClientProvider;
use actix_web::{
    get, Responder
};
use serde_json::to_value;
use shared::presentation::http::v1::contracts;
use shared::presentation::http::v1::responses::PlatformDetails;

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
    let platform_capabilities = ClientProvider::get_platform_client_capabilities();
    let mut response: Vec<PlatformDetails> = Vec::new();
    for (platform, capabilities) in platform_capabilities {
        let mut capabilities_resp: Vec<String> = Vec::new();
        for capability in capabilities {
            capabilities_resp.push(capability.to_string());
        }
        response.push(PlatformDetails {
            name: platform,
            capabilities: capabilities_resp
        });
    }

    match to_value(response) {
        Ok(v) => build_success_response(Some(v), Some("Success".into()), None),
        Err(err) => build_error_response(500, err.to_string())
    }
}