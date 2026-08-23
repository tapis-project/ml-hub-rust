use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateAgentRecordBody {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: String,
    #[validate(length(min = 1))]
    pub supported_interfaces: Vec<AgentInterface>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct AgentInterface {
    pub protocol: Protocol,
    pub message_binding: Option<MessageBinding>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum Protocol {
    RestHttp,
    Rpc,
    Stdio,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[cfg(test)]
#[path = "body.test.rs"]
mod body_test;
