pub mod document_to_entity;
pub mod entity_to_document;

use mongodb::bson::{Uuid, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: String,
    pub interfaces: Vec<AgentInterface>,
    pub capabilities: Capabilities,
    pub provider: Option<AgentProvider>,
    pub version: String,
    pub artifact_locators: Vec<ArtifactLocator>,
    pub skills: Vec<AgentSkill>,
    pub icon_url: Option<String>,
    pub documentation_url: Option<String>,
    pub visibility: crate::infra::persistence::mongo::documents::visibility::Visibility,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentInterface {
    pub name: String,
    pub description: Option<String>,
    pub protocol: Protocol,
    pub message_binding: Option<MessageBinding>,
    pub liveness_probe_config: Option<LivenessProbeConfiguration>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Protocol {
    RestHttp,
    Rpc,
    Stdio,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LivenessProbeConfiguration {
    RestHttp {
        route: String,
        interval_seconds: u32,
        timeout_seconds: u32,
        missed_heartbeat_threshold: u16,
        initial_delay_seconds: u32,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Capabilities {
    pub streaming: bool,
    pub push_notifications: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentProvider {
    pub organization: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtifactLocator {
    pub artifact_type: AgentArtifactType,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AgentArtifactType {
    Binary,
    DockerImage,
    HelmChart,
    PythonPackage,
    SourceCode,
    Unspecified,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
}

#[cfg(test)]
#[path = "agent_record.test.rs"]
mod agent_record_test;
