use actix_web::{get, web, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::services::endpoint_catalog_query_service::EndpointCatalogQueryService,
    infra::configuration::SiteConfiguration,
    presentation::http::v1::{
        contracts::responses::ListEndpointsResponse,
        requests::endpoints::{ListEndpointsQuery, ListResourceEndpointsPath},
        responses::endpoints::Endpoint,
    },
    shared_kernel::{context::RequestContext, identifiers::urn::Urn},
};

use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response, catalog_error_response,
};

#[utoipa::path(
    get,
    path = "/endpoints-api/resources/{resource_urn}/endpoints",
    tag = "Endpoints",
    summary = "List endpoints issued for an owned resource",
    params(
        ListResourceEndpointsPath,
        ListEndpointsQuery
    ),
    responses(
        (
            status = 200,
            description = "Resource endpoints listed",
            body = ListEndpointsResponse
        ),
        (
            status = 400,
            description = "Invalid resource URN or pagination parameters"
        ),
        (
            status = 403,
            description = "Resource access is forbidden"
        ),
        (
            status = 404,
            description = "Resource not found"
        ),
        (
            status = 500,
            description = "Unable to list resource endpoints"
        ),
    ),
)]
#[get("endpoints-api/resources/{resource_urn}/endpoints")]
pub async fn list_resource_endpoints(
    path: web::Path<String>,
    query: web::Query<ListEndpointsQuery>,
    ctx: RequestContext,
    service: web::Data<EndpointCatalogQueryService>,
    site_configuration: web::Data<SiteConfiguration>,
) -> impl Responder {
    let output = match service
        .list_for_resource(
            &ctx,
            Urn::new(path.into_inner()),
            query.limit,
            query.cursor.clone(),
            query.include_count.unwrap_or(false),
        )
        .await
    {
        Ok(output) => output,
        Err(error) => return catalog_error_response(error),
    };

    let mut endpoints = output
        .endpoints
        .into_iter()
        .map(Endpoint::from)
        .collect::<Vec<_>>();

    for endpoint in &mut endpoints {
        endpoint.populate_proxy_url(&site_configuration.endpoint_router);
    }

    let result = match to_value(endpoints) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let mut metadata = Map::new();

    if let Some(cursor) = output.cursor {
        metadata.insert("cursor".into(), Value::String(cursor));
    }

    if let Some(count) = output.count {
        metadata.insert("count".into(), Value::Number(count.into()));
    }

    build_success_response(
        Some(result),
        Some("Successfully listed resource endpoints".into()),
        Some(Value::Object(metadata)),
    )
}
