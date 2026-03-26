use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::entities::identity as entities;
use crate::infra::common::mongo::{ToBsonDateTime, ToTimeStamp};

// Document
type PrincipalId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FederatedIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub principal_id: PrincipalId,
    pub issuer: String,
    pub subject: String,
    pub metadata: Option<Value>,
    pub tenant_id: String,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}

// Mapping (Entity -> Document)
impl From<(entities::FederatedIdentity, String)> for FederatedIdentity {
    fn from(value: (entities::FederatedIdentity, String)) -> Self {
        let identity = value.0;
        let principal_id = value.1;
        Self {
            _id: None,
            principal_id,
            issuer: identity.issuer,
            subject: identity.subject,
            metadata: identity.metadata,
            tenant_id: identity.tenant_id,
            created_at: identity.created_at.to_bson(),
            last_modified: identity.last_modified.to_bson(),
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