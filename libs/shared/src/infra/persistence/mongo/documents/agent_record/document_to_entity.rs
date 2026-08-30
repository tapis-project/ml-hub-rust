use crate::domain::entities::agent_record as entities;
use crate::infra::persistence::mongo::documents::agent_record as documents;
use uuid::Uuid;

impl TryFrom<documents::AgentRecord> for entities::AgentRecord {
    type Error = entities::AgentRecordError;

    fn try_from(value: documents::AgentRecord) -> Result<Self, Self::Error> {
        let skills = value
            .skills
            .into_iter()
            .map(entities::AgentSkill::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        entities::AgentRecord::reconstitute(entities::ReconstituteAgentRecordProps {
            id: Uuid::from_bytes(value.id.bytes()),
            name: value.name,
            tenant_id: value.tenant_id,
            owner: value.owner,
            description: value.description,
            interfaces: value
                .interfaces
                .into_iter()
                .map(entities::AgentInterface::from)
                .collect(),
            capabilities: entities::Capabilities::from(value.capabilities),
            provider: value.provider.map(entities::AgentProvider::from),
            version: value.version,
            artifact_locators: value
                .artifact_locators
                .into_iter()
                .map(entities::ArtifactLocator::from)
                .collect(),
            default_input_modes: value
                .default_input_modes
                .into_iter()
                .map(|io_mode| entities::IoMode::new(&io_mode))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| {
                    entities::AgentRecordError::DataIntegrityError(format!(
                        "Agent record contains invalid default input modes: {error}"
                    ))
                })?,
            default_output_modes: value
                .default_output_modes
                .into_iter()
                .map(|io_mode| entities::IoMode::new(&io_mode))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| {
                    entities::AgentRecordError::DataIntegrityError(format!(
                        "Agent record contains invalid default output modes: {error}"
                    ))
                })?,
            skills,
            tags: value.tags,
            icon_url: value.icon_url,
            documentation_url: value.documentation_url,
            visibility: value.visibility.into(),
        })
    }
}

impl From<documents::AgentInterface> for entities::AgentInterface {
    fn from(value: documents::AgentInterface) -> Self {
        Self::new(
            value.name,
            value.description,
            value.protocol.into(),
            value.message_binding.map(Into::into),
            value.liveness_probe_config.map(Into::into),
        )
    }
}

impl From<documents::LivenessProbeConfiguration> for entities::LivenessProbeConfiguration {
    fn from(value: documents::LivenessProbeConfiguration) -> Self {
        match value {
            documents::LivenessProbeConfiguration::RestHttp {
                route,
                interval_seconds,
                timeout_seconds,
                missed_heartbeat_threshold,
                initial_delay_seconds,
            } => Self::RestHttp {
                route,
                interval_seconds,
                timeout_seconds,
                missed_heartbeat_threshold,
                initial_delay_seconds,
            },
        }
    }
}

impl From<documents::Protocol> for entities::Protocol {
    fn from(value: documents::Protocol) -> Self {
        match value {
            documents::Protocol::RestHttp => Self::RestHttp,
            documents::Protocol::Rpc => Self::Rpc,
            documents::Protocol::Stdio => Self::Stdio,
        }
    }
}

impl From<documents::MessageBinding> for entities::MessageBinding {
    fn from(value: documents::MessageBinding) -> Self {
        match value {
            documents::MessageBinding::HttpJson => Self::HttpJson,
            documents::MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            documents::MessageBinding::Grpc => Self::Grpc,
        }
    }
}

impl From<documents::Capabilities> for entities::Capabilities {
    fn from(value: documents::Capabilities) -> Self {
        Self::new(value.streaming, value.push_notifications)
    }
}

impl From<documents::AgentProvider> for entities::AgentProvider {
    fn from(value: documents::AgentProvider) -> Self {
        Self::new(value.organization, value.url)
    }
}

impl From<documents::ArtifactLocator> for entities::ArtifactLocator {
    fn from(value: documents::ArtifactLocator) -> Self {
        Self::new(value.artifact_type.into(), value.url)
    }
}

impl From<documents::AgentArtifactType> for entities::AgentArtifactType {
    fn from(value: documents::AgentArtifactType) -> Self {
        match value {
            documents::AgentArtifactType::Binary => Self::Binary,
            documents::AgentArtifactType::DockerImage => Self::DockerImage,
            documents::AgentArtifactType::HelmChart => Self::HelmChart,
            documents::AgentArtifactType::PythonPackage => Self::PythonPackage,
            documents::AgentArtifactType::SourceCode => Self::SourceCode,
            documents::AgentArtifactType::Unspecified => Self::Unspecified,
        }
    }
}

impl TryFrom<documents::AgentSkill> for entities::AgentSkill {
    type Error = entities::AgentRecordError;

    fn try_from(value: documents::AgentSkill) -> Result<Self, Self::Error> {
        entities::AgentSkill::reconstitute(entities::ReconstituteAgentSkillProps {
            id: value.id,
            name: value.name,
            description: value.description,
            tags: value.tags,
            examples: value.examples,
            input_modes: value
                .input_modes
                .map(|input_modes| {
                    input_modes
                        .into_iter()
                        .map(|io_mode| entities::IoMode::new(&io_mode))
                        .collect()
                })
                .transpose()
                .map_err(|error| {
                    entities::AgentRecordError::DataIntegrityError(format!(
                        "Agent skill contains invalid input modes: {error}"
                    ))
                })?,
            output_modes: value
                .output_modes
                .map(|output_modes| {
                    output_modes
                        .into_iter()
                        .map(|io_mode| entities::IoMode::new(&io_mode))
                        .collect()
                })
                .transpose()
                .map_err(|error| {
                    entities::AgentRecordError::DataIntegrityError(format!(
                        "Agent skill contains invalid output modes: {error}"
                    ))
                })?,
        })
        .map_err(|error| entities::AgentRecordError::DataIntegrityError(error.to_string()))
    }
}
