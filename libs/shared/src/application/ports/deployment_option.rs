use async_trait::async_trait;
use thiserror::Error;

use crate::{
    application::ports::errors::InfrastructureError,
    domain::entities::deployment_option::DeploymentOption,
    shared_kernel::identifiers::ExternalModelId,
};

#[derive(Debug, Error)]
pub enum DeploymentOptionRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait DeploymentOptionRepository: Send + Sync {
    async fn find_by_external_model_id(
        &self,
        external_model_id: &ExternalModelId,
    ) -> Result<Vec<DeploymentOption>, DeploymentOptionRepositoryError>;

    async fn replace_for_external_model(
        &self,
        external_model_id: &ExternalModelId,
        deployment_options: &[DeploymentOption],
    ) -> Result<(), DeploymentOptionRepositoryError>;
}
