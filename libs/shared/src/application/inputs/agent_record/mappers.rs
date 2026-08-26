use crate::application::inputs::agent_record as inputs;
use crate::domain::entities::agent_record as entities;
use crate::presentation::http::v1::requests::create_agent_record::body as requests;
use crate::shared_kernel::enums::Visibility as DomainVisibility;

impl From<requests::CreateAgentRecordBody> for inputs::CreateAgentRecordInput {
    fn from(value: requests::CreateAgentRecordBody) -> Self {
        Self {
            name: value.name,
            description: value.description,
            interfaces: value
                .rest_http_interfaces
                .into_iter()
                .map(Into::into)
                .chain(value.rpc_interfaces.into_iter().map(Into::into))
                .chain(value.stdio_interfaces.into_iter().map(Into::into))
                .collect(),
            capabilities: value.capabilities.into(),
            provider: value.provider.map(Into::into),
            version: value.version,
            artifact_locators: value
                .artifact_locators
                .into_iter()
                .map(Into::into)
                .collect(),
            skills: value.skills.into_iter().map(Into::into).collect(),
            tags: value.tags,
            icon_url: value.icon_url,
            documentation_url: value.documentation_url,
            visibility: value.visibility.into(),
        }
    }
}

impl From<requests::Visibility> for inputs::VisibilityInput {
    fn from(value: requests::Visibility) -> Self {
        match value {
            requests::Visibility::Public => Self::Public,
            requests::Visibility::Private => Self::Private,
        }
    }
}

impl From<requests::RestHttpAgentInterface> for inputs::AgentInterfaceInput {
    fn from(value: requests::RestHttpAgentInterface) -> Self {
        Self {
            name: value.name,
            description: value.description,
            protocol: inputs::ProtocolInput::RestHttp,
            message_binding: value.message_binding.map(Into::into),
            liveness_probe_config: value.liveness_probe_config.map(Into::into),
        }
    }
}

impl From<requests::RpcAgentInterface> for inputs::AgentInterfaceInput {
    fn from(value: requests::RpcAgentInterface) -> Self {
        Self {
            name: value.name,
            description: value.description,
            protocol: inputs::ProtocolInput::Rpc,
            message_binding: value.message_binding.map(Into::into),
            liveness_probe_config: None,
        }
    }
}

impl From<requests::StdioAgentInterface> for inputs::AgentInterfaceInput {
    fn from(value: requests::StdioAgentInterface) -> Self {
        Self {
            name: value.name,
            description: value.description,
            protocol: inputs::ProtocolInput::Stdio,
            message_binding: value.message_binding.map(Into::into),
            liveness_probe_config: None,
        }
    }
}

impl From<requests::RestHttpLivenessProbe> for inputs::LivenessProbeConfigurationInput {
    fn from(value: requests::RestHttpLivenessProbe) -> Self {
        Self::RestHttp {
            route: value.route,
            interval_seconds: value.interval_seconds,
            timeout_seconds: value.timeout_seconds,
            missed_heartbeat_threshold: value.missed_heartbeat_threshold,
            initial_delay_seconds: value.initial_delay_seconds,
        }
    }
}

impl From<requests::MessageBinding> for inputs::MessageBindingInput {
    fn from(value: requests::MessageBinding) -> Self {
        match value {
            requests::MessageBinding::HttpJson => Self::HttpJson,
            requests::MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            requests::MessageBinding::Grpc => Self::Grpc,
        }
    }
}

impl From<requests::Capabilities> for inputs::CapabilitiesInput {
    fn from(value: requests::Capabilities) -> Self {
        Self {
            streaming: value.streaming,
            push_notifications: value.push_notifications,
        }
    }
}

impl From<requests::AgentProvider> for inputs::AgentProviderInput {
    fn from(value: requests::AgentProvider) -> Self {
        Self {
            organization: value.organization,
            url: value.url,
        }
    }
}

impl From<requests::ArtifactLocator> for inputs::ArtifactLocatorInput {
    fn from(value: requests::ArtifactLocator) -> Self {
        Self {
            artifact_type: value.artifact_type.into(),
            url: value.url,
        }
    }
}

