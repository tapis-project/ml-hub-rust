use crate::config::VERSION;
use std::collections::HashMap;
use clients::registrars::ModelsClientRegistrar;
use actix_web::{
    web,
    post,
    HttpRequest as ActixHttpRequest,
    HttpResponse,
    Responder as ActixResponder
};
use actix_multipart::Multipart;
use shared::logging::SharedLogger;
use shared::requests::{PublishModelPath, PublishModelRequest};
use shared::responses::JsonResponse;

#[post("models-api/platforms/{platform}/models/{model_id:.*}/files/{path:.*}")]
async fn publish_model(
    req: ActixHttpRequest,
    path: web::Path<PublishModelPath>,
    query: web::Query<HashMap<String, String>>,
    payload: Multipart,
) -> impl ActixResponder {
    let logger = SharedLogger::new();
    
    logger.debug("Start publish model operation");

    // Catch directory traversal attacks. 'model_id' may be used by clients to
    // constuct directories in the shared file system
    if path.model_id.contains("..") {
        return HttpResponse::Forbidden()
            .content_type("application/json")
            .json(JsonResponse {
                status: Some(403),
                message: Some(String::from("Forbidden")),
                result: None,
                metadata: None,
                version: Some(VERSION.to_string())
            });
    }

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
    let request = PublishModelRequest{
        req: req.clone(),
        path,
        query,
        payload
    };

    // Publish the model
    let _client_resp = match client.publish_model(&request) {
        Ok(client_resp) => client_resp,
        Err(err) => {
            logger.debug(&err.to_string());
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
    };

    return HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(JsonResponse {
            status: Some(501),
            message: Some(String::from("Artifact responses for MIME type multipart/mixed not yet implemented")),
            result: None,
            metadata: None,
            version: Some(VERSION.to_string())
        })
}