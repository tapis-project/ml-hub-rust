mod dto_to_entity;
mod dto_to_input;
mod dto_to_shared_kernel;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, IntoParams, ToSchema)]
pub struct DeployModelWithStrategyPathParams {
    /// The target platform for the Model Deployment
    pub platform: Platform,
    /// The name of the deployment strategy
    pub strategy_name: String,
}

#[derive(Debug, Deserialize, Clone, IntoParams, ToSchema)]
pub struct DeployModelWithStrategyBody {
    pub name: String,
    pub description: Option<String>,
    pub model_id: Uuid,
    pub deployment_modality: DeploymentModality,
    pub arguments: Option<Vec<Argument>>,
    pub replicas: Option<u8>,
    pub parallelism_strategies: Option<Vec<ParallelismStrategy>>,
}

#[derive(Clone, Debug, Deserialize, ToSchema, Serialize)]
pub struct Argument {
    pub parameter_name: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, ToSchema, Serialize)]
pub enum ParallelismStrategy {
    PipelineParallelism,
    TensorParallelism,
    SequenceParallelism,
    ContextParallelism,
    ExpertParallelism,
}

#[derive(Clone, Debug, ToSchema, Deserialize)]
pub enum DeploymentModality {
    Batch,
    Service,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StartModelDeploymentPathParams {
    pub deployment_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StopModelDeploymentPathParams {
    pub deployment_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UndeployModelDeploymentPathParams {
    pub deployment_id: Uuid,
}
