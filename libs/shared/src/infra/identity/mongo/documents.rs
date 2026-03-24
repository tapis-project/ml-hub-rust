use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FederatedIdentity {
    pub issuer: String,
    pub subject: String,
    pub metadata: Option<Value>,
    pub tenant_id: String,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}