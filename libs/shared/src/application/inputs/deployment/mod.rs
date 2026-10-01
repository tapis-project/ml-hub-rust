use crate::application::workflows::reconciliation::ReconciliationAction;
use crate::domain::entities::deployment::{
    DesiredState, ModelDeployment, ParallelismStrategy, State,
};
use crate::domain::entities::{
    deployment_option::DeploymentOptionId, model::external_model::ExternalModel,
};
use uuid::Uuid;

use crate::shared_kernel::enums::DeploymentModality;

pub struct ClientModelDeploymentRequest {
    pub deployment: ModelDeployment,
    pub external_model: ExternalModel,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DeployWithOptionInput {
    pub name: String,
    pub description: Option<String>,
    pub deployment_option_id: DeploymentOptionId,
    pub deployment_modality: DeploymentModality,
    pub replicas: Option<u8>,
    pub parallelism_strategies: Option<Vec<ParallelismStrategy>>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub parameter_name: String,
    pub value: String,
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
    pub external_model: ExternalModel,
}

#[derive(Clone)]
pub struct UpdateModelDeploymentInput {
    pub deployment: ModelDeployment,
}
