use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateAgentRecordBody {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: String,
    #[validate(
        length(min = 1),
        nested,
        custom(function = "validate_unique_interface_names")
    )]
    pub interfaces: Vec<AgentInterface>,
    pub capabilities: Capabilities,
    #[validate(nested)]
    pub provider: Option<AgentProvider>,
    #[validate(length(min = 1))]
    pub version: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
pub struct AgentProvider {
    #[validate(length(min = 1))]
    pub organization: String,
    #[validate(url)]
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct Capabilities {
    pub streaming: bool,
    pub push_notifications: bool,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
pub struct AgentInterface {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
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

fn validate_unique_interface_names(
    interfaces: &Vec<AgentInterface>,
) -> Result<(), ValidationError> {
    let mut names = HashSet::new();

    for interface in interfaces {
        if !names.insert(&interface.name) {
            return Err(ValidationError::new("duplicate_agent_interface_name"));
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "body.test.rs"]
mod body_test;
