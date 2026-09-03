use crate::domain::entities::agent as entities;
use crate::infra::persistence::mongo::documents::agent as documents;
use crate::infra::persistence::mongo::documents::visibility::Visibility;

impl From<&entities::Agent> for documents::Agent {
    fn from(value: &entities::Agent) -> Self {
        Self {
            _id: None,
            id: mongodb::bson::Uuid::from_bytes(*value.id().as_bytes()),
            tenant_id: value.tenant_id().into(),
            name: value.name().into(),
            owner: value.owner().into(),
            description: value.description().into(),
            deployment_modality: value.deployment_modality().into(),
            liveness: value.liveness().into(),
            last_missed_heartbeat: value
                .last_missed_heartbeat()
                .map(|timestamp| String::from(timestamp.clone())),
            consecutive_missed_heartbeats: value.consecutive_missed_heartbeats(),
            target_endpoints: value
                .target_endpoints()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            tags: value
                .tags()
                .iter()
                .map(|tag| tag.as_str().to_owned())
                .collect(),
            visibility: if value.is_public() {
                Visibility::Public
            } else {
                Visibility::Private
            },
            created_at: String::from(value.created_at().clone()),
            last_modified: String::from(value.last_modified().clone()),
            agent_record_id: value
                .agent_record_id()
                .copied()
                .map(|id| mongodb::bson::Uuid::from_bytes(*id.as_bytes())),
        }
    }
}

impl From<entities::AgentEndpoint> for documents::AgentEndpoint {
    fn from(value: entities::AgentEndpoint) -> Self {
        Self {
            name: value.name().map(str::to_owned),
            protocol: value.protocol().clone().into(),
            message_binding: value.message_binding().clone().map(Into::into),
            base_url: value.base_url().map(str::to_owned),
            liveness_probe: value.liveness_probe().cloned().map(Into::into),
        }
    }
}

impl From<&entities::AgentLiveness> for documents::AgentLiveness {
    fn from(value: &entities::AgentLiveness) -> Self {
        match value {
            entities::AgentLiveness::Alive => Self::Alive,
            entities::AgentLiveness::Dead => Self::Dead,
        }
    }
}
impl From<&entities::AgentDeploymentModality> for documents::AgentDeploymentModality {
    fn from(value: &entities::AgentDeploymentModality) -> Self {
        match value {
            entities::AgentDeploymentModality::Persistent => Self::Persistent,
            entities::AgentDeploymentModality::OnDemand => Self::OnDemand,
        }
    }
}
