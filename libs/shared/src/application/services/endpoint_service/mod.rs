use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
use crate::domain::entities::endpoint::{Endpoint, NetworkAddressableResource};
use crate::domain::services::endpoint_issuance_service::{
    EndpointIssuanceService, EndpointIssuanceServiceError,
};
use crate::shared_kernel::context::RequestContext;

#[derive(Debug, Error)]
pub enum EndpointServiceError {
    #[error(transparent)]
    Domain(#[from] EndpointIssuanceServiceError),

    #[error(transparent)]
    Repository(#[from] EndpointRepositoryError),
}

pub struct EndpointService {
    endpoint_repository: Arc<dyn EndpointRepository>,
}

impl EndpointService {
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

    pub async fn issue_for_resource(
        &self,
        ctx: &RequestContext,
        resource: &impl NetworkAddressableResource,
    ) -> Result<Vec<Endpoint>, EndpointServiceError> {
        let candidates = EndpointIssuanceService::issue_for_resource(ctx.actor(), resource)?;

        let tenant_id = resource.tenant_id();
        let target_resource_urn = resource.urn();

        let existing_endpoints = retry_async(
            || {
                self.endpoint_repository
                    .list_by_target_urn(tenant_id.as_str(), target_resource_urn.as_str())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?;

        let mut endpoints = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            let existing = existing_endpoints
                .iter()
                .find(|endpoint| endpoint.target_name() == candidate.target_name())
                .cloned();

            match existing {
                Some(endpoint) => endpoints.push(endpoint),
                None => {
                    retry_async(
                        || self.endpoint_repository.save(&candidate),
                        &Self::REPOSITORY_RETRY_POLICY,
                        None,
                    )
                    .await?;

                    endpoints.push(candidate);
                }
            }
        }

        Ok(endpoints)
    }
}

#[cfg(test)]
#[path = "endpoint_service.test.rs"]
mod endpoint_service_test;
