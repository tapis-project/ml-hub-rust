use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
use crate::domain::entities::endpoint::{Endpoint, NetworkAddressableResource};
use crate::shared_kernel::context::RequestContext;

#[derive(Debug, Error)]
pub enum EndpointCatalogServiceError {
    #[error(transparent)]
    Repository(#[from] EndpointRepositoryError),
}

pub struct EndpointCatalogService {
    endpoint_repository: Arc<dyn EndpointRepository>,
}

impl EndpointCatalogService {
    const REPOSITORY_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(endpoint_repository: Arc<dyn EndpointRepository>) -> Self {
        Self {
            endpoint_repository,
        }
    }

    pub async fn find_by_network_addressable_resource(
        &self,
        ctx: &RequestContext,
        resource: &impl NetworkAddressableResource,
    ) -> Result<Vec<Endpoint>, EndpointCatalogServiceError> {
        let target_resource_urn = resource.urn();

        Ok(retry_async(
            || {
                self.endpoint_repository
                    .list_by_target_urn(ctx.actor_tenant_id(), target_resource_urn.as_str())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }
}

#[cfg(test)]
#[path = "endpoint_catalog_service.test.rs"]
mod endpoint_catalog_service_test;
