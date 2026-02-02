use crate::application::inputs::deployment::FindForReconciliationInput;
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
    async fn update_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    async fn update_desired_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    async fn find_for_reconciliation(self, input: FindForReconciliationInput) -> Result<ModelDeployment, ApplicationError>;
}