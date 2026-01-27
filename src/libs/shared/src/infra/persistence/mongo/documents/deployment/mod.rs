pub mod entity_to_document;

use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};
use crate::infra::persistence::mongo::documents::visibility::Visibility;
use mongodb::bson::{oid::ObjectId, DateTime, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelReference {
    pub name: String,
    pub author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentStrategyReference {
    pub client: String,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ModelDeploymentStatus {
    Submitted,
    Queued,
    Provisioning,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDeployment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub owner: String,
    pub model: ModelReference,
    pub status: ModelDeploymentStatus,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<DeploymentStrategyReference>,
    pub visibility: Visibility,
    pub created_at: DateTime,
    pub last_modified: DateTime,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub parallelism: Option<ReplicaGroup>,
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
pub enum ModelDeploymentInterface {
    RestApi(RestApi)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestApi {
    pub spec: OpenAPI,
}