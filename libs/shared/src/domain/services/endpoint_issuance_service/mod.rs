use std::collections::HashSet;

use thiserror::Error;

use crate::domain::entities::endpoint::{Endpoint, NetworkAddressableResource};
use crate::shared_kernel::context::Actor;

pub struct EndpointIssuanceService;

impl EndpointIssuanceService {
    pub fn issue_for_resource(
        actor: &Actor,
        resource: &impl NetworkAddressableResource,
    ) -> Result<Vec<Endpoint>, EndpointIssuanceServiceError> {
        let resource_tenant_id = resource.tenant_id();

        if actor.tenant_id() != &resource_tenant_id {
            return Err(EndpointIssuanceServiceError::TenantMismatch {
                actor_tenant_id: actor.tenant_id().clone(),
                resource_tenant_id,
            });
        }

        let mut target_urls = HashSet::new();

        Ok(resource
            .target_urls()
            .into_iter()
            .filter(|target_url| target_urls.insert(target_url.clone()))
            .map(|target_url| Endpoint::new_from_resource(resource, target_url))
            .collect())
    }
}

#[derive(Debug, Error)]
pub enum EndpointIssuanceServiceError {
    #[error("Actor tenant {actor_tenant_id} does not match resource tenant {resource_tenant_id}")]
    TenantMismatch {
        actor_tenant_id: String,
        resource_tenant_id: String,
    },
}

#[cfg(test)]
#[path = "endpoint_issuance_service.test.rs"]
mod endpoint_issuance_service_test;
