use crate::application::workflows::reconciliation::ReconciliationAction;
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentId, ParallelismStrategy, State,
};
use crate::domain::entities::{
    deployment_option::DeploymentOptionId, model::external_model::ExternalModel,
};
use crate::shared_kernel::enums::DeploymentModality;

pub struct ClientModelDeploymentRequest {
    pub deployment: ModelDeployment,
    pub external_model: ExternalModel,
}

#[derive(Debug, Clone)]
pub struct FindForReconciliationInput {
    pub deployment_id: ModelDeploymentId,
    pub revision: u32,
}

pub struct FilterInput {
    pub deployment_id: Option<ModelDeploymentId>,
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
    pub deployment_id: ModelDeploymentId,
}

#[derive(Debug)]
pub struct StopModelDeploymentInput {
    pub owner: String,
    pub deployment_id: ModelDeploymentId,
}

#[derive(Debug)]
pub struct UndeployModelDeploymentInput {
    pub owner: String,
    pub deployment_id: ModelDeploymentId,
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
