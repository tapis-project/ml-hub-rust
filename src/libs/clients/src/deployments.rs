use shared::application::inputs::deployment;
use crate::client::Client;
use shared::domain::entities::deployment::ModelDeployment;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelDeploymentError {
    #[error("{0}")]
    Unimplemented(String),
}

#[async_trait::async_trait]
pub trait ModelDeploymentClient: Client {
    async fn deploy_model_with_strategy(&self, _input: &deployment::DeployWithStrategyInput, _provisioner: ()) -> Result<ModelDeployment, ModelDeploymentError> {
        return Err(ModelDeploymentError::Unimplemented("Not implemented".into()));
    }
}