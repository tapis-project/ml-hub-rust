use crate::domain::entities::agent_record as entities;
use crate::infra::persistence::mongo::documents::agent_record as documents;
use crate::infra::persistence::mongo::documents::visibility::Visibility as DocumentVisibility;
use mongodb::bson::Uuid;

impl From<&entities::AgentRecord> for documents::AgentRecord {
    fn from(value: &entities::AgentRecord) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(*value.id().as_bytes()),
            name: value.name().clone(),
            tenant_id: value.tenant_id().clone(),
            owner: value.owner().clone(),
            description: value.description().clone(),
            interfaces: value
                .interfaces()
                .iter()
                .cloned()
                .map(documents::AgentInterface::from)
                .collect(),
            capabilities: documents::Capabilities {
                streaming: value.supports_streaming(),
                push_notifications: value.supports_push_notifications(),
            },
            provider: match (value.provider_organization(), value.provider_url()) {
                (Some(organization), Some(url)) => Some(documents::AgentProvider {
                    organization: organization.into(),
                    url: url.into(),
                }),
                _ => None,
            },
            version: value.version().to_owned(),
            artifact_locators: value
                .artifact_locators()
                .iter()
                .cloned()
                .map(documents::ArtifactLocator::from)
                .collect(),
            default_input_modes: value
                .default_input_modes()
                .iter()
                .map(|io_mode| io_mode.as_str().to_owned())
                .collect(),
            default_output_modes: value
                .default_output_modes()
                .iter()
                .map(|io_mode| io_mode.as_str().to_owned())
                .collect(),
            skills: value
                .skills()
                .iter()
                .cloned()
                .map(documents::AgentSkill::from)
                .collect(),
            tags: value
                .tags()
                .iter()
                .map(|tag| tag.as_str().to_owned())
                .collect(),
            icon_url: value.icon_url().map(str::to_owned),
            documentation_url: value.documentation_url().map(str::to_owned),
            visibility: if value.is_public() {
                DocumentVisibility::Public
            } else {
                DocumentVisibility::Private
            },
        }
    }
}

impl From<entities::AgentInterface> for documents::AgentInterface {
    fn from(value: entities::AgentInterface) -> Self {
        Self {
            name: value.name().clone(),
            description: value.description().clone(),
            protocol: value.protocol().clone().into(),
            message_binding: value.message_binding().clone().map(Into::into),
            liveness_probe_config: value.liveness_probe_config().cloned().map(Into::into),
        }
    }
}

impl From<entities::LivenessProbeConfiguration> for documents::LivenessProbeConfiguration {
    fn from(value: entities::LivenessProbeConfiguration) -> Self {
        match value {
            entities::LivenessProbeConfiguration::RestHttp {
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

impl From<entities::Protocol> for documents::Protocol {
    fn from(value: entities::Protocol) -> Self {
        match value {
            entities::Protocol::RestHttp => Self::RestHttp,
            entities::Protocol::Rpc => Self::Rpc,
            entities::Protocol::Stdio => Self::Stdio,
        }
    }
}

impl From<entities::MessageBinding> for documents::MessageBinding {
    fn from(value: entities::MessageBinding) -> Self {
        match value {
            entities::MessageBinding::HttpJson => Self::HttpJson,
            entities::MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            entities::MessageBinding::Grpc => Self::Grpc,
        }
    }
}

impl From<entities::ArtifactLocator> for documents::ArtifactLocator {
    fn from(value: entities::ArtifactLocator) -> Self {
        Self {
            artifact_type: value.artifact_type().clone().into(),
            url: value.url().into(),
        }
    }
}

impl From<entities::AgentArtifactType> for documents::AgentArtifactType {
    fn from(value: entities::AgentArtifactType) -> Self {
        match value {
            entities::AgentArtifactType::Binary => Self::Binary,
            entities::AgentArtifactType::DockerImage => Self::DockerImage,
            entities::AgentArtifactType::HelmChart => Self::HelmChart,
            entities::AgentArtifactType::PythonPackage => Self::PythonPackage,
            entities::AgentArtifactType::SourceCode => Self::SourceCode,
            entities::AgentArtifactType::Unspecified => Self::Unspecified,
        }
    }
}

impl From<entities::AgentSkill> for documents::AgentSkill {
    fn from(value: entities::AgentSkill) -> Self {
        Self {
            id: value.id().into(),
            name: value.name().into(),
            description: value.description().into(),
            tags: value.tags().iter().cloned().collect(),
            examples: value.examples().to_vec(),
            input_modes: value.input_modes().map(|input_modes| {
                input_modes
                    .iter()
                    .map(|io_mode| io_mode.as_str().to_owned())
                    .collect()
            }),
            output_modes: value.output_modes().map(|output_modes| {
                output_modes
                    .iter()
                    .map(|io_mode| io_mode.as_str().to_owned())
                    .collect()
            }),
        }
    }
}
