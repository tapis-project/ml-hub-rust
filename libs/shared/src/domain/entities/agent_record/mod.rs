use std::collections::HashSet;

use nonempty::NonEmpty;
use semver::Version;
use thiserror::Error;
use uuid::Uuid;

use crate::impl_urn_generator;
use crate::shared_kernel::enums::Visibility;

#[derive(Clone, Debug)]
pub struct AgentRecord {
    id: Uuid,
    name: String,
    tenant_id: String,
    owner: String,
    description: String,
    interfaces: NonEmpty<AgentInterface>,
    capabilities: Capabilities,
    provider: Option<AgentProvider>,
    version: String,
    artifact_locators: Vec<ArtifactLocator>,
    skills: Vec<AgentSkill>,
    icon_url: Option<String>,
    documentation_url: Option<String>,
    visibility: Visibility,
}

impl_urn_generator!(AgentRecord, tenant_id, "agent_record", id);

impl AgentRecord {
    pub fn new(
        name: String,
        tenant_id: String,
        owner: String,
        description: String,
        interfaces: Vec<AgentInterface>,
        capabilities: Capabilities,
        provider: Option<AgentProvider>,
        version: String,
        artifact_locators: Vec<ArtifactLocator>,
        skills: Vec<AgentSkill>,
        icon_url: Option<String>,
        documentation_url: Option<String>,
        visibility: Visibility,
    ) -> Result<Self, AgentRecordError> {
        if Self::validate_version(&version).is_err() {
            return Err(AgentRecordError::InvalidVersion(version));
        }

        let interfaces = Self::interfaces_from_vec(interfaces)?;

        Self::ensure_unique_interface_names(&interfaces)
            .map_err(AgentRecordError::DuplicateAgentInterfaceIdentifier)?;

        Self::ensure_compatible_liveness_probe_configurations(&interfaces)
            .map_err(AgentRecordError::IncompatibleLivenessProbeConfiguration)?;

        Self::ensure_unique_skill_ids(&skills)
            .map_err(AgentRecordError::DuplicateAgentSkillIdentifier)?;

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            tenant_id,
            owner,
            description,
            interfaces,
            capabilities,
            provider,
            version,
            artifact_locators,
            skills,
            icon_url,
            documentation_url,
            visibility,
        })
    }

    pub fn reconstitute(props: ReconstituteAgentRecordProps) -> Result<Self, AgentRecordError> {
        if Self::validate_version(&props.version).is_err() {
            return Err(AgentRecordError::DataIntegrityError(format!(
                "Agent record contains an invalid semantic version: {}",
                props.version
            )));
        }

        let interfaces = Self::interfaces_from_vec(props.interfaces)?;

        Self::ensure_unique_interface_names(&interfaces).map_err(|duplicate_name| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains interfaces with duplicate names. Duplicate found: {duplicate_name}"
            ))
        })?;

        Self::ensure_compatible_liveness_probe_configurations(&interfaces).map_err(
            |interface_name| {
                AgentRecordError::DataIntegrityError(format!(
                    "Agent record contains an incompatible liveness probe configuration for interface: {interface_name}"
                ))
            },
        )?;

        Self::ensure_unique_skill_ids(&props.skills).map_err(|duplicate_id| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains skills with duplicate IDs. Duplicate found: {duplicate_id}"
            ))
        })?;

        Ok(Self {
            id: props.id,
            name: props.name,
            tenant_id: props.tenant_id,
            owner: props.owner,
            description: props.description,
            interfaces,
            capabilities: props.capabilities,
            provider: props.provider,
            version: props.version,
            artifact_locators: props.artifact_locators,
            skills: props.skills,
            icon_url: props.icon_url,
            documentation_url: props.documentation_url,
            visibility: props.visibility,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn tenant_id(&self) -> &String {
        &self.tenant_id
    }

    pub fn owner(&self) -> &String {
        &self.owner
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn interfaces(&self) -> &NonEmpty<AgentInterface> {
        &self.interfaces
    }

    pub fn supports_streaming(&self) -> bool {
        self.capabilities.supports_streaming()
    }

    pub fn supports_push_notifications(&self) -> bool {
        self.capabilities.supports_push_notifications()
    }

    pub fn provider_organization(&self) -> Option<&str> {
        self.provider
            .as_ref()
            .map(|provider| provider.organization())
    }

    pub fn provider_url(&self) -> Option<&str> {
        self.provider.as_ref().map(|provider| provider.url())
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn artifact_locators(&self) -> &[ArtifactLocator] {
        &self.artifact_locators
    }

    pub fn skills(&self) -> &[AgentSkill] {
        &self.skills
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn documentation_url(&self) -> Option<&str> {
        self.documentation_url.as_deref()
    }

    pub fn is_public(&self) -> bool {
        matches!(self.visibility, Visibility::Public)
    }

    fn interfaces_from_vec(
        interfaces: Vec<AgentInterface>,
    ) -> Result<NonEmpty<AgentInterface>, AgentRecordError> {
        NonEmpty::from_vec(interfaces).ok_or_else(|| {
            AgentRecordError::DataIntegrityError(
                "Agent record MUST have at least one supported interface".into(),
            )
        })
    }

    fn ensure_unique_interface_names(interfaces: &NonEmpty<AgentInterface>) -> Result<(), String> {
        let mut names = HashSet::new();

        for interface in interfaces {
            if !names.insert(interface.name()) {
                return Err(interface.name().clone());
            }
        }

        Ok(())
    }

    fn ensure_unique_skill_ids(skills: &[AgentSkill]) -> Result<(), String> {
        let mut ids = HashSet::new();

        for skill in skills {
            if !ids.insert(skill.id()) {
                return Err(skill.id().into());
            }
        }

        Ok(())
    }

    fn ensure_compatible_liveness_probe_configurations(
        interfaces: &NonEmpty<AgentInterface>,
    ) -> Result<(), String> {
        for interface in interfaces {
            if interface.liveness_probe_config().is_some()
                && !matches!(interface.protocol(), Protocol::RestHttp)
            {
                return Err(interface.name().clone());
            }
        }

        Ok(())
    }

    fn validate_version(version: &str) -> Result<(), semver::Error> {
        Version::parse(version).map(|_| ())
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentRecordProps {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: String,
    pub interfaces: Vec<AgentInterface>,
    pub capabilities: Capabilities,
    pub provider: Option<AgentProvider>,
    pub artifact_locators: Vec<ArtifactLocator>,
    pub skills: Vec<AgentSkill>,
    pub icon_url: Option<String>,
    pub documentation_url: Option<String>,
    pub visibility: Visibility,
}

/// A discrete agent capability used for discovery, routing, compliance auditing, and coarse-grained classification.
#[derive(Clone, Debug)]
pub struct AgentSkill {
    id: String,
    name: String,
    description: String,
    tags: NonEmpty<String>,
    examples: Vec<String>,
}

impl AgentSkill {
    pub fn new(
        id: String,
        name: String,
        description: String,
        tags: Vec<String>,
        examples: Vec<String>,
    ) -> Result<Self, AgentSkillError> {
        if !is_lower_kebab_case(&id) {
            return Err(AgentSkillError::InvalidIdentifier(id));
        }

        let tags = NonEmpty::from_vec(tags).ok_or(AgentSkillError::EmptyTags)?;

        Ok(Self {
            id,
            name,
            description,
            tags,
            examples,
        })
    }

    pub fn reconstitute(props: ReconstituteAgentSkillProps) -> Result<Self, AgentSkillError> {
        Self::new(
            props.id,
            props.name,
            props.description,
            props.tags,
            props.examples,
        )
        .map_err(|error| AgentSkillError::DataIntegrityError(error.to_string()))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn tags(&self) -> &NonEmpty<String> {
        &self.tags
    }

    pub fn examples(&self) -> &[String] {
        &self.examples
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentSkillProps {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
}

fn is_lower_kebab_case(value: &str) -> bool {
    !value.is_empty()
        && value.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        })
}

#[derive(Clone, Debug)]
pub struct ArtifactLocator {
    artifact_type: AgentArtifactType,
    url: String,
}

impl ArtifactLocator {
    pub fn new(artifact_type: AgentArtifactType, url: String) -> Self {
        Self { artifact_type, url }
    }

    pub fn artifact_type(&self) -> &AgentArtifactType {
        &self.artifact_type
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Clone, Debug)]
pub enum AgentArtifactType {
    Binary,
    DockerImage,
    HelmChart,
    PythonPackage,
    SourceCode,
    Unspecified,
}

#[derive(Clone, Debug)]
pub struct AgentProvider {
    organization: String,
    url: String,
}

impl AgentProvider {
    pub fn new(organization: String, url: String) -> Self {
        Self { organization, url }
    }

    pub fn organization(&self) -> &str {
        &self.organization
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Clone, Debug)]
pub struct Capabilities {
    streaming: bool,
    push_notifications: bool,
}

impl Capabilities {
    pub fn new(streaming: bool, push_notifications: bool) -> Self {
        Self {
            streaming,
            push_notifications,
        }
    }

    pub fn supports_streaming(&self) -> bool {
        self.streaming
    }

    pub fn supports_push_notifications(&self) -> bool {
        self.push_notifications
    }
}

#[derive(Clone, Debug)]
pub struct AgentInterface {
    name: String,
    description: Option<String>,
    protocol: Protocol,
    message_binding: Option<MessageBinding>,
    liveness_probe_config: Option<LivenessProbeConfiguration>,
}

impl AgentInterface {
    pub fn new(
        name: String,
        description: Option<String>,
        protocol: Protocol,
        message_binding: Option<MessageBinding>,
        liveness_probe_config: Option<LivenessProbeConfiguration>,
    ) -> Self {
        Self {
            name,
            description,
            protocol,
            message_binding,
            liveness_probe_config,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn message_binding(&self) -> &Option<MessageBinding> {
        &self.message_binding
    }

    pub fn liveness_probe_config(&self) -> Option<&LivenessProbeConfiguration> {
        self.liveness_probe_config.as_ref()
    }
}

#[derive(Clone, Debug)]
pub enum Protocol {
    RestHttp,
    Rpc,
    Stdio,
}

#[derive(Clone, Debug)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[derive(Debug, Clone)]
pub enum LivenessProbeConfiguration {
    RestHttp {
        // Serves as the default route to query for a heartbeat.
        // Anything other than a 200 at this endpoint results in a
        // missed heartbeat
        route: String,

        /// How frequently the platform should initiate or expect a heartbeat check
        interval_seconds: u32,

        /// Maximum time allowed for the agent to respond to the heartbeat request
        timeout_seconds: u32,

        /// Number of consecutive missed heartbeats required to mark the instance as Dead
        missed_heartbeat_threshold: u16,

        /// Time to wait after the agent boots up before starting heartbeat evaluations
        /// (Crucial for allowing LLM weights to load or databases to initialize)
        initial_delay_seconds: u32,
    },
}

#[derive(Debug, Error, Clone)]
pub enum AgentRecordError {
    #[error("Duplicate agent interface identifier: {0}")]
    DuplicateAgentInterfaceIdentifier(String),

    #[error("Duplicate agent skill identifier: {0}")]
    DuplicateAgentSkillIdentifier(String),

    #[error("Incompatible liveness probe configuration for agent interface: {0}")]
    IncompatibleLivenessProbeConfiguration(String),

    #[error("Invalid agent record version: {0}")]
    InvalidVersion(String),

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[derive(Debug, Error, Clone)]
pub enum AgentSkillError {
    #[error("Invalid agent skill identifier: {0}")]
    InvalidIdentifier(String),

    #[error("Agent skill MUST have at least one tag")]
    EmptyTags,

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
pub mod test_fixtures;

#[cfg(test)]
#[path = "agent_record.test.rs"]
mod agent_record_test;
