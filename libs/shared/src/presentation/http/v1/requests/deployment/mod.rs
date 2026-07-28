mod dto_to_shared_kernel;
mod dto_to_entity;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{presentation::http::v1::requests::common::Scope};

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
    pub model_name: String,
    pub model_author: String,
    pub deployment_modality: DeploymentModality,
    pub params: Value,
    pub replicas: Option<u8>,
    pub parallelism_strategies: Option<Vec<ParallelismStrategy>>,
    #[serde(default = "default_scope")]
    #[param(value_type = Scope, inline, required)]
    /// Selector for global vs tenant-scoped models
    pub scope: Scope
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

fn default_scope () -> Scope { Scope::Tenant }

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StartModelDeploymentPathParams {
    pub deployment_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StopModelDeploymentPathParams {
    pub deployment_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UndeployModelDeploymentPathParams {
    pub deployment_id: Uuid
}