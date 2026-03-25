use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::entities::identity as entities;
use crate::infra::common::mongo::{ToBsonDateTime, ToTimeStamp};

// Document
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FederatedIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub issuer: String,
    pub subject: String,
    pub metadata: Option<Value>,
    pub tenant_id: String,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}

// Mapping (Entity -> Document)
impl From<entities::FederatedIdentity> for FederatedIdentity {
    fn from(value: entities::FederatedIdentity) -> Self {
        Self {
            _id: None,
            issuer: value.issuer,
            subject: value.subject,
            metadata: value.metadata,
            tenant_id: value.tenant_id,
            created_at: value.created_at.to_bson(),
            last_modified: value.last_modified.to_bson(),
        }
    }
}

// Mapping (Document -> Entity)
impl From<FederatedIdentity> for entities::FederatedIdentity {
    fn from(value: FederatedIdentity) -> Self {
        Self {
            issuer: value.issuer,
            subject: value.subject,
            metadata: value.metadata,
            tenant_id: value.tenant_id,
            created_at: value.created_at.to_timestamp(),
            last_modified: value.last_modified.to_timestamp(),
        }
    }
}