use crate::application::inputs::agent as inputs;
use crate::domain::entities::agent as entities;
use crate::domain::entities::agent_record as record_entities;
use crate::presentation::http::v1::requests::create_agent::body as requests;
use crate::presentation::http::v1::requests::create_agent_record::body::{
    MessageBinding, RestHttpLivenessProbe, Visibility as RequestVisibility,
};
use crate::shared_kernel::enums::Visibility;

impl From<requests::CreateAgentBody> for inputs::RegisterAgentInput {
    fn from(value: requests::CreateAgentBody) -> Self {
        Self {
            name: value.name,
            description: value.description,
            deployment_modality: value.deployment_modality.into(),
            endpoints: value
                .rest_http_endpoints
                .into_iter()
                .map(Into::into)
                .chain(value.rpc_endpoints.into_iter().map(Into::into))
                .chain(value.stdio_endpoints.into_iter().map(Into::into))
                .collect(),
            agent_record_id: value.agent_record_id,
            visibility: value.visibility.into(),
        }
    }
}

impl From<requests::RestHttpAgentEndpoint> for inputs::AgentEndpointInput {
    fn from(value: requests::RestHttpAgentEndpoint) -> Self {
        Self {
            name: value.name,
            protocol: inputs::ProtocolInput::RestHttp,
            message_binding: value.message_binding.map(Into::into),
            base_url: value.base_url,
            liveness_probe: value.liveness_probe.map(Into::into),
        }
    }
}

impl From<requests::RpcAgentEndpoint> for inputs::AgentEndpointInput {
    fn from(value: requests::RpcAgentEndpoint) -> Self {
        Self {
            name: value.name,
            protocol: inputs::ProtocolInput::Rpc,
            message_binding: value.message_binding.map(Into::into),
            base_url: value.base_url,
            liveness_probe: None,
        }
    }
}

impl From<requests::StdioAgentEndpoint> for inputs::AgentEndpointInput {
    fn from(value: requests::StdioAgentEndpoint) -> Self {
        Self {
            name: value.name,
            protocol: inputs::ProtocolInput::Stdio,
            message_binding: value.message_binding.map(Into::into),
            base_url: value.base_url,
            liveness_probe: None,
        }
    }
}

impl From<requests::AgentDeploymentModality> for inputs::AgentDeploymentModalityInput {
    fn from(value: requests::AgentDeploymentModality) -> Self {
        match value {
            requests::AgentDeploymentModality::Persistent => Self::Persistent,
            requests::AgentDeploymentModality::OnDemand => Self::OnDemand,
        }
    }
}

impl From<MessageBinding> for inputs::MessageBindingInput {
    fn from(value: MessageBinding) -> Self {
        match value {
            MessageBinding::HttpJson => Self::HttpJson,
            MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            MessageBinding::Grpc => Self::Grpc,
        }
    }
}

impl From<RestHttpLivenessProbe> for inputs::LivenessProbeConfigurationInput {
    fn from(value: RestHttpLivenessProbe) -> Self {
        Self::RestHttp {
            route: value.route,
            interval_seconds: value.interval_seconds,
            timeout_seconds: value.timeout_seconds,
            missed_heartbeat_threshold: value.missed_heartbeat_threshold,
            initial_delay_seconds: value.initial_delay_seconds,
        }
    }
}

impl From<RequestVisibility> for inputs::VisibilityInput {
    fn from(value: RequestVisibility) -> Self {
        match value {
            RequestVisibility::Public => Self::Public,
            RequestVisibility::Private => Self::Private,
        }
    }
}

impl From<inputs::AgentEndpointInput> for entities::AgentEndpoint {
    fn from(value: inputs::AgentEndpointInput) -> Self {
        Self::new(
            value.name,
            value.protocol.into(),
            value.message_binding.map(Into::into),
            value.base_url,
            value.liveness_probe.map(Into::into),
        )
    }
}

impl From<inputs::ProtocolInput> for record_entities::Protocol {
    fn from(value: inputs::ProtocolInput) -> Self {
        match value {
            inputs::ProtocolInput::RestHttp => Self::RestHttp,
            inputs::ProtocolInput::Rpc => Self::Rpc,
            inputs::ProtocolInput::Stdio => Self::Stdio,
        }
    }
}

impl From<inputs::MessageBindingInput> for record_entities::MessageBinding {
    fn from(value: inputs::MessageBindingInput) -> Self {
        match value {
            inputs::MessageBindingInput::HttpJson => Self::HttpJson,
            inputs::MessageBindingInput::JsonRpc2_0 => Self::JsonRpc2_0,
            inputs::MessageBindingInput::Grpc => Self::Grpc,
        }
    }
}

impl From<inputs::LivenessProbeConfigurationInput> for record_entities::LivenessProbeConfiguration {
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

impl From<inputs::AgentDeploymentModalityInput> for entities::AgentDeploymentModality {
    fn from(value: inputs::AgentDeploymentModalityInput) -> Self {
        match value {
            inputs::AgentDeploymentModalityInput::Persistent => Self::Persistent,
            inputs::AgentDeploymentModalityInput::OnDemand => Self::OnDemand,
        }
    }
}

impl From<inputs::VisibilityInput> for Visibility {
    fn from(value: inputs::VisibilityInput) -> Self {
        match value {
            inputs::VisibilityInput::Public => Self::Public,
            inputs::VisibilityInput::Private => Self::Private,
        }
    }
}
