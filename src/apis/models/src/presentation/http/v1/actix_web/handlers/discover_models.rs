use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{DiscoveryCriteria, Headers};
use actix_web::{post, web, HttpRequest, Responder};
use shared::logging::SharedLogger;
use std::collections::HashMap;
use crate::presentation::http::v1::contracts;

#[utoipa::path(
    post,
    path = "/models-api/models",
    tag="Models",
    description="Discover models known to MLHub",
    request_body=DiscoveryCriteria,
    responses(
        (status=200, description="Discovered models", body=contracts::responses::DiscoverModelsResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[post("models-api/models")]
async fn discover_models(
    req: HttpRequest,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<DiscoveryCriteria>,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start operation discover_models");

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    build_success_response(None, Some("testing".into()), None)
}