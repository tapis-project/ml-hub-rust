use serde_json::Value;
use platforms::Platform;
use uuid::Uuid;
use crate::domain::entities::{model_metadata::ModelMetadata, deployment_strategy::client_strategy::ClientStrategy};
use crate::domain::entities::deployment::{DesiredState, ModelDeployment, ParallelismStrategy, ReplicaGroup, State};
use crate::application::workflows::reconciliation::ReconciliationAction;

use crate::application::inputs::common::Scope;
use crate::shared_kernel::enums::DeploymentModality;

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
    pub name: String,
    pub description: Option<String>,
    pub platform: Platform,
    pub model_name: String,
    pub model_author: String,
    pub model_scope: Scope,
    pub strategy_name: String,
    pub deployment_modality: DeploymentModality,
    pub replicas: Option<u8>,
    pub parallelism_strategies: Option<Vec<ParallelismStrategy>>,
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