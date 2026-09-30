pub mod client_strategy_set;
mod entity_to_response;
pub mod parameter_set;
pub mod rule_set;
pub mod strategy;

use crate::presentation::http::v1::responses::visibility::Visibility;
use openapiv3::OpenAPI;
use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ModelDeployment {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub platform: Platform,
    pub owner: String,
    pub model: ModelReference,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<String>,
    pub visibility: Visibility,
    pub created_at: String,
    pub last_modified: String,
    pub last_state_change: String,
    pub last_desired_state_change: String,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: ReplicaGroup,
    pub metadata: Option<HashMap<String, Value>>,
    pub revision: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ModelReference {
    pub model_id: uuid::Uuid,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, ToSchema)]
pub enum State {
    NotDeployed,
    Running,
    Stopped,
    Failed,
    Blocked,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, ToSchema)]
pub enum DesiredState {
    Running,
    Stopped,
    NotDeployed,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ReplicaGroup {
    pub count: u8,
    pub parallelism_strategies: Vec<ParallelismStrategy>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum ParallelismStrategy {
    PipelineParallelism,
    TensorParallelism,
    SequenceParallelism,
    ContextParallelism,
    ExpertParallelism,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum ModelDeploymentInterface {
    RestApi(RestApi),
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RestApi {
    #[schema(value_type = Value)]
    pub spec: OpenAPI,
}
