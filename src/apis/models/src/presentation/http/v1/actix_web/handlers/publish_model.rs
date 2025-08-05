use crate::presentation::http::v1::actix_web::helpers::{
    build_client_error_response, build_error_response, build_success_response,
};
use crate::presentation::http::v1::dto::{Headers, PublishArtifactPath, PublishArtifactBody, PublishArtifactRequest};
use actix_web::{post, web, HttpRequest, Responder};
use client_provider::ClientProvider;
use clients::PublishModelClient;
use shared::logging::SharedLogger;
use std::collections::HashMap;

#[post("models-api/artifacts/{artifact_id}/publications")]
async fn publish_model(
    req: HttpRequest,
    path: web::Path<PublishArtifactPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<PublishArtifactBody>,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start publish model operation");

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = PublishArtifactRequest {
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body: body.into_inner(),
    };

    // Catch directory traversal attacks. 'model_id' may be used by clients to
    // constuct directories in the shared file system
    if request.path.artifact_id.contains("..") {
        return build_error_response(403, String::from("Forbidden"));
    }

    // Get the client for the provided platform
    let client = match ClientProvider::provide_publish_model_client(&request.body.platform) {
        Ok(client) => client,
        Err(_) => {
            return build_error_response(
                500,
                String::from(format!(
                    "Failed to find client for platform '{}'",
                    &request.body.platform
                )),
            )
        }
    };

    // Publish the model
    let client_resp = match client.publish_model(&request).await {
        Ok(client_resp) => client_resp,
        Err(err) => {
            logger.debug(&err.to_string());
            return build_client_error_response(err);
        }
    };

    build_success_response(client_resp.result, client_resp.message, None)
}
