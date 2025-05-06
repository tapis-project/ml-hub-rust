use crate::helpers::{build_error_response, build_success_response};
use std::collections::HashMap;
use clients::registrars::ModelsClientRegistrar;
use actix_web::{web, post, Responder, HttpRequest};
use actix_multipart::Multipart;
use shared::logging::SharedLogger;
use shared::models::web::dto::{PublishModelPath, PublishModelRequest};

#[post("models-api/platforms/{platform}/models/{model_id:.*}/files/{path:.*}")]
async fn publish_model(
    req: HttpRequest,
    path: web::Path<PublishModelPath>,
    query: web::Query<HashMap<String, String>>,
    payload: Multipart,
) -> impl Responder {
    let logger = SharedLogger::new();
    
    logger.debug("Start publish model operation");

    // Catch directory traversal attacks. 'model_id' may be used by clients to
    // constuct directories in the shared file system
    if path.model_id.contains("..") {
        return build_error_response(403, String::from("Forbidden"))
    }

    // Initialize the client registrar
    let registrar = ModelsClientRegistrar::new();

    // Get the client for the provided platform
    let client = if let Ok(client) = registrar.get_client(&path.platform) {
        client
    } else {
        return build_error_response(500, String::from(format!("Failed to find client for platform '{}'", &path.platform)))
    };

    // Build the request used by the client
    let request = PublishModelRequest{
        req: req.clone(),
        path,
        query,
        payload
    };

    // Publish the model
    let client_resp = match client.publish_model(&request) {
        Ok(client_resp) => client_resp,
        Err(err) => {
            logger.debug(&err.to_string());
            return build_error_response(500, err.to_string())
        }
    };

    build_success_response(client_resp.result, Some(200), client_resp.message)
}