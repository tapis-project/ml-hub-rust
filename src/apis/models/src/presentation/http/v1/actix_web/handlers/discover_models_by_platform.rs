use crate::presentation::http::v1::actix_web::helpers::{
    build_client_error_response, build_error_response, build_success_response,
};
use crate::presentation::http::v1::requests::{
    DiscoverModelsByPlatformPath, DiscoverModelsByPlatformRequest, DiscoveryCriteria, Headers,
};
use actix_web::{post, web, HttpRequest, Responder};
use clients::DiscoverModelsClient;
use client_provider::ClientProvider;
use shared::presentation::http::v1::contracts::responses;
use std::collections::HashMap;
use platforms::Platform;

#[utoipa::path(
    post,
    path = "/models-api/platforms/{platform}/models",
    tag="Platforms",
    description="Discover models from external platforms",
    params(
        ("platform" = Platform, Path, description = "The platform on which you want to discover models")
    ),
    request_body=DiscoveryCriteria,
    responses(
        (status=200, description="Discovered models", body=responses::DiscoverModelsByPlatformResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[post("models-api/platforms/{platform}/models")]
async fn discover_models_by_platform(
    req: HttpRequest,
    path: web::Path<DiscoverModelsByPlatformPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<DiscoveryCriteria>,
) -> impl Responder {
    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = DiscoverModelsByPlatformRequest {
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