use std::collections::HashMap;
use crate::presentation::http::v1::helpers::{build_error_response, build_client_error_response, build_success_response};
use client_provider::ClientProvider;
use actix_web::{
    web,
    get,
    HttpRequest,
    Responder,
};
use shared::logging::SharedLogger;
use crate::presentation::http::v1::dto::{ListModelsPath, ListModelsRequest, Headers};
use clients::ListModelsClient;

#[get("models-api/platforms/{platform}/models")]
async fn list_models(
    req: HttpRequest,
    path: web::Path<ListModelsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Start operation list_models");
    logger.debug(format!("path: {:#?}", path).as_str());

    // Get the client for the provided platform
    let client = if let Ok(client) = ClientProvider::provide_list_models_client(&path.platform) {
        client
    } else {
        return build_error_response(
            500,
            String::from(format!("Failed to proivde client for platform '{}'", &path.platform))
        )
    };

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

    let request = ListModelsRequest{
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body
    };

    // Fetch the list of models
    match client.list_models(&request) {
        Ok(resp) => {
            return build_success_response(resp.result, Some(String::from("success")), None);
        },
        Err(err) => {
            return build_client_error_response(err)
        }
    }
}