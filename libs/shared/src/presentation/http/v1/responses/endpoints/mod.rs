mod entity_to_response;

use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Endpoint {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub tenant_id: String,
    pub target_resource_urn: String,
    pub target_name: String,
    pub slug: String,
    pub target_base_url: String,
}

#[derive(Debug, Error)]
pub enum EndpointResponseError {
    #[error("Endpoint target {0} cannot be resolved on the Agent")]
    UnresolvableTarget(String),
}

#[cfg(test)]
#[path = "endpoints.test.rs"]
mod endpoints_test;
