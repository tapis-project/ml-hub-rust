use async_trait::async_trait;
use uuid::Uuid;
use thiserror::Error;

// Application
use crate::application::ports::errors::CommonRepositoryError;

// Domain
use crate::domain::entities::deployment::argument::Argument;

#[derive(Debug, Error, Clone)]
pub enum DeploymentArgumentRepositoryError {
    #[error(transparent)]
    Persistence(#[from] CommonRepositoryError),
}

#[async_trait]
pub trait DeploymentArgumentRepository {
    async fn save_all(&self, deployment_id: &Uuid, arguments: &[Argument]) -> Result<(), DeploymentArgumentRepositoryError>;
    async fn find_all_for_deployment(&self, deployment_id: Uuid) -> Result<Vec<Argument>, DeploymentArgumentRepositoryError>;
}