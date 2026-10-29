use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::endpoint_catalog_query_service::EndpointCatalogQueryService,
    infra::configuration::SiteConfiguration,
    presentation::http::v1::{
        contracts::responses::GetEndpointResponse, requests::endpoints::GetEndpointPath,
        responses::endpoints::Endpoint,
    },
    shared_kernel::context::RequestContext,
};
use uuid::Uuid;

use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response, catalog_error_response,
};

#[utoipa::path(
    get,
    path = "/endpoints-api/endpoints/{endpoint_id}",
    tag = "Endpoints",
    summary = "Get an owned endpoint",
    params(GetEndpointPath),
    responses(
        (
            status = 200,
            description = "Endpoint found",
            body = GetEndpointResponse
        ),
        (
            status = 400,
            description = "Invalid endpoint identifier"
        ),
        (
            status = 404,
            description = "Endpoint not found"
        ),
        (
            status = 500,
            description = "Unable to get endpoint"
        ),
    ),
)]
#[get("endpoints-api/endpoints/{endpoint_id}")]
pub async fn get_endpoint(
    path: web::Path<String>,
    ctx: RequestContext,
    service: web::Data<EndpointCatalogQueryService>,
    site_configuration: web::Data<SiteConfiguration>,
) -> impl Responder {
    let endpoint_id = match Uuid::parse_str(path.as_str()) {
        Ok(endpoint_id) => endpoint_id,
        Err(error) => return build_error_response(400, error.to_string()),
    };

    let output = match service.get(&ctx, &endpoint_id).await {
        Ok(output) => output,
        Err(error) => return catalog_error_response(error),
    };

    let mut endpoint = Endpoint::from(output);
    endpoint.populate_proxy_url(&site_configuration.endpoint_router);

    let result = match to_value(endpoint) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(Some(result), Some("Endpoint found".into()), None)
}
