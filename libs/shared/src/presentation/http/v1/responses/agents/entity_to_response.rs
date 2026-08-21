use crate::domain::entities::agent as entities;
use crate::presentation::http::v1::responses::agents as responses;

impl From<entities::Agent> for responses::Agent {
    fn from(value: entities::Agent) -> Self {
        Self {
            id: *value.id(),
            name: value.name().clone(),
            tenant_id: value.tenant_id().clone(),
            description: value.description().clone(),
        }
    }
}
