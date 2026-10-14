use std::sync::Arc;

use retry_utils::{retry_async, ExponentialBackoff, Jitter, Retry, RetryPolicy};
use thiserror::Error;
use uuid::Uuid;

use crate::application::ports::agent::{AgentRepository, AgentRepositoryError};
use crate::application::ports::deployment::{
    ModelDeploymentRepository, ModelDeploymentRepositoryError,
};
use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
use crate::domain::entities::deployment::ModelDeploymentId;
use crate::domain::entities::endpoint::{
    NetworkAddressableResource, NetworkTargetError, ResolvedNetworkTarget,
};
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::identifiers::urn::{NetworkResourceReference, UrnParseError};

#[derive(Clone, Debug)]
pub struct ResolvedEndpointRoute {
    endpoint_id: Uuid,
    owner: String,
    target: ResolvedNetworkTarget,
}

impl ResolvedEndpointRoute {
    pub fn endpoint_id(&self) -> &Uuid {
        &self.endpoint_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn target(&self) -> &ResolvedNetworkTarget {
        &self.target
    }
}

pub struct EndpointRoutingService {
    endpoint_repository: Arc<dyn EndpointRepository>,
    agent_repository: Arc<dyn AgentRepository>,
    deployment_repository: Arc<dyn ModelDeploymentRepository>,
}

impl EndpointRoutingService {
    pub fn new(
        endpoint_repository: Arc<dyn EndpointRepository>,
        agent_repository: Arc<dyn AgentRepository>,
        deployment_repository: Arc<dyn ModelDeploymentRepository>,
    ) -> Self {
        Self {
            endpoint_repository,
            agent_repository,
            deployment_repository,
        }
    }

    pub async fn resolve(
        &self,
        ctx: &RequestContext,
        slug: &str,
    ) -> Result<ResolvedEndpointRoute, EndpointRoutingServiceError> {
        let tenant_id = ctx.actor_tenant_id();

        let endpoint = retry_async(
            || {
                self.endpoint_repository
                    .get_by_tenant_and_slug(tenant_id, slug)
            },
            &repository_retry_policy(),
            None,
        )
        .await?
        .ok_or(EndpointRoutingServiceError::EndpointNotFound)?;

        let reference = endpoint.target_resource_urn().parse_network_resource()?;

        if reference.tenant_id() != tenant_id {
            return Err(EndpointRoutingServiceError::DataIntegrity(
                "Endpoint URN tenant does not match the Endpoint tenant".into(),
            ));
        }

        let resolved = match reference {
            NetworkResourceReference::Agent { id, .. } => {
                let agent = retry_async(
                    || self.agent_repository.find_by_id(tenant_id, &id),
                    &repository_retry_policy(),
                    None,
                )
                .await?
                .ok_or(EndpointRoutingServiceError::ReferencedResourceNotFound)?;

                if agent.owner() != ctx.actor_principal_id() {
                    return Err(EndpointRoutingServiceError::Forbidden);
                }

                let target = agent
                    .resolve_network_target(endpoint.target_name())?
                    .ok_or(EndpointRoutingServiceError::TargetUnavailable)?;

                ResolvedEndpointRoute {
                    endpoint_id: *endpoint.id(),
                    owner: agent.owner().to_owned(),
                    target,
                }
            }
            NetworkResourceReference::ModelDeployment { id, .. } => {
                let deployment_id = ModelDeploymentId::reconstitute(id);

                let deployment = retry_async(
                    || {
                        self.deployment_repository
                            .find_by_id_and_tenant(tenant_id, &deployment_id)
                    },
                    &repository_retry_policy(),
                    None,
                )
                .await?
                .ok_or(EndpointRoutingServiceError::ReferencedResourceNotFound)?;

                if deployment.owner() != ctx.actor_principal_id() {
                    return Err(EndpointRoutingServiceError::Forbidden);
                }

                let target = deployment
                    .resolve_network_target(endpoint.target_name())?
                    .ok_or(EndpointRoutingServiceError::TargetUnavailable)?;

                ResolvedEndpointRoute {
                    endpoint_id: *endpoint.id(),
                    owner: deployment.owner().to_owned(),
                    target,
                }
            }
        };

        Ok(resolved)
    }
}

fn repository_retry_policy() -> RetryPolicy {
    RetryPolicy::ExponentialBackoff(ExponentialBackoff {
        retries: Retry::NTimes(3),
        delay: 50,
        base: Some(2),
        max_delay: 500,
        jitter: Some(Jitter::Full),
    })
}

#[derive(Debug, Error)]
pub enum EndpointRoutingServiceError {
    #[error("Endpoint not found")]
    EndpointNotFound,

    #[error("Endpoint access is forbidden")]
    Forbidden,

    #[error("Endpoint target is temporarily unavailable")]
    TargetUnavailable,

    #[error("Endpoint references a missing resource")]
    ReferencedResourceNotFound,

    #[error("Endpoint data integrity error: {0}")]
    DataIntegrity(String),

    #[error(transparent)]
    EndpointRepository(#[from] EndpointRepositoryError),

    #[error(transparent)]
    AgentRepository(#[from] AgentRepositoryError),

    #[error(transparent)]
    DeploymentRepository(#[from] ModelDeploymentRepositoryError),

    #[error(transparent)]
    InvalidUrn(#[from] UrnParseError),

    #[error(transparent)]
    InvalidTarget(#[from] NetworkTargetError),
}
