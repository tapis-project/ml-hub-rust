use actix_web::HttpResponse;
use serde_json::Value;
use shared::application::ports::endpoint::EndpointRepositoryError;
use shared::application::services::endpoint_catalog_query_service::EndpointCatalogQueryServiceError;
use shared::presentation::http::v1::actix_web::helpers::{
    build_error_response as error, build_success_response as success,
};

use crate::config::VERSION;

pub fn build_error_response(status: u16, message: String) -> HttpResponse {
    error(status, message, Some(VERSION.into()), None)
}

pub fn catalog_error_response(error: EndpointCatalogQueryServiceError) -> HttpResponse {
    let status = match error {
        EndpointCatalogQueryServiceError::InvalidUrn(_)
        | EndpointCatalogQueryServiceError::EndpointRepository(
            EndpointRepositoryError::InvalidCursor,
        ) => 400,
        EndpointCatalogQueryServiceError::Forbidden => 403,
        EndpointCatalogQueryServiceError::NotFound
        | EndpointCatalogQueryServiceError::ResourceNotFound => 404,
        _ => 500,
    };

    build_error_response(status, error.to_string())
}

pub fn build_success_response(
    result: Option<Value>,
    message: Option<String>,
    metadata: Option<Value>,
) -> HttpResponse {
    success(
        result,
        message,
        Some(VERSION.into()),
        metadata.or_else(|| Some(Value::Object(serde_json::Map::new()))),
    )
}
