use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;
use uuid::Uuid;

use crate::application::inputs::agent::RegisterAgentInput;
use crate::application::ports::agent::{AgentRepository, AgentRepositoryError};
use crate::application::ports::agent_record::{AgentRecordRepository, AgentRecordRepositoryError};
use crate::application::services::endpoint_issuance_service::{
    EndpointIssuanceService, EndpointIssuanceServiceError,
};
use crate::domain::entities::agent::{Agent, AgentError, RegisterAgentProps};
use crate::shared_kernel::context::RequestContext;

#[derive(Debug, Error)]
pub enum AgentServiceError {
    #[error("Agent repository error: {0}")]
    Repository(#[from] AgentRepositoryError),

    #[error("Agent record repository error: {0}")]
    AgentRecordRepository(#[from] AgentRecordRepositoryError),

    #[error("Agent record not found: {0}")]
    AgentRecordNotFound(Uuid),

    #[error("Agent domain error: {0}")]
    Domain(#[from] AgentError),

    #[error("Endpoint issuance error: {0}")]
    EndpointIssuance(#[from] EndpointIssuanceServiceError),
}

pub struct AgentService {
    agent_repository: Arc<dyn AgentRepository>,
    agent_record_repository: Arc<dyn AgentRecordRepository>,
    endpoint_issuance_service: EndpointIssuanceService,
}

impl AgentService {
    const REPOSITORY_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        agent_repository: Arc<dyn AgentRepository>,
        agent_record_repository: Arc<dyn AgentRecordRepository>,
        endpoint_issuance_service: EndpointIssuanceService,
    ) -> Self {
        Self {
            agent_repository,
            agent_record_repository,
            endpoint_issuance_service,
        }
    }

    pub async fn register_agent(
        &self,
        ctx: &RequestContext,
        input: RegisterAgentInput,
    ) -> Result<Agent, AgentServiceError> {
        let agent_record = match input.agent_record_id {
            Some(id) => Some(
                retry_async(
                    || {
                        self.agent_record_repository
                            .find_by_id(ctx.actor_tenant_id(), id)
                    },
                    &Self::REPOSITORY_RETRY_POLICY,
                    None,
                )
                .await?
                .ok_or(AgentServiceError::AgentRecordNotFound(id))?,
            ),
            None => None,
        };

        let agent = Agent::register(
            RegisterAgentProps {
                name: input.name,
                description: input.description,
                owner: ctx.actor_principal_id().clone(),
                tenant_id: ctx.actor_tenant_id().clone(),
                deployment_modality: input.deployment_modality.into(),
                endpoints: input.endpoints.into_iter().map(Into::into).collect(),
                tags: input.tags,
                visibility: input.visibility.into(),
            },
            agent_record.as_ref(),
        )?;

        retry_async(
            || self.agent_repository.save(&agent),
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?;

        self.endpoint_issuance_service
            .issue_for_resource(ctx, &agent)
            .await?;

        Ok(agent)
    }

    pub async fn list_for_user(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<Agent>, AgentServiceError> {
        Ok(retry_async(
            || {
                self.agent_repository
                    .list_by_owner(ctx.actor_tenant_id(), ctx.actor_principal_id())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }

    pub async fn list_shared_with_user(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<Agent>, AgentServiceError> {
        Ok(retry_async(
            || {
                self.agent_repository
                    .list_shared_with_user(ctx.actor_tenant_id(), ctx.actor_principal_id())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }
}

#[cfg(test)]
#[path = "agent_service.test.rs"]
mod agent_service_test;
