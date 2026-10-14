mod entity_to_response;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Endpoint {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub tenant_id: String,
    pub target_resource_urn: String,
    pub target_name: String,
    pub slug: String,
    pub proxy_url: String,
    pub target_base_url: String,
}

impl Endpoint {
    pub fn populate_proxy_url(
        &mut self,
        configuration: &crate::infra::configuration::EndpointRouterConfiguration,
    ) {
        self.proxy_url = configuration.build_public_url(&self.tenant_id, &self.slug);
    }
}

#[derive(Debug, Error)]
pub enum EndpointResponseError {
    #[error("Endpoint target {0} cannot be resolved on the resource")]
    UnresolvableTarget(String),
}

#[cfg(test)]
#[path = "endpoints.test.rs"]
mod endpoints_test;
