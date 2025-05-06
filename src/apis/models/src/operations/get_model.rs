use std::collections::HashMap;
use crate::config::VERSION;
use clients::registrars::ModelsClientRegistrar;
use shared::logging::SharedLogger;
use shared::models::web::dto::{GetModelPath, GetModelRequest};
use shared::responses::JsonResponse;
use actix_web::{
    web,
    get,
    HttpRequest as ActixHttpRequest,
    HttpResponse,
    Responder as ActixResponder
};

#[get("models-api/platforms/{platform}/models/{model_id:.*}")]
async fn get_model(
    req: ActixHttpRequest,
    path: web::Path<GetModelPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl ActixResponder {
    let logger = SharedLogger::new();

    logger.debug("Start operation list_models");

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
    let request = GetModelRequest{
        req,
        path,
        query,
        body
    };

    // Fetch the list of models
    match client.get_model(&request) {
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