use crate::presentation::http::v1::actix_web::helpers::{
    build_client_error_response, build_error_response, build_success_response,
};
use crate::presentation::http::v1::requests::{Headers, ListModelsPath, ListModelsRequest};
use actix_web::{get, web, HttpRequest, Responder};
use client_provider::ClientProvider;
use clients::ListModelsClient;
use platforms::Platform;
use shared::logging::SharedLogger;
use shared::presentation::http::v1::contracts;
use std::collections::HashMap;

#[utoipa::path(
    get,
    path="/models-api/platforms/{platform}/models",
    tag="Platforms",
    description="List models from an external platform",
    params(
        ("platform" = Platform, Path, description = "The platform for which you want to list the models"),
    ),
    responses(
        (status=200, description="Listed models", body=contracts::responses::ListModelsByPlatformResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("models-api/platforms/{platform}/models")]
async fn list_models_by_platform(
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
            String::from(format!(
                "Failed to proivde client for platform '{}'",
                &path.platform
            )),
        );
    };

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = ListModelsRequest {
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body,
    };

    // Fetch the list of models
    match client.list_models(&request).await {
        Ok(resp) => {
            return build_success_response(resp.result, Some(String::from("success")), None);
        }
        Err(err) => return build_client_error_response(err),
    }
}
