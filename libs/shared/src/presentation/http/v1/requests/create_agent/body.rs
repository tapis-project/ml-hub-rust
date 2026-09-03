use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::presentation::http::v1::requests::create_agent_record::body::{
    MessageBinding, RestHttpLivenessProbe, Visibility,
};
use crate::shared_kernel::value_objects::{MAX_TAG_LENGTH_BYTES, MAX_TAGS};

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_endpoint_collections"))]
pub struct CreateAgentBody {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: String,
    pub deployment_modality: AgentDeploymentModality,
    #[serde(default)]
    #[validate(nested)]
    pub rest_http_endpoints: Vec<RestHttpAgentEndpoint>,
    #[serde(default)]
    #[validate(nested)]
    pub rpc_endpoints: Vec<RpcAgentEndpoint>,
    #[serde(default)]
    #[validate(nested)]
    pub stdio_endpoints: Vec<StdioAgentEndpoint>,
    #[serde(default)]
    #[validate(custom(function = "validate_tags"))]
    pub tags: Vec<String>,
    pub agent_record_id: Option<Uuid>,
    #[serde(default)]
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RestHttpAgentEndpoint {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub message_binding: Option<MessageBinding>,
    #[validate(url)]
    pub base_url: Option<String>,
    pub liveness_probe: Option<RestHttpLivenessProbe>,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RpcAgentEndpoint {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub message_binding: Option<MessageBinding>,
    #[validate(url)]
    pub base_url: Option<String>,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct StdioAgentEndpoint {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub message_binding: Option<MessageBinding>,
    #[validate(url)]
    pub base_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum AgentDeploymentModality {
    Persistent,
    OnDemand,
}

fn validate_endpoint_collections(body: &CreateAgentBody) -> Result<(), ValidationError> {
    let mut names = HashSet::new();
    let endpoints = body
        .rest_http_endpoints
        .iter()
        .map(|endpoint| &endpoint.name)
        .chain(body.rpc_endpoints.iter().map(|endpoint| &endpoint.name))
        .chain(body.stdio_endpoints.iter().map(|endpoint| &endpoint.name));
    let mut count = 0;

    for name in endpoints {
        count += 1;
        if let Some(name) = name {
            if !names.insert(name) {
                return Err(ValidationError::new("duplicate_agent_endpoint_name"));
            }
        }
    }

    if count == 0 {
        return Err(ValidationError::new("missing_agent_endpoints"));
    }
    Ok(())
}

fn validate_tags(tags: &Vec<String>) -> Result<(), ValidationError> {
    if tags.len() > MAX_TAGS {
        return Err(ValidationError::new("too_many_tags"));
    }

    if tags
        .iter()
        .any(|tag| tag.is_empty() || tag.len() > MAX_TAG_LENGTH_BYTES)
    {
        return Err(ValidationError::new("invalid_tag"));
    }

    Ok(())
}

#[cfg(test)]
#[path = "body.test.rs"]
mod body_test;
