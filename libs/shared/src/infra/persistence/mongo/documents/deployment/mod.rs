pub mod entity_to_document;
pub mod document_to_entity;

use std::collections::HashMap;
use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};
use crate::infra::persistence::mongo::documents::visibility::Visibility;
use mongodb::bson::{oid::ObjectId, DateTime, Uuid};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDeployment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub deployment_modality: DeploymentModality,
    pub tenant_id: String,
    pub platform: platforms::Platform,
    pub owner: String,
    pub model: ModelReference,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<String>,
    pub visibility: Visibility,
    pub created_at: DateTime,
    pub last_modified: DateTime,
    pub last_state_change: DateTime,
    pub last_desired_state_change: DateTime,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: Option<ReplicaGroup>,
    pub metadata: Option<HashMap<String, Value>>,
    pub revision: u32, 
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelReference {
    pub name: String,
    pub author: String,
    pub tenant_id: String,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct DeploymentStrategyReference {
//     pub platform: Platform,
//     pub name: String,
// }

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
    Service
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaGroup {
    pub count: u8,
    pub resources: ResourceRequirements,
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
    DataSharding,
    ModelSharding,
    PipelineSharding,
    TensorSharding,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelDeploymentInterface {
    RestApi(RestApi)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestApi {
    pub spec: OpenAPI,
}