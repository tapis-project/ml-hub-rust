use crate::application::inputs::deployment::FilterInput;
use crate::domain::entities::deployment::ModelDeployment;
use crate::application::errors::ApplicationError;
use async_trait::async_trait;

pub trait DeploymentStrategyProvider {
    fn provide(&self) -> &Vec<ClientStrategySet>;
}

#[async_trait]
pub trait ModelDeploymentRepository: Send + Sync {
    async fn save(&self, model_deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    // async fn update_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    // async fn update_desired_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    async fn find(&self, input: &FilterInput) -> Result<Option<ModelDeployment>, ApplicationError>;
}