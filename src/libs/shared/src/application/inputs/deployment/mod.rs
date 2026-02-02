use serde_json::Value;
use platforms::Platform;
use uuid::Uuid;
use crate::domain::entities::{model_metadata::ModelMetadata, deployment_strategy::client_strategy::ClientStrategy};
use crate::domain::entities::deployment::{ModelDeployment, State};
use crate::application::workflows::reconciliation::ReconciliationAction;

pub struct ClientModelDeploymentRequest {
    pub deployment: ModelDeployment,
    pub metadata: ModelMetadata,
    pub strategy: Option<ClientStrategy>
}

pub struct FindForReconciliationInput {
    pub deployemnt_id: Uuid,
    pub revision: u32,
    pub state: State,
}

pub struct DeployWithStrategyInput {
    pub owner: String,
    pub platform: Platform,
    pub model_name: String,
    pub model_author: String,
    pub strategy_name: String,
    pub params: Value,
}

pub struct ReconcileDeploymentInput {
    action: ReconciliationAction,
    model_metadata: ModelMetadata,
    deployemnt: ModelDeployment,
}