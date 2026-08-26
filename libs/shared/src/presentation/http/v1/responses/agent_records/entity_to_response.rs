use crate::domain::entities::agent_record as entities;
use crate::presentation::http::v1::responses::agent_records as responses;
use crate::presentation::http::v1::responses::visibility::Visibility as ResponseVisibility;

impl From<entities::AgentRecord> for responses::AgentRecord {
    fn from(value: entities::AgentRecord) -> Self {
        Self {
            id: *value.id(),
            name: value.name().clone(),
            tenant_id: value.tenant_id().clone(),
            owner: value.owner().clone(),
            description: value.description().clone(),
            rest_http_interfaces: value
                .interfaces()
                .iter()
                .filter(|interface| matches!(interface.protocol(), entities::Protocol::RestHttp))
                .cloned()
                .map(responses::RestHttpAgentInterface::from)
                .collect(),
            rpc_interfaces: value
                .interfaces()
                .iter()
                .filter(|interface| matches!(interface.protocol(), entities::Protocol::Rpc))
                .cloned()
                .map(responses::RpcAgentInterface::from)
                .collect(),
            stdio_interfaces: value
                .interfaces()
                .iter()
                .filter(|interface| matches!(interface.protocol(), entities::Protocol::Stdio))
                .cloned()
                .map(responses::StdioAgentInterface::from)
                .collect(),
            capabilities: responses::Capabilities {
                streaming: value.supports_streaming(),
                push_notifications: value.supports_push_notifications(),
            },
            provider: match (value.provider_organization(), value.provider_url()) {
                (Some(organization), Some(url)) => Some(responses::AgentProvider {
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
                .map(responses::ArtifactLocator::from)
                .collect(),
            skills: value
                .skills()
                .iter()
                .cloned()
                .map(responses::AgentSkill::from)
                .collect(),
            tags: value
                .tags()
                .iter()
                .map(|tag| tag.as_str().to_owned())
                .collect(),
            icon_url: value.icon_url().map(str::to_owned),
            documentation_url: value.documentation_url().map(str::to_owned),
            visibility: if value.is_public() {
                ResponseVisibility::Public
            } else {
                ResponseVisibility::Private
            },
        }
    }
}

impl From<entities::AgentSkill> for responses::AgentSkill {
    fn from(value: entities::AgentSkill) -> Self {
        Self {
            id: value.id().into(),
            name: value.name().into(),
            description: value.description().into(),
            tags: value.tags().iter().cloned().collect(),
            examples: value.examples().to_vec(),
        }
    }
}

impl From<entities::ArtifactLocator> for responses::ArtifactLocator {
    fn from(value: entities::ArtifactLocator) -> Self {
        Self {
            artifact_type: value.artifact_type().clone().into(),
            url: value.url().into(),
        }
    }
}

impl From<entities::AgentArtifactType> for responses::AgentArtifactType {
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

impl From<entities::AgentInterface> for responses::RestHttpAgentInterface {
    fn from(value: entities::AgentInterface) -> Self {
        Self {
            name: value.name().clone(),
            description: value.description().clone(),
            message_binding: value.message_binding().clone().map(Into::into),
            liveness_probe_config: value.liveness_probe_config().cloned().map(Into::into),
        }
    }
}

impl From<entities::AgentInterface> for responses::RpcAgentInterface {
    fn from(value: entities::AgentInterface) -> Self {
        Self {
            name: value.name().clone(),
            description: value.description().clone(),
            message_binding: value.message_binding().clone().map(Into::into),
        }
    }
}

impl From<entities::AgentInterface> for responses::StdioAgentInterface {
    fn from(value: entities::AgentInterface) -> Self {
        Self {
            name: value.name().clone(),
            description: value.description().clone(),
            message_binding: value.message_binding().clone().map(Into::into),
        }
    }
}

impl From<entities::LivenessProbeConfiguration> for responses::RestHttpLivenessProbe {
    fn from(value: entities::LivenessProbeConfiguration) -> Self {
        match value {
            entities::LivenessProbeConfiguration::RestHttp {
                route,
                interval_seconds,
                timeout_seconds,
                missed_heartbeat_threshold,
                initial_delay_seconds,
            } => Self {
                route,
                interval_seconds,
                timeout_seconds,
                missed_heartbeat_threshold,
                initial_delay_seconds,
            },
        }
    }
}

impl From<entities::MessageBinding> for responses::MessageBinding {
    fn from(value: entities::MessageBinding) -> Self {
        match value {
            entities::MessageBinding::HttpJson => Self::HttpJson,
            entities::MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            entities::MessageBinding::Grpc => Self::Grpc,
        }
    }
}
