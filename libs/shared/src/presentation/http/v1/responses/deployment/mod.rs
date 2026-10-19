mod entity_to_response;

use crate::presentation::http::v1::responses::endpoints::Endpoint;
use crate::presentation::http::v1::responses::visibility::Visibility;
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
    pub tenant_id: String,
    pub owner: String,
    pub external_model_id: Uuid,
    pub deployment_option_id: Uuid,
    pub deployment_modality: DeploymentModality,
    pub state: State,
    pub batch_execution_outcome: Option<BatchExecutionOutcome>,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub visibility: Visibility,
    pub created_at: String,
    pub last_modified: String,
    pub last_state_change: String,
    pub last_observed: Option<String>,
    pub expires_at: Option<String>,
    pub reconciliation_status: Option<ReconciliationStatus>,
    pub retries: u32,
    pub retry_after: Option<String>,
    pub last_desired_state_change: String,
    pub target_endpoint: Option<ModelDeploymentEndpoint>,
    pub endpoint: Option<Endpoint>,
    pub replicas: ReplicaGroup,
    pub metadata: Option<HashMap<String, Value>>,
    pub revision: u32,
}

impl ModelDeployment {
    pub fn populate_endpoint_proxy_url(
        &mut self,
        configuration: &crate::infra::configuration::EndpointRouterConfiguration,
    ) {
        if let Some(endpoint) = self.endpoint.as_mut() {
            endpoint.populate_proxy_url(configuration);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum DeploymentModality {
    Batch,
    Service,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum ReconciliationStatus {
    Pending,
    Starting,
    Stopping,
    Undeploying,
    Retrying,
    Synchronized,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum BatchExecutionOutcome {
    Succeeded,
    Cancelled,
    Failed,
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
pub struct ModelDeploymentEndpoint {
    pub name: Option<String>,
    pub protocol: Protocol,
    pub base_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum Protocol {
    RestHttp,
}
