use crate::presentation::http::v1::actix_web::helpers::{
    build_client_error_response, build_error_response, build_success_response,
};
use crate::presentation::http::v1::dto::{
    DiscoverModelsPath, DiscoverModelsRequest, DiscoveryCriteriaBody, Headers,
};
use actix_web::{post, web, HttpRequest, Responder};
use client_provider::ClientProvider;
use clients::DiscoverModelsClient;
use shared::logging::SharedLogger;
use std::collections::HashMap;

#[post("models-api/platforms/{platform}/models")]
async fn discover_models(
    req: HttpRequest,
    path: web::Path<DiscoverModelsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<DiscoveryCriteriaBody>,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start operation discover_models");

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = DiscoverModelsRequest {
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body: body.into_inner(),
    };

    // Get the client for the provided platform
    let client = if let Ok(client) =
        ClientProvider::provide_discover_models_client(&request.path.platform)
    {
        client
    } else {
        return build_error_response(
            500,
            String::from(format!(
                "Failed to find client for platform '{}'",
                &request.path.platform
            )),
        );
    };

    // Fetch the list of models
    match client.discover_models(&request).await {
        Ok(resp) => {
            return build_success_response(resp.result, Some(String::from("success")), None)
        }
        Err(err) => return build_client_error_response(err),
    }
}
