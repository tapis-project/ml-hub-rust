use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy, RetryStrategyAction};
use thiserror::Error;

use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
use crate::domain::entities::endpoint::{Endpoint, NetworkAddressableResource};
use crate::domain::services::endpoint_issuance_service::{
    EndpointIssuanceService as EndpointIssuanceDomainService,
    EndpointIssuanceServiceError as EndpointIssuanceDomainServiceError,
};
use crate::shared_kernel::context::RequestContext;

#[derive(Debug, Error)]
pub enum EndpointIssuanceServiceError {
    #[error(transparent)]
    Domain(#[from] EndpointIssuanceDomainServiceError),

    #[error(transparent)]
    Repository(#[from] EndpointRepositoryError),
}

pub struct EndpointIssuanceService {
    endpoint_repository: Arc<dyn EndpointRepository>,
}

impl EndpointIssuanceService {
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
    ) -> Result<Vec<Endpoint>, EndpointIssuanceServiceError> {
        let candidates = EndpointIssuanceDomainService::issue_for_resource(ctx.actor(), resource)?;

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
                    let save_result = retry_async(
                        || self.endpoint_repository.save(&candidate),
                        &Self::REPOSITORY_RETRY_POLICY,
                        |error: &EndpointRepositoryError, _| match error {
                            EndpointRepositoryError::AlreadyExists => {
                                RetryStrategyAction::ReturnResult
                            }
                            EndpointRepositoryError::Persistence(_) => {
                                RetryStrategyAction::ContinueRetries
                            }
                        },
                    )
                    .await;

                    if let Err(EndpointRepositoryError::AlreadyExists) = save_result {
                        let existing = retry_async(
                            || {
                                self.endpoint_repository.get_by_target(
                                    tenant_id.as_str(),
                                    target_resource_urn.as_str(),
                                    candidate.target_name(),
                                )
                            },
                            &Self::REPOSITORY_RETRY_POLICY,
                            None,
                        )
                        .await?
                        .ok_or(EndpointRepositoryError::AlreadyExists)?;

                        endpoints.push(existing);

                        continue;
                    }

                    save_result?;

                    endpoints.push(candidate);
                }
            }
        }

        Ok(endpoints)
    }
}

#[cfg(test)]
#[path = "endpoint_issuance_service.test.rs"]
mod endpoint_issuance_service_test;
