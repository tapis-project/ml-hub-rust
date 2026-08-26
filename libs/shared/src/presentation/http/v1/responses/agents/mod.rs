mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::presentation::http::v1::responses::agent_records::MessageBinding;
use crate::presentation::http::v1::responses::agent_records::RestHttpLivenessProbe;
use crate::presentation::http::v1::responses::visibility::Visibility;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Agent {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: String,
    pub deployment_modality: AgentDeploymentModality,
    pub liveness: AgentLiveness,
    pub rest_http_endpoints: Vec<RestHttpAgentEndpoint>,
    pub rpc_endpoints: Vec<RpcAgentEndpoint>,
    pub stdio_endpoints: Vec<StdioAgentEndpoint>,
    pub visibility: Visibility,
    pub created_at: String,
    pub last_modified: String,
    #[schema(value_type = Option<String>, format = "uuid")]
    pub agent_record_id: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RestHttpAgentEndpoint {
    pub name: Option<String>,
    pub message_binding: Option<MessageBinding>,
    pub base_url: Option<String>,
    pub liveness_probe: Option<RestHttpLivenessProbe>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RpcAgentEndpoint {
    pub name: Option<String>,
    pub message_binding: Option<MessageBinding>,
    pub base_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct StdioAgentEndpoint {
    pub name: Option<String>,
    pub message_binding: Option<MessageBinding>,
    pub base_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum AgentDeploymentModality {
    Persistent,
    OnDemand,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum AgentLiveness {
    Alive,
    Dead,
}
