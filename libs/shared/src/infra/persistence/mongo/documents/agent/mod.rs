pub mod document_to_entity;
pub mod entity_to_document;

use mongodb::bson::{oid::ObjectId, Uuid};
use serde::{Deserialize, Serialize};

use crate::infra::persistence::mongo::documents::agent_record::{
    LivenessProbeConfiguration, MessageBinding, Protocol,
};
use crate::infra::persistence::mongo::documents::visibility::Visibility;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Agent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub deployment_modality: AgentDeploymentModality,
    pub liveness: AgentLiveness,
    pub target_endpoints: Vec<AgentEndpoint>,
    pub visibility: Visibility,
    pub created_at: String,
    pub last_modified: String,
    pub agent_record_id: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentEndpoint {
    pub name: Option<String>,
    pub protocol: Protocol,
    pub message_binding: Option<MessageBinding>,
    pub base_url: Option<String>,
    pub liveness_probe: Option<LivenessProbeConfiguration>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AgentLiveness {
    Alive,
    Dead,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AgentDeploymentModality {
    Persistent,
    OnDemand,
}
