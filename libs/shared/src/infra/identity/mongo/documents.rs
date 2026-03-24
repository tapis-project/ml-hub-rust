use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::entities::identity as entities;
use crate::infra::common::mongo::ToBsonDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FederatedIdentity {
    pub issuer: String,
    pub subject: String,
    pub metadata: Option<Value>,
    pub tenant_id: String,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}

impl From<entities::FederatedIdentity> for FederatedIdentity {
    fn from(value: entities::FederatedIdentity) -> Self {
        Self {
            issuer: value.issuer,
            subject: value.subject,
            metadata: value.metadata,
            tenant_id: value.tenant_id,
            created_at: value.created_at.to_bson(),
            last_modified: value.last_modified.to_bson(),
        }
    }
}