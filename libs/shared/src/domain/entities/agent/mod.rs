use std::collections::HashSet;

use nonempty::NonEmpty;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::agent_record::{
    AgentRecord, LivenessProbeConfiguration, MessageBinding, Protocol,
};
use crate::impl_urn_generator;
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::TimeStamp;

#[derive(Clone, Debug)]
pub struct Agent {
    id: Uuid,
    tenant_id: String,
    name: String,
    owner: String,
    description: String,
    deployment_modality: AgentDeploymentModality,
    liveness: AgentLiveness,
    /// The protocols, message bindings, and base URLs for communicating with the agent.
    target_endpoints: NonEmpty<AgentEndpoint>,
    visibility: Visibility,
    created_at: TimeStamp,
    last_modified: TimeStamp,
    agent_record_id: Option<Uuid>,
}

impl_urn_generator!(Agent, tenant_id, "agent", id);

impl Agent {
    pub fn register(
        props: RegisterAgentProps,
        agent_record: Option<&AgentRecord>,
    ) -> Result<Self, AgentError> {
        let endpoints = Self::target_endpoints_from_vec(props.endpoints)?;
        Self::validate_target_endpoints(&endpoints)?;

        if let Some(agent_record) = agent_record {
            Self::validate_agent_record_interfaces(&endpoints, agent_record)?;
        }

        let now = TimeStamp::now();

        Ok(Self {
            id: Uuid::now_v7(),
            tenant_id: props.tenant_id,
            name: props.name,
            owner: props.owner,
            description: props.description,
            deployment_modality: props.deployment_modality,
            liveness: AgentLiveness::Dead,
            target_endpoints: endpoints,
            visibility: props.visibility,
            created_at: now.clone(),
            last_modified: now,
            agent_record_id: agent_record.map(|record| *record.id()),
        })
    }

