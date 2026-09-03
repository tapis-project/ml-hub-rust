use std::collections::HashSet;

use nonempty::NonEmpty;
use thiserror::Error;
use uuid::Uuid;

use crate::impl_urn_generator;
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::{SemanticVersion, Tags, TagsError};

#[derive(Clone, Debug)]
pub struct AgentRecord {
    id: Uuid,
    name: String,
    version: SemanticVersion,
    tenant_id: String,
    owner: String,
    description: String,
    interfaces: NonEmpty<AgentInterface>,
    capabilities: Capabilities,
    provider: Option<AgentProvider>,
    artifact_locators: Vec<ArtifactLocator>,
    default_input_modes: NonEmpty<IoMode>,
    default_output_modes: NonEmpty<IoMode>,
    skills: Vec<AgentSkill>,
    tags: Tags,
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
        default_input_modes: Vec<IoMode>,
        default_output_modes: Vec<IoMode>,
        skills: Vec<AgentSkill>,
        tags: Vec<String>,
        icon_url: Option<String>,
        documentation_url: Option<String>,
        visibility: Visibility,
    ) -> Result<Self, AgentRecordError> {
        let version = SemanticVersion::new(version.clone())
            .map_err(|_| AgentRecordError::InvalidVersion(version))?;

        let interfaces = Self::interfaces_from_vec(interfaces)?;

        let default_input_modes = Self::default_input_modes_from_vec(default_input_modes)?;

        let default_output_modes = Self::default_output_modes_from_vec(default_output_modes)?;

        Self::ensure_unique_interface_names(&interfaces)
            .map_err(AgentRecordError::DuplicateAgentInterfaceIdentifier)?;

        Self::ensure_compatible_liveness_probe_configurations(&interfaces)
            .map_err(AgentRecordError::IncompatibleLivenessProbeConfiguration)?;

        Self::ensure_unique_skill_ids(&skills)
            .map_err(AgentRecordError::DuplicateAgentSkillIdentifier)?;

        let tags = Tags::new(tags).map_err(AgentRecordError::InvalidTags)?;

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
            default_input_modes,
            default_output_modes,
            skills,
            tags,
            icon_url,
            documentation_url,
            visibility,
        })
    }

    pub fn reconstitute(props: ReconstituteAgentRecordProps) -> Result<Self, AgentRecordError> {
        let version = SemanticVersion::reconstitute(props.version).map_err(|error| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains an invalid semantic version: {error}"
            ))
        })?;

        let interfaces = Self::interfaces_from_vec(props.interfaces)?;

        let default_input_modes =
            Self::reconstitute_default_input_modes(props.default_input_modes)?;

        let default_output_modes =
            Self::reconstitute_default_output_modes(props.default_output_modes)?;

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

        let tags = Tags::reconstitute(props.tags).map_err(|error| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains invalid tags: {error}"
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
            version,
            artifact_locators: props.artifact_locators,
            default_input_modes,
            default_output_modes,
            skills: props.skills,
            tags,
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

    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    pub fn artifact_locators(&self) -> &[ArtifactLocator] {
        &self.artifact_locators
    }

    pub fn default_input_modes(&self) -> &NonEmpty<IoMode> {
        &self.default_input_modes
    }

    pub fn default_output_modes(&self) -> &NonEmpty<IoMode> {
        &self.default_output_modes
    }

    pub fn skills(&self) -> &[AgentSkill] {
        &self.skills
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
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

    fn default_input_modes_from_vec(
        default_input_modes: Vec<IoMode>,
    ) -> Result<NonEmpty<IoMode>, AgentRecordError> {
        NonEmpty::from_vec(default_input_modes).ok_or(AgentRecordError::EmptyDefaultInputModes)
    }

    fn default_output_modes_from_vec(
        default_output_modes: Vec<IoMode>,
    ) -> Result<NonEmpty<IoMode>, AgentRecordError> {
        NonEmpty::from_vec(default_output_modes).ok_or(AgentRecordError::EmptyDefaultOutputModes)
    }

    fn reconstitute_default_input_modes(
        default_input_modes: Vec<IoMode>,
    ) -> Result<NonEmpty<IoMode>, AgentRecordError> {
        Self::default_input_modes_from_vec(default_input_modes).map_err(|error| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains invalid default input modes: {error}"
            ))
        })
    }

    fn reconstitute_default_output_modes(
        default_output_modes: Vec<IoMode>,
    ) -> Result<NonEmpty<IoMode>, AgentRecordError> {
        Self::default_output_modes_from_vec(default_output_modes).map_err(|error| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains invalid default output modes: {error}"
            ))
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
    pub default_input_modes: Vec<IoMode>,
    pub default_output_modes: Vec<IoMode>,
    pub skills: Vec<AgentSkill>,
    pub tags: Vec<String>,
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
    input_modes: Option<NonEmpty<IoMode>>,
    output_modes: Option<NonEmpty<IoMode>>,
}

