use crate::domain::entities::agent as entities;
use crate::infra::persistence::mongo::documents::agent as documents;

impl TryFrom<documents::Agent> for entities::Agent {
    type Error = entities::AgentError;
    fn try_from(value: documents::Agent) -> Result<Self, Self::Error> {
        entities::Agent::reconstitute(entities::ReconstituteAgentProps {
            id: uuid::Uuid::from_bytes(value.id.bytes()),
            tenant_id: value.tenant_id,
            name: value.name,
            owner: value.owner,
            description: value.description,
            deployment_modality: value.deployment_modality.into(),
            liveness: value.liveness.into(),
            endpoints: value.target_endpoints.into_iter().map(Into::into).collect(),
            tags: value.tags,
            visibility: value.visibility.into(),
            created_at: crate::shared_kernel::value_objects::TimeStamp::parse_string(
                &value.created_at,
            )
            .map_err(|error| entities::AgentError::DataIntegrityError(error.to_string()))?,
            last_modified: crate::shared_kernel::value_objects::TimeStamp::parse_string(
                &value.last_modified,
            )
            .map_err(|error| entities::AgentError::DataIntegrityError(error.to_string()))?,
            agent_record_id: value
                .agent_record_id
                .map(|id| uuid::Uuid::from_bytes(id.bytes())),
        })
    }
}

impl From<documents::AgentEndpoint> for entities::AgentEndpoint {
    fn from(value: documents::AgentEndpoint) -> Self {
        Self::new(
            value.name,
            value.protocol.into(),
            value.message_binding.map(Into::into),
            value.base_url,
            value.liveness_probe.map(Into::into),
        )
    }
}
impl From<documents::AgentLiveness> for entities::AgentLiveness {
    fn from(value: documents::AgentLiveness) -> Self {
        match value {
            documents::AgentLiveness::Alive => Self::Alive,
            documents::AgentLiveness::Dead => Self::Dead,
        }
    }
}
impl From<documents::AgentDeploymentModality> for entities::AgentDeploymentModality {
    fn from(value: documents::AgentDeploymentModality) -> Self {
        match value {
            documents::AgentDeploymentModality::Persistent => Self::Persistent,
            documents::AgentDeploymentModality::OnDemand => Self::OnDemand,
        }
    }
}
