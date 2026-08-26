#![cfg(test)]

use crate::domain::entities::agent::{
    Agent, AgentDeploymentModality, AgentEndpoint, AgentError, ReconstituteAgentProps,
    RegisterAgentProps,
};
use crate::domain::entities::agent_record::{MessageBinding, Protocol};
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::TimeStamp;

pub struct AgentBuilder {
    name: Option<String>,
    description: Option<String>,
    owner: Option<String>,
    tenant_id: Option<String>,
    deployment_modality: Option<AgentDeploymentModality>,
    endpoints: Option<Vec<AgentEndpoint>>,
    tags: Option<Vec<String>>,
    last_missed_heartbeat: Option<TimeStamp>,
    consecutive_missed_heartbeats: Option<u16>,
    visibility: Option<Visibility>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            owner: None,
            tenant_id: None,
            deployment_modality: None,
            endpoints: None,
            tags: None,
            last_missed_heartbeat: None,
            consecutive_missed_heartbeats: None,
            visibility: None,
        }
    }

    pub fn with_endpoints(mut self, endpoints: Vec<AgentEndpoint>) -> Self {
        self.endpoints = Some(endpoints);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn with_last_missed_heartbeat(mut self, last_missed_heartbeat: TimeStamp) -> Self {
        self.last_missed_heartbeat = Some(last_missed_heartbeat);
        self
    }

    pub fn with_consecutive_missed_heartbeats(
        mut self,
        consecutive_missed_heartbeats: u16,
    ) -> Self {
        self.consecutive_missed_heartbeats = Some(consecutive_missed_heartbeats);
        self
    }

    pub fn build_registered(&self) -> Result<Agent, AgentError> {
        Agent::register(
            RegisterAgentProps {
                name: self.name.clone().unwrap_or_else(|| "Test Agent".into()),
                description: self
                    .description
                    .clone()
                    .unwrap_or_else(|| "Test agent description".into()),
                owner: self.owner.clone().unwrap_or_else(|| "test-owner".into()),
                tenant_id: self
                    .tenant_id
                    .clone()
                    .unwrap_or_else(|| "test-tenant".into()),
                deployment_modality: self
                    .deployment_modality
                    .clone()
                    .unwrap_or(AgentDeploymentModality::Persistent),
            endpoints: self.endpoints.clone().unwrap_or_else(|| {
                    vec![AgentEndpoint::new(
                        Some("default".into()),
                        Protocol::RestHttp,
                        Some(MessageBinding::HttpJson),
                        Some("https://example.test".into()),
                        None,
                    )]
            }),
            tags: self.tags.clone().unwrap_or_default(),
            visibility: self.visibility.clone().unwrap_or(Visibility::Private),
            },
            None,
        )
    }

    pub fn build_reconstituted(&self) -> Result<Agent, AgentError> {
        let timestamp = TimeStamp::now();

        Agent::reconstitute(ReconstituteAgentProps {
            id: uuid::Uuid::now_v7(),
            name: self.name.clone().unwrap_or_else(|| "Test Agent".into()),
            description: self
                .description
                .clone()
                .unwrap_or_else(|| "Test agent description".into()),
            owner: self.owner.clone().unwrap_or_else(|| "test-owner".into()),
            tenant_id: self
                .tenant_id
                .clone()
                .unwrap_or_else(|| "test-tenant".into()),
            deployment_modality: self
                .deployment_modality
                .clone()
                .unwrap_or(AgentDeploymentModality::Persistent),
            liveness: crate::domain::entities::agent::AgentLiveness::Dead,
            last_missed_heartbeat: self.last_missed_heartbeat.clone(),
            consecutive_missed_heartbeats: self.consecutive_missed_heartbeats.unwrap_or(0),
            endpoints: self.endpoints.clone().unwrap_or_else(|| {
                vec![AgentEndpoint::new(
                    Some("default".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    Some("https://example.test".into()),
                    None,
                )]
            }),
            tags: self.tags.clone().unwrap_or_default(),
            visibility: self.visibility.clone().unwrap_or(Visibility::Private),
            created_at: timestamp.clone(),
            last_modified: timestamp,
            agent_record_id: None,
        })
    }
}