impl From<requests::AgentArtifactType> for inputs::AgentArtifactTypeInput {
    fn from(value: requests::AgentArtifactType) -> Self {
        match value {
            requests::AgentArtifactType::Binary => Self::Binary,
            requests::AgentArtifactType::DockerImage => Self::DockerImage,
            requests::AgentArtifactType::HelmChart => Self::HelmChart,
            requests::AgentArtifactType::PythonPackage => Self::PythonPackage,
            requests::AgentArtifactType::SourceCode => Self::SourceCode,
            requests::AgentArtifactType::Unspecified => Self::Unspecified,
        }
    }
}

impl From<requests::AgentSkill> for inputs::AgentSkillInput {
    fn from(value: requests::AgentSkill) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            tags: value.tags,
            examples: value.examples,
        }
    }
}

impl From<inputs::AgentInterfaceInput> for entities::AgentInterface {
    fn from(value: inputs::AgentInterfaceInput) -> Self {
        Self::new(
            value.name,
            value.description,
            value.protocol.into(),
            value.message_binding.map(Into::into),
            value.liveness_probe_config.map(Into::into),
        )
    }
}

impl From<inputs::LivenessProbeConfigurationInput> for entities::LivenessProbeConfiguration {
    fn from(value: inputs::LivenessProbeConfigurationInput) -> Self {
        match value {
            inputs::LivenessProbeConfigurationInput::RestHttp {
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

impl From<inputs::ProtocolInput> for entities::Protocol {
    fn from(value: inputs::ProtocolInput) -> Self {
        match value {
            inputs::ProtocolInput::RestHttp => Self::RestHttp,
            inputs::ProtocolInput::Rpc => Self::Rpc,
            inputs::ProtocolInput::Stdio => Self::Stdio,
        }
    }
}

impl From<inputs::MessageBindingInput> for entities::MessageBinding {
    fn from(value: inputs::MessageBindingInput) -> Self {
        match value {
            inputs::MessageBindingInput::HttpJson => Self::HttpJson,
            inputs::MessageBindingInput::JsonRpc2_0 => Self::JsonRpc2_0,
            inputs::MessageBindingInput::Grpc => Self::Grpc,
        }
    }
}

impl From<inputs::CapabilitiesInput> for entities::Capabilities {
    fn from(value: inputs::CapabilitiesInput) -> Self {
        Self::new(value.streaming, value.push_notifications)
    }
}

impl From<inputs::AgentProviderInput> for entities::AgentProvider {
    fn from(value: inputs::AgentProviderInput) -> Self {
        Self::new(value.organization, value.url)
    }
}

impl From<inputs::ArtifactLocatorInput> for entities::ArtifactLocator {
    fn from(value: inputs::ArtifactLocatorInput) -> Self {
        Self::new(value.artifact_type.into(), value.url)
    }
}

impl From<inputs::AgentArtifactTypeInput> for entities::AgentArtifactType {
    fn from(value: inputs::AgentArtifactTypeInput) -> Self {
        match value {
            inputs::AgentArtifactTypeInput::Binary => Self::Binary,
            inputs::AgentArtifactTypeInput::DockerImage => Self::DockerImage,
            inputs::AgentArtifactTypeInput::HelmChart => Self::HelmChart,
            inputs::AgentArtifactTypeInput::PythonPackage => Self::PythonPackage,
            inputs::AgentArtifactTypeInput::SourceCode => Self::SourceCode,
            inputs::AgentArtifactTypeInput::Unspecified => Self::Unspecified,
        }
    }
}

impl TryFrom<inputs::AgentSkillInput> for entities::AgentSkill {
    type Error = entities::AgentSkillError;
    fn try_from(value: inputs::AgentSkillInput) -> Result<Self, Self::Error> {
        Self::new(
            value.id,
            value.name,
            value.description,
            value.tags,
            value.examples,
        )
    }
}

impl From<inputs::VisibilityInput> for DomainVisibility {
    fn from(value: inputs::VisibilityInput) -> Self {
        match value {
            inputs::VisibilityInput::Public => Self::Public,
            inputs::VisibilityInput::Private => Self::Private,
        }
    }
}
