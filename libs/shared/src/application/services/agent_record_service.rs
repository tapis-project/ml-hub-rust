use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::application::inputs::agent_record::CreateAgentRecordInput;
use crate::application::ports::agent_record::{AgentRecordRepository, AgentRecordRepositoryError};
use crate::domain::entities::agent_record::{AgentRecord, AgentRecordError, AgentSkillError};
use crate::shared_kernel::context::RequestContext;

#[derive(Debug, Error)]
pub enum AgentRecordServiceError {
    #[error("Agent record repository error: {0}")]
    Repository(#[from] AgentRecordRepositoryError),

    #[error("Agent record domain error: {0}")]
    Domain(#[from] AgentRecordError),

    #[error("Agent skill domain error: {0}")]
    Skill(#[from] AgentSkillError),
}

pub struct AgentRecordService {
    agent_record_repository: Arc<dyn AgentRecordRepository>,
}

impl AgentRecordService {
    const REPOSITORY_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(agent_record_repository: Arc<dyn AgentRecordRepository>) -> Self {
        Self {
            agent_record_repository,
        }
    }

    pub async fn create_agent_record(
        &self,
        ctx: &RequestContext,
        input: CreateAgentRecordInput,
    ) -> Result<AgentRecord, AgentRecordServiceError> {
        let agent_record = AgentRecord::new(
            input.name,
            ctx.actor_tenant_id().clone(),
            ctx.actor_principal_id().clone(),
            input.description,
            input.interfaces.into_iter().map(Into::into).collect(),
            input.capabilities.into(),
            input.provider.map(Into::into),
            input.version,
            input
                .artifact_locators
                .into_iter()
                .map(Into::into)
                .collect(),
            input
                .skills
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            input.icon_url,
            input.documentation_url,
            input.visibility.into(),
        )?;

        retry_async(
            || self.agent_record_repository.save(&agent_record),
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(agent_record)
    }

    pub async fn list_for_user(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<AgentRecord>, AgentRecordServiceError> {
        let agent_records = retry_async(
            || {
                self.agent_record_repository
                    .list_by_owner(ctx.actor_tenant_id(), ctx.actor_principal_id())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(agent_records)
    }

    pub async fn list_shared_with_user(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<AgentRecord>, AgentRecordServiceError> {
        let agent_records = retry_async(
            || {
                self.agent_record_repository
                    .list_shared_with_user(ctx.actor_tenant_id(), ctx.actor_principal_id())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(agent_records)
    }
}

#[cfg(test)]
#[path = "agent_record_service.test.rs"]
mod agent_record_service_test;
