use std::collections::HashSet;

use semver::Version;
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_interface_collections"))]
pub struct CreateAgentRecordBody {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: String,
    #[serde(default)]
    #[validate(nested)]
    pub rest_http_interfaces: Vec<RestHttpAgentInterface>,
    #[serde(default)]
    #[validate(nested)]
    pub rpc_interfaces: Vec<RpcAgentInterface>,
    #[serde(default)]
    #[validate(nested)]
    pub stdio_interfaces: Vec<StdioAgentInterface>,
    pub capabilities: Capabilities,
    #[validate(nested)]
    pub provider: Option<AgentProvider>,
    #[validate(length(min = 1), custom(function = "validate_semver"))]
    pub version: String,
    #[serde(default, deserialize_with = "deserialize_null_to_empty")]
    #[schema(nullable, default = json!([]))]
    #[validate(nested)]
    pub artifact_locators: Vec<ArtifactLocator>,
    #[serde(default)]
    #[validate(nested, custom(function = "validate_unique_skill_ids"))]
    pub skills: Vec<AgentSkill>,
    #[validate(url)]
    pub icon_url: Option<String>,
    #[validate(url)]
    pub documentation_url: Option<String>,
    #[serde(default)]
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
pub struct AgentProvider {
    #[validate(length(min = 1))]
    pub organization: String,
    #[validate(url)]
    pub url: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
pub struct ArtifactLocator {
    pub artifact_type: AgentArtifactType,
    #[validate(url)]
    pub url: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
pub struct AgentSkill {
    #[validate(custom(function = "validate_lower_kebab_case"))]
    pub id: String,
    pub name: String,
    pub description: String,
    #[validate(length(min = 1))]
    pub tags: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum AgentArtifactType {
    Binary,
    DockerImage,
    HelmChart,
    PythonPackage,
    SourceCode,
    Unspecified,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct Capabilities {
    pub streaming: bool,
    pub push_notifications: bool,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RestHttpAgentInterface {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    pub message_binding: Option<MessageBinding>,
    pub liveness_probe_config: Option<RestHttpLivenessProbe>,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RpcAgentInterface {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    pub message_binding: Option<MessageBinding>,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct StdioAgentInterface {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    pub message_binding: Option<MessageBinding>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct RestHttpLivenessProbe {
    pub route: String,
    pub interval_seconds: u32,
    pub timeout_seconds: u32,
    pub missed_heartbeat_threshold: u16,
    pub initial_delay_seconds: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum Visibility {
    Public,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}

fn validate_interface_collections(
    body: &CreateAgentRecordBody,
) -> Result<(), ValidationError> {
    let mut names = HashSet::new();

    for interface in body
        .rest_http_interfaces
        .iter()
        .map(|interface| &interface.name)
        .chain(body.rpc_interfaces.iter().map(|interface| &interface.name))
        .chain(body.stdio_interfaces.iter().map(|interface| &interface.name))
    {
        if !names.insert(interface) {
            return Err(ValidationError::new("duplicate_agent_interface_name"));
        }
    }

    if names.is_empty() {
        return Err(ValidationError::new("missing_agent_interfaces"));
    }

    Ok(())
}

fn validate_unique_skill_ids(skills: &Vec<AgentSkill>) -> Result<(), ValidationError> {
    let mut ids = HashSet::new();

    for skill in skills {
        if !ids.insert(&skill.id) {
            return Err(ValidationError::new("duplicate_agent_skill_id"));
        }
    }

    Ok(())
}

fn deserialize_null_to_empty<'de, D>(deserializer: D) -> Result<Vec<ArtifactLocator>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<Vec<ArtifactLocator>>::deserialize(deserializer)? {
        Some(artifact_locators) => Ok(artifact_locators),
        None => Ok(Vec::new()),
    }
}

fn validate_semver(version: &str) -> Result<(), ValidationError> {
    Version::parse(version)
        .map(|_| ())
        .map_err(|_| ValidationError::new("semver"))
}

fn validate_lower_kebab_case(value: &str) -> Result<(), ValidationError> {
    let is_valid = !value.is_empty()
        && value.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        });

    if is_valid {
        Ok(())
    } else {
        Err(ValidationError::new("lower_kebab_case"))
    }
}

#[cfg(test)]
#[path = "body.test.rs"]
mod body_test;