    pub fn reconstitute(props: ReconstituteAgentProps) -> Result<Self, AgentError> {
        let endpoints = Self::target_endpoints_from_vec(props.endpoints).map_err(|error| {
            AgentError::DataIntegrityError(format!(
                "Agent contains invalid target endpoints: {error}"
            ))
        })?;

        Self::validate_target_endpoints(&endpoints).map_err(|error| {
            AgentError::DataIntegrityError(format!(
                "Agent contains invalid target endpoints: {error}"
            ))
        })?;

        Ok(Self {
            id: props.id,
            tenant_id: props.tenant_id,
            name: props.name,
            owner: props.owner,
            description: props.description,
            deployment_modality: props.deployment_modality,
            liveness: props.liveness,
            target_endpoints: endpoints,
            visibility: props.visibility,
            created_at: props.created_at,
            last_modified: props.last_modified,
            agent_record_id: props.agent_record_id,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn deployment_modality(&self) -> &AgentDeploymentModality {
        &self.deployment_modality
    }

    pub fn liveness(&self) -> &AgentLiveness {
        &self.liveness
    }

    pub fn target_endpoints(&self) -> &NonEmpty<AgentEndpoint> {
        &self.target_endpoints
    }

    pub fn is_public(&self) -> bool {
        matches!(self.visibility, Visibility::Public)
    }

    pub fn created_at(&self) -> &TimeStamp {
        &self.created_at
    }

    pub fn last_modified(&self) -> &TimeStamp {
        &self.last_modified
    }

    pub fn agent_record_id(&self) -> Option<&Uuid> {
        self.agent_record_id.as_ref()
    }

    fn target_endpoints_from_vec(
        endpoints: Vec<AgentEndpoint>,
    ) -> Result<NonEmpty<AgentEndpoint>, AgentError> {
        NonEmpty::from_vec(endpoints).ok_or(AgentError::MissingTargetEndpoints)
    }

    fn validate_target_endpoints(endpoints: &NonEmpty<AgentEndpoint>) -> Result<(), AgentError> {
        let mut names = HashSet::new();

        for endpoint in endpoints {
            if let Some(name) = endpoint.name() {
                if name.is_empty() {
                    return Err(AgentError::EmptyAgentEndpointIdentifier);
                }

                if !names.insert(name) {
                    return Err(AgentError::DuplicateAgentEndpointIdentifier(name.into()));
                }
            }

            if endpoint.liveness_probe().is_some()
                && !matches!(endpoint.protocol(), Protocol::RestHttp)
            {
                return Err(AgentError::IncompatibleLivenessProbeConfiguration(
                    endpoint.name()
                        .unwrap_or("<unnamed>")
                        .into(),
                ));
            }
        }

        Ok(())
    }

    fn validate_agent_record_interfaces(
        endpoints: &NonEmpty<AgentEndpoint>,
        agent_record: &AgentRecord,
    ) -> Result<(), AgentError> {
        for endpoint in endpoints {
            let name = endpoint
                .name()
                .ok_or(AgentError::MissingAgentEndpointIdentifier)?;

            let record_interface = agent_record
                .interfaces()
                .iter()
                .find(|interface| interface.name() == name)
                .ok_or_else(|| AgentError::MismatchedAgentInterfaceDetails(name.into()))?;

            if !same_protocol(endpoint.protocol(), record_interface.protocol())
                || !same_message_binding(
                    endpoint.message_binding(),
                    record_interface.message_binding(),
                )
                || !same_liveness_probe(
                    endpoint.liveness_probe(),
                    record_interface.liveness_probe_config(),
                )
            {
                return Err(AgentError::MismatchedAgentInterfaceDetails(name.into()));
            }
        }

        Ok(())
    }
}

fn same_protocol(left: &Protocol, right: &Protocol) -> bool {
    matches!(
        (left, right),
        (Protocol::RestHttp, Protocol::RestHttp)
            | (Protocol::Rpc, Protocol::Rpc)
            | (Protocol::Stdio, Protocol::Stdio)
    )
}

fn same_message_binding(left: &Option<MessageBinding>, right: &Option<MessageBinding>) -> bool {
    matches!(
        (left, right),
        (None, None)
            | (
                Some(MessageBinding::HttpJson),
                Some(MessageBinding::HttpJson)
            )
            | (
                Some(MessageBinding::JsonRpc2_0),
                Some(MessageBinding::JsonRpc2_0)
            )
            | (Some(MessageBinding::Grpc), Some(MessageBinding::Grpc))
    )
}

fn same_liveness_probe(
    left: Option<&LivenessProbeConfiguration>,
    right: Option<&LivenessProbeConfiguration>,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (
            Some(LivenessProbeConfiguration::RestHttp {
                interval_seconds: left_interval,
                timeout_seconds: left_timeout,
                missed_heartbeat_threshold: left_threshold,
                initial_delay_seconds: left_delay,
                ..
            }),
            Some(LivenessProbeConfiguration::RestHttp {
                interval_seconds: right_interval,
                timeout_seconds: right_timeout,
                missed_heartbeat_threshold: right_threshold,
                initial_delay_seconds: right_delay,
                ..
            }),
        ) => {
            left_interval == right_interval
                && left_timeout == right_timeout
                && left_threshold == right_threshold
                && left_delay == right_delay
        }
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct RegisterAgentProps {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub tenant_id: String,
    pub deployment_modality: AgentDeploymentModality,
    pub endpoints: Vec<AgentEndpoint>,
    pub visibility: Visibility,
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentProps {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub tenant_id: String,
    pub deployment_modality: AgentDeploymentModality,
    pub liveness: AgentLiveness,
    pub endpoints: Vec<AgentEndpoint>,
    pub visibility: Visibility,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub agent_record_id: Option<Uuid>,
}

#[derive(Clone, Debug)]
pub struct AgentEndpoint {
    name: Option<String>,
    protocol: Protocol,
    message_binding: Option<MessageBinding>,
    base_url: Option<String>,
    liveness_probe: Option<LivenessProbeConfiguration>,
}

impl AgentEndpoint {
    pub fn new(
        name: Option<String>,
        protocol: Protocol,
        message_binding: Option<MessageBinding>,
        base_url: Option<String>,
        liveness_probe: Option<LivenessProbeConfiguration>,
    ) -> Self {
        Self {
            name,
            protocol,
            message_binding,
            base_url,
            liveness_probe,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn message_binding(&self) -> &Option<MessageBinding> {
        &self.message_binding
    }

    pub fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn liveness_probe(&self) -> Option<&LivenessProbeConfiguration> {
        self.liveness_probe.as_ref()
    }
}

#[derive(Clone, Debug)]
pub enum AgentLiveness {
    Alive,
    Dead,
}

#[derive(Clone, Debug)]
pub enum AgentDeploymentModality {
    Persistent,
    OnDemand,
}

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent MUST have at least one target endpoint")]
    MissingTargetEndpoints,
    #[error("Agent endpoint identifier MUST not be empty")]
    EmptyAgentEndpointIdentifier,
    #[error("Agent endpoint identifier is required when registering from an agent record")]
    MissingAgentEndpointIdentifier,
    #[error("Duplicate agent endpoint identifier: {0}")]
    DuplicateAgentEndpointIdentifier(String),
    #[error("Incompatible liveness probe configuration for agent endpoint: {0}")]
    IncompatibleLivenessProbeConfiguration(String),
    #[error("Mismatched agent interface details: {0}")]
    MismatchedAgentInterfaceDetails(String),
    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
pub mod test_fixtures;

#[cfg(test)]
#[path = "agent.test.rs"]
mod agent_test;
