use std::collections::HashMap;
use crate::config::VERSION;
use clients::registrars::ModelsClientRegistrar;
use actix_web::{
    web,
    post,
    HttpRequest as ActixHttpRequest,
    HttpResponse,
    Responder as ActixResponder
};
use shared::logging::SharedLogger;
use shared::responses::JsonResponse;
use shared::requests::{DiscoverModelsPath, DiscoverModelsRequest, DiscoveryCriteriaBody};

#[post("models-api/platforms/{platform}/models")]
async fn discover_models(
    req: ActixHttpRequest,
    path: web::Path<DiscoverModelsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<DiscoveryCriteriaBody>,
) -> impl ActixResponder {
    let logger = SharedLogger::new();

    logger.debug("Start operation discover_models");

    // Initialize the client registrar
    let registrar = ModelsClientRegistrar::new();

    // Get the client for the provided platform
    let client = if let Ok(client) = registrar.get_client(&path.platform) {
        client
    } else {
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .json(JsonResponse {
                status: Some(500),
                message: Some(String::from(format!("Failed to find client for platform '{}'", &path.platform))),
                result: None,
                metadata: None,
                version: Some(VERSION.to_string()),
            });
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
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: resp.result,
                    metadata: None,
                    version: Some(VERSION.to_string())
                })
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(err.to_string()),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string())
                })
        }
    }
}