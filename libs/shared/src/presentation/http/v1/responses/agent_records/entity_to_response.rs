use crate::domain::entities::agent_record as entities;
use crate::presentation::http::v1::responses::agent_records as responses;

impl From<entities::AgentRecord> for responses::AgentRecord {
    fn from(value: entities::AgentRecord) -> Self {
        Self {
            id: *value.id(),
            name: value.name().clone(),
            tenant_id: value.tenant_id().clone(),
            owner: value.owner().clone(),
            description: value.description().clone(),
        }
    }
}
