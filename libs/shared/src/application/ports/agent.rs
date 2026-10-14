use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::agent::Agent;

#[derive(Debug, Error)]
pub enum AgentRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait AgentRepository: Send + Sync {
    async fn find_by_id(
        &self,
        tenant_id: &str,
        id: &Uuid,
    ) -> Result<Option<Agent>, AgentRepositoryError>;

    async fn save(&self, agent: &Agent) -> Result<(), AgentRepositoryError>;

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<Agent>, AgentRepositoryError>;

    async fn list_shared_with_user(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<Agent>, AgentRepositoryError>;
}