impl AgentSkill {
    pub fn new(
        id: String,
        name: String,
        description: String,
        tags: Vec<String>,
        examples: Vec<String>,
    ) -> Result<Self, AgentSkillError> {
        Self::new_with_io_modes(id, name, description, tags, examples, None, None)
    }

    pub fn new_with_io_modes(
        id: String,
        name: String,
        description: String,
        tags: Vec<String>,
        examples: Vec<String>,
        input_modes: Option<Vec<IoMode>>,
        output_modes: Option<Vec<IoMode>>,
    ) -> Result<Self, AgentSkillError> {
        if !is_lower_kebab_case(&id) {
            return Err(AgentSkillError::InvalidIdentifier(id));
        }

        let tags = NonEmpty::from_vec(tags).ok_or(AgentSkillError::EmptyTags)?;

        let input_modes = Self::input_modes_from_vec(input_modes)?;

        let output_modes = Self::output_modes_from_vec(output_modes)?;

        Ok(Self {
            id,
            name,
            description,
            tags,
            examples,
            input_modes,
            output_modes,
        })
    }

    pub fn reconstitute(props: ReconstituteAgentSkillProps) -> Result<Self, AgentSkillError> {
        Self::new_with_io_modes(
            props.id,
            props.name,
            props.description,
            props.tags,
            props.examples,
            props.input_modes,
            props.output_modes,
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

    pub fn input_modes(&self) -> Option<&NonEmpty<IoMode>> {
        self.input_modes.as_ref()
    }

    pub fn output_modes(&self) -> Option<&NonEmpty<IoMode>> {
        self.output_modes.as_ref()
    }

    fn input_modes_from_vec(
        input_modes: Option<Vec<IoMode>>,
    ) -> Result<Option<NonEmpty<IoMode>>, AgentSkillError> {
        input_modes
            .map(|input_modes| {
                NonEmpty::from_vec(input_modes).ok_or(AgentSkillError::EmptyInputModes)
            })
            .transpose()
    }

    fn output_modes_from_vec(
        output_modes: Option<Vec<IoMode>>,
    ) -> Result<Option<NonEmpty<IoMode>>, AgentSkillError> {
        output_modes
            .map(|output_modes| {
                NonEmpty::from_vec(output_modes).ok_or(AgentSkillError::EmptyOutputModes)
            })
            .transpose()
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentSkillProps {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
    pub input_modes: Option<Vec<IoMode>>,
    pub output_modes: Option<Vec<IoMode>>,
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

    #[error("Invalid agent record tags: {0}")]
    InvalidTags(TagsError),

    #[error("Agent record MUST have at least one default input mode")]
    EmptyDefaultInputModes,

    #[error("Agent record MUST have at least one default output mode")]
    EmptyDefaultOutputModes,

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[derive(Debug, Error, Clone)]
pub enum AgentSkillError {
    #[error("Invalid agent skill identifier: {0}")]
    InvalidIdentifier(String),

    #[error("Agent skill MUST have at least one tag")]
    EmptyTags,

    #[error("Agent skill input modes MUST not be empty when supplied")]
    EmptyInputModes,

    #[error("Agent skill output modes MUST not be empty when supplied")]
    EmptyOutputModes,

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[derive(Debug, Clone, Error)]
pub enum IoModeError {
    #[error("Invalid I/O mode: '{0}'")]
    Invalid(String),
}

#[derive(Clone, Debug)]
pub struct IoMode(String);

impl IoMode {
    pub fn new(io_mode: &str) -> Result<Self, IoModeError> {
        Ok(Self(
            io_mode
                .parse::<mime::Mime>()
                .map_err(|_| IoModeError::Invalid(io_mode.into()))?
                .to_string(),
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
pub mod test_fixtures;

#[cfg(test)]
#[path = "agent_record.test.rs"]
mod agent_record_test;
