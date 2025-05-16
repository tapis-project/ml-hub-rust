use crate::presentation::http::v1::helpers::{build_error_response, build_success_response};
use std::collections::HashMap;
use client_provider::ClientProvider;
use actix_web::{web, post, Responder, HttpRequest};
use actix_multipart::Multipart;
use shared::logging::SharedLogger;
use crate::presentation::http::v1::dto::{PublishModelPath, PublishModelRequest, Headers};
use shared::clients::PublishModelClient;

#[post("models-api/platforms/{platform}/models/{model_id:.*}/files/{path:.*}")]
async fn publish_model(
    req: HttpRequest,
    path: web::Path<PublishModelPath>,
    query: web::Query<HashMap<String, String>>,
    payload: Multipart,
) -> impl Responder {
    let logger = SharedLogger::new();
    
    logger.debug("Start publish model operation");

     // Build the request used by the client
     let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => {
            return build_error_response(
                400,
                String::from(err.to_string())
            )
        }
    };

    let request = PublishModelRequest{
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        payload
    };

    // Catch directory traversal attacks. 'model_id' may be used by clients to
    // constuct directories in the shared file system
    if request.path.model_id.contains("..") {
        return build_error_response(403, String::from("Forbidden"))
    }

    // Get the client for the provided platform
    let client = match ClientProvider::provide_publish_model_client(&request.path.platform) {
        Ok(client) => client,
        Err(_) => return build_error_response(500, String::from(format!("Failed to find client for platform '{}'", &request.path.platform)))
    };

    // Publish the model
    let client_resp = match client.publish_model(&request) {
        Ok(client_resp) => client_resp,
        Err(err) => {
            logger.debug(&err.to_string());
            return build_error_response(500, err.to_string())
        }
    };

    build_success_response(client_resp.result, client_resp.message, None)
}