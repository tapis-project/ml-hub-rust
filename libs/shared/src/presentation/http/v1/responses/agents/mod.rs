mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Agent {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub description: Option<String>,
}

#[cfg(test)]
#[path = "agents.test.rs"]
mod agents_test;
