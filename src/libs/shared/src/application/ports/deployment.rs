use crate::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::domain::entities::deployment::ModelDeployment;
use crate::application::errors::ApplicationError;
use async_trait::async_trait;

pub trait DeploymentStrategyProvider {
    fn provide(&self) -> &Vec<ClientStrategySet>;
}


#[async_trait]
pub trait ModelDeploymentRepository: Send + Sync {
    async fn save(&self, model_deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    async fn update_status(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    // async fn list_model_deployments(&self, artifact_type: ArtifactType) -> Result<Vec<Artifact>, ApplicationError>;
}