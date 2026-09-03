pub mod document_to_entity;
pub mod entity_to_document;

use mongodb::bson::{oid::ObjectId, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub tenant_id: String,
    pub target_resource_urn: String,
    pub target_name: String,
    pub slug: String,
}

#[cfg(test)]
#[path = "endpoint.test.rs"]
mod endpoint_test;
