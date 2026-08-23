mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct AgentRecord {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: String,
    pub interfaces: Vec<AgentInterface>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct AgentInterface {
    pub name: String,
    pub description: Option<String>,
    pub protocol: Protocol,
    pub message_binding: Option<MessageBinding>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum Protocol {
    RestHttp,
    Rpc,
    Stdio,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[cfg(test)]
#[path = "agent_records.test.rs"]
mod agent_records_test;
