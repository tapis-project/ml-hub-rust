use serde_json::Value;
use platforms::Platform;
use uuid::Uuid;
use crate::domain::entities::{model_metadata::ModelMetadata, deployment_strategy::client_strategy::ClientStrategy};
use crate::domain::entities::deployment::{DesiredState, ModelDeployment, State};
use crate::application::workflows::reconciliation::ReconciliationAction;

pub struct ClientModelDeploymentRequest {
    pub deployment: ModelDeployment,
    pub metadata: ModelMetadata,
    pub strategy: Option<ClientStrategy>
}

pub struct FindForReconciliationInput {
    pub deployment_id: Uuid,
    pub revision: u32,
    pub desired_state: DesiredState,
    pub state: State,
}

pub struct FilterInput {
    pub deployment_id: Option<Uuid>,
    pub revision: Option<u32>,
    pub state: Option<State>,
}

#[derive(Debug)]
pub struct DeployWithStrategyInput {
    pub owner: String,
    pub platform: Platform,
    pub model_name: String,
    pub model_author: String,
    pub strategy_name: String,
    pub params: Value,
}

#[derive(Debug)]
pub struct StartModelDeploymentInput {
    pub owner: String,
    pub deployment_id: Uuid,
}

#[derive(Debug)]
pub struct StopModelDeploymentInput {
    pub owner: String,
    pub deployment_id: Uuid,
}

#[derive(Debug)]
pub struct UndeployModelDeploymentInput {
    pub owner: String,
    pub deployment_id: Uuid,
}

pub struct ReconcileModelDeploymentInput {
    pub action: ReconciliationAction,
    pub deployment: ModelDeployment,
    pub model_metadata: ModelMetadata,
}

#[derive(Clone)]
pub struct UpdateModelDeploymentInput {
    pub deployment: ModelDeployment,
}