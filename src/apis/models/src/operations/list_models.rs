use std::collections::HashMap;
use crate::helpers::{build_error_response, build_success_response};
use clients::registrars::ModelsClientRegistrar;
use actix_web::{
    web,
    get,
    HttpRequest as ActixHttpRequest,
    Responder as ActixResponder
};
use shared::logging::SharedLogger;
use shared::models::web::dto::{ListModelsPath, ListModelsRequest};

#[get("models-api/platforms/{platform}/models")]
async fn list_models(
    req: ActixHttpRequest,
    path: web::Path<ListModelsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl ActixResponder {
    let logger = SharedLogger::new();

    logger.debug("Start operation list_models");
    logger.debug(format!("path: {:#?}", path).as_str());

    // Initialize the client registrar
    let registrar = ModelsClientRegistrar::new();

    // Get the client for the provided platform
    let client = if let Ok(client) = registrar.get_client(&path.platform) {
        client
    } else {
        return build_error_response(
            500,
            String::from(format!("Failed to find client for platform '{}'", &path.platform))
        )
    };

    // Build the request used by the client
    let request = ListModelsRequest{
        req,
        path,
        query,
        body
    };

    // Fetch the list of models
    match client.list_models(&request) {
        Ok(resp) => {
            return build_success_response(resp.result, Some(200), Some(String::from("success")));
        },
        Err(err) => {
            return build_error_response(500, err.to_string())
        }
    }
}