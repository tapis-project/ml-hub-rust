use async_trait::async_trait;
use thiserror::Error;

// Application
use crate::application::ports::errors::InfrastructureError;

// Domain
use crate::domain::entities::deployment::{argument::Argument, ModelDeploymentId};

#[derive(Debug, Error, Clone)]
pub enum DeploymentArgumentRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait DeploymentArgumentRepository: Send + Sync {
    async fn save_all(
        &self,
        deployment_id: &ModelDeploymentId,
        arguments: &[Argument],
    ) -> Result<(), DeploymentArgumentRepositoryError>;

    async fn find_all_for_deployment(
        &self,
        deployment_id: &ModelDeploymentId,
    ) -> Result<Vec<Argument>, DeploymentArgumentRepositoryError>;
}
