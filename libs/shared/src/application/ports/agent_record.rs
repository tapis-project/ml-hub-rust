use async_trait::async_trait;
use thiserror::Error;

use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::agent_record::AgentRecord;

#[derive(Debug, Error)]
pub enum AgentRecordRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait AgentRecordRepository: Send + Sync {
    async fn save(&self, agent_record: &AgentRecord) -> Result<(), AgentRecordRepositoryError>;
    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<AgentRecord>, AgentRecordRepositoryError>;
    async fn list_by_tenant(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<AgentRecord>, AgentRecordRepositoryError>;
}
