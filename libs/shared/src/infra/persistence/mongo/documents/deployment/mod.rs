pub mod document_to_entity;
pub mod entity_to_document;

use crate::infra::persistence::mongo::documents::visibility::Visibility;
use mongodb::bson::{oid::ObjectId, DateTime, Uuid};
use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDeployment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub deployment_modality: DeploymentModality,
    pub tenant_id: String,
    pub owner: String,
    pub external_model_id: Uuid,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_option_id: Uuid,
    pub deployment_option_snapshot: DeploymentOptionSnapshot,
    pub visibility: Visibility,
    pub created_at: DateTime,
    pub last_modified: DateTime,
    pub last_state_change: DateTime,
    pub last_desired_state_change: DateTime,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: ReplicaGroup,
    pub metadata: Option<HashMap<String, Value>>,
    pub revision: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentOptionSnapshot {
    pub deployment_option_id: Uuid,
    pub external_model_id: Uuid,
    pub serving_runtime: ServingRuntime,
    pub target_type: DeploymentTargetType,
    pub hpc_cluster_queue: Option<HpcClusterQueueSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServingRuntime {
    FlexServ,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeploymentTargetType {
    HpcClusterQueue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HpcClusterQueueSnapshot {
    pub hpc_cluster_id: Uuid,
    pub batch_scheduler_queue_id: Uuid,
    pub cluster_host: String,
    pub queue_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum State {
    /// The deployment infrastructure does not exist
    NotDeployed,
    /// The deployment infrastructure exists and is running
    Running,
    /// The client has successfully stopped the deployment
    Stopped,
    /// The deployment has failed (never started or crashed)
    Failed,
    /// The deployment cannot be acted up or controlled
    Blocked,
    /// Observability gap. The state of the deployment cannot be known
    Unknown,
}

impl From<State> for String {
    fn from(value: State) -> Self {
        match value {
            State::NotDeployed => "NotDeployed".into(),
            State::Running => "Running".into(),
            State::Stopped => "Stopped".into(),
            State::Failed => "Failed".into(),
            State::Blocked => "Blocked".into(),
            State::Unknown => "Unknown".into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DesiredState {
    Running,
    Stopped,
    NotDeployed,
}

impl From<DesiredState> for String {
    fn from(value: DesiredState) -> Self {
        match value {
            DesiredState::Running => "Running".into(),
            DesiredState::Stopped => "Stopped".into(),
            DesiredState::NotDeployed => "NotDeployed".into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentModality {
    Batch,
    Service,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaGroup {
    pub count: u8,
    pub parallelism_strategies: Vec<ParallelismStrategy>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cores: Option<f32>,
    pub disk: Option<f32>,
    pub memory: Option<f32>,
    pub gpu: Option<GpuResource>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpuResource {
    pub memory: Option<f32>,
    pub vendor: Option<String>,
    pub gpu_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParallelismStrategy {
    PipelineParallelism,
    TensorParallelism,
    SequenceParallelism,
    ContextParallelism,
    ExpertParallelism,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDeploymentInterface {
    pub interface_type: ModelDeploymentInterfaceType,
    pub rest_api: Option<RestApi>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelDeploymentInterfaceType {
    RestApi,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestApi {
    pub spec: OpenAPI,
}
