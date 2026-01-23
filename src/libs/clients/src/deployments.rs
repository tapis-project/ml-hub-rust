use shared::application::inputs::deployment;
use crate::client::Client;
use shared::domain::entities::deployment::ModelDeployment;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelDeployentError {
    #[error("{0}")]
    Unimplemented(String),
}

#[async_trait::async_trait]
pub trait CreateModelDeploymentClient: Client {
    async fn deploy_model_with_strategy(&self, _request: &deployment::DeployWithStrategyInput) -> Result<ModelDeployment, ModelDeployentError> {
        return Err(ModelDeployentError::Unimplemented("Not implemented".into()));
    }
}