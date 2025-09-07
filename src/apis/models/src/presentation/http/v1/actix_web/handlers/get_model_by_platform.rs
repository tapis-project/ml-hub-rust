use crate::presentation::http::v1::actix_web::helpers::{
    build_client_error_response, build_error_response, build_success_response,
};
use crate::presentation::http::v1::requests::{GetModelPath, GetModelRequest, Headers};
use shared::presentation::http::v1::contracts::responses::GetModelByPlatformResponse;
use actix_web::{get, web, HttpRequest, Responder};
use client_provider::{ClientProvider, Platform};
use clients::GetModelClient;
use shared::logging::SharedLogger;
use std::collections::HashMap;

#[utoipa::path(
    get,
    path="/models-api/platforms/{platform}/models/{model_id}",
    tag="External Models",
    description="Fetch a model from an external platform by id",
    params(
        ("platform"=Platform, Path, description="Name of the platform from which to fetch the model"),
        ("model_id"=String, Path, description="Id of the model the fetch from the source platform"),
    ),
    responses(
        (status=200, description="Model fetched successfully", body=GetModelByPlatformResponse)
    )
)]
#[get("models-api/platforms/{platform}/models/{model_id:.*}")]
async fn get_model_by_platform(
    req: HttpRequest,
    path: web::Path<GetModelPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start operation get_model");

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = GetModelRequest {
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body,
    };

    // Get the client for the provided platform
    let client =
        if let Ok(client) = ClientProvider::provide_get_model_client(&request.path.platform) {
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
    match client.get_model(&request).await {
        Ok(resp) => {
            return build_success_response(resp.result, Some(String::from("success")), None)
        }
        Err(err) => return build_client_error_response(err),
    }
}
