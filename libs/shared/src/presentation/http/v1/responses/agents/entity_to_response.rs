use crate::domain::entities::agent as entities;
use crate::domain::entities::agent_record::LivenessProbeConfiguration;
use crate::presentation::http::v1::responses::agent_records::{
    MessageBinding, RestHttpLivenessProbe,
};
use crate::presentation::http::v1::responses::agents as responses;
use crate::presentation::http::v1::responses::visibility::Visibility;

impl From<entities::Agent> for responses::Agent {
    fn from(value: entities::Agent) -> Self {
        let mut rest_http_endpoints = Vec::new();
        let mut rpc_endpoints = Vec::new();
        let mut stdio_endpoints = Vec::new();

        for endpoint in value.target_endpoints() {
            match endpoint.protocol() {
                crate::domain::entities::agent_record::Protocol::RestHttp => rest_http_endpoints
                    .push(responses::RestHttpAgentEndpoint {
                        name: endpoint.name().map(str::to_owned),
                        message_binding: endpoint.message_binding().as_ref().map(Into::into),
                        base_url: endpoint.base_url().map(str::to_owned),
                        liveness_probe: endpoint.liveness_probe().map(|probe| match probe {
                            LivenessProbeConfiguration::RestHttp {
                                route,
                                interval_seconds,
                                timeout_seconds,
                                missed_heartbeat_threshold,
                                initial_delay_seconds,
                            } => RestHttpLivenessProbe {
                                route: route.clone(),
                                interval_seconds: *interval_seconds,
                                timeout_seconds: *timeout_seconds,
                                missed_heartbeat_threshold: *missed_heartbeat_threshold,
                                initial_delay_seconds: *initial_delay_seconds,
                            },
                        }),
                    }),
                crate::domain::entities::agent_record::Protocol::Rpc => {
                    rpc_endpoints.push(responses::RpcAgentEndpoint {
                        name: endpoint.name().map(str::to_owned),
                        message_binding: endpoint.message_binding().as_ref().map(Into::into),
                        base_url: endpoint.base_url().map(str::to_owned),
                    })
                }
                crate::domain::entities::agent_record::Protocol::Stdio => {
                    stdio_endpoints.push(responses::StdioAgentEndpoint {
                        name: endpoint.name().map(str::to_owned),
                        message_binding: endpoint.message_binding().as_ref().map(Into::into),
                        base_url: endpoint.base_url().map(str::to_owned),
                    })
                }
            }
        }

        Self {
            id: *value.id(),
            name: value.name().into(),
            tenant_id: value.tenant_id().into(),
            owner: value.owner().into(),
            description: value.description().into(),
            deployment_modality: value.deployment_modality().into(),
            liveness: value.liveness().into(),
            rest_http_endpoints,
            rpc_endpoints,
            stdio_endpoints,
            visibility: if value.is_public() {
                Visibility::Public
            } else {
                Visibility::Private
            },
            created_at: String::from(value.created_at().clone()),
            last_modified: String::from(value.last_modified().clone()),
            agent_record_id: value.agent_record_id().copied(),
        }
    }
}

impl From<&entities::AgentDeploymentModality> for responses::AgentDeploymentModality {
    fn from(value: &entities::AgentDeploymentModality) -> Self {
        match value {
            entities::AgentDeploymentModality::Persistent => Self::Persistent,
            entities::AgentDeploymentModality::OnDemand => Self::OnDemand,
        }
    }
}

impl From<&entities::AgentLiveness> for responses::AgentLiveness {
    fn from(value: &entities::AgentLiveness) -> Self {
        match value {
            entities::AgentLiveness::Alive => Self::Alive,
            entities::AgentLiveness::Dead => Self::Dead,
        }
    }
}

impl From<&crate::domain::entities::agent_record::MessageBinding> for MessageBinding {
    fn from(value: &crate::domain::entities::agent_record::MessageBinding) -> Self {
        match value {
            crate::domain::entities::agent_record::MessageBinding::HttpJson => Self::HttpJson,
            crate::domain::entities::agent_record::MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            crate::domain::entities::agent_record::MessageBinding::Grpc => Self::Grpc,
        }
    }
}
