use async_trait::async_trait;
use thiserror::Error;

use crate::{
    application::inputs::deployment_option::ListDeploymentOptionsInput,
    application::ports::errors::InfrastructureError,
    domain::entities::{
        deployment_option::{DeploymentOption, DeploymentOptionId},
        model::external_model::ExternalModelId,
    },
};

#[derive(Debug, Error)]
pub enum DeploymentOptionRepositoryError {
    #[error("Invalid deployment option cursor")]
    InvalidCursor,

    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[derive(Debug)]
pub struct DeploymentOptionPage {
    pub deployment_options: Vec<DeploymentOption>,
    pub count: Option<u64>,
    pub cursor: Option<String>,
}

#[async_trait]
pub trait DeploymentOptionRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &DeploymentOptionId,
    ) -> Result<Option<DeploymentOption>, DeploymentOptionRepositoryError>;

    async fn find_by_external_model_id(
        &self,
        external_model_id: &ExternalModelId,
    ) -> Result<Vec<DeploymentOption>, DeploymentOptionRepositoryError>;

    async fn list_by_external_model_id(
        &self,
        external_model_id: &ExternalModelId,
        input: &ListDeploymentOptionsInput,
    ) -> Result<DeploymentOptionPage, DeploymentOptionRepositoryError>;

    async fn replace_for_external_model(
        &self,
        external_model_id: &ExternalModelId,
        deployment_options: &[DeploymentOption],
    ) -> Result<(), DeploymentOptionRepositoryError>;
}
