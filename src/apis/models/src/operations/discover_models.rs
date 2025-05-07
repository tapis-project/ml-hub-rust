use std::collections::HashMap;
use crate::helpers::{build_error_response, build_success_response};
use clients::registrars::ModelsClientRegistrar;
use actix_web::{
    web,
    post,
    HttpRequest as ActixHttpRequest,
    Responder
};
use shared::logging::SharedLogger;
use shared::models::web::v1::dto::{DiscoverModelsPath, DiscoverModelsRequest, DiscoveryCriteriaBody};

#[post("models-api/platforms/{platform}/models")]
async fn discover_models(
    req: ActixHttpRequest,
    path: web::Path<DiscoverModelsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<DiscoveryCriteriaBody>,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start operation discover_models");

    // Initialize the client registrar
    let registrar = ModelsClientRegistrar::new();

    // Get the client for the provided platform
    let client = if let Ok(client) = registrar.get_client(&path.platform) {
        client
    } else {
        return build_error_response(
            500,
            String::from(format!("Failed to find client for platform '{}'", &path.platform)),
        )
    };

    // Build the request used by the client
    let request = DiscoverModelsRequest{
        req,
        path,
        query,
        body
    };

    // Fetch the list of models
    match client.discover_models(&request) {
        Ok(resp) => {
            return build_success_response(resp.result, Some(String::from("success")), None)
        },
        Err(err) => {
            return build_error_response(500, err.to_string())
        }
    }
}