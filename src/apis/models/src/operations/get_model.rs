use std::collections::HashMap;
use crate::helpers::{build_error_response, build_success_response};
use clients::registrar::ClientRegistrar;
use shared::logging::SharedLogger;
use shared::models::web::v1::dto::{GetModelPath, GetModelRequest};
use shared::clients::GetModelClient;
use actix_web::{
    web,
    get,
    HttpRequest as ActixHttpRequest,
    Responder
};

#[get("models-api/platforms/{platform}/models/{model_id:.*}")]
async fn get_model(
    req: ActixHttpRequest,
    path: web::Path<GetModelPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start operation list_models");

    // Get the client for the provided platform
    let client = if let Ok(client) = ClientRegistrar::resolve_get_model_client(&path.platform) {
        client
    } else {
        return build_error_response(500, String::from(format!("Failed to find client for platform '{}'", &path.platform)))
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
            return build_success_response(resp.result, Some(String::from("success")), None)
        },
        Err(err) => {
            return build_error_response(500, err.to_string())
        }
    }
}