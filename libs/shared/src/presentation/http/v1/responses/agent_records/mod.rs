mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::presentation::http::v1::responses::visibility::Visibility;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct AgentRecord {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: String,
    pub rest_http_interfaces: Vec<RestHttpAgentInterface>,
    pub rpc_interfaces: Vec<RpcAgentInterface>,
    pub stdio_interfaces: Vec<StdioAgentInterface>,
    pub capabilities: Capabilities,
    pub provider: Option<AgentProvider>,
    pub version: String,
    pub artifact_locators: Vec<ArtifactLocator>,
    pub skills: Vec<AgentSkill>,
    pub tags: Vec<String>,
    pub icon_url: Option<String>,
    pub documentation_url: Option<String>,
    pub visibility: Visibility,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct AgentProvider {
    pub organization: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct ArtifactLocator {
    pub artifact_type: AgentArtifactType,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct AgentSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum AgentArtifactType {
    Binary,
    DockerImage,
    HelmChart,
    PythonPackage,
    SourceCode,
    Unspecified,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Capabilities {
    pub streaming: bool,
    pub push_notifications: bool,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RestHttpAgentInterface {
    pub name: String,
    pub description: Option<String>,
    pub message_binding: Option<MessageBinding>,
    pub liveness_probe_config: Option<RestHttpLivenessProbe>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RpcAgentInterface {
    pub name: String,
    pub description: Option<String>,
    pub message_binding: Option<MessageBinding>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct StdioAgentInterface {
    pub name: String,
    pub description: Option<String>,
    pub message_binding: Option<MessageBinding>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RestHttpLivenessProbe {
    pub route: String,
    pub interval_seconds: u32,
    pub timeout_seconds: u32,
    pub missed_heartbeat_threshold: u16,
    pub initial_delay_seconds: u32,
}

#[cfg(test)]
#[path = "agent_records.test.rs"]
mod agent_records_test;
