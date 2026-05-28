use serde_json::Value;
use crate::domain::entities::timestamp::TimeStamp;

pub const MLHUB_SERVICE_ID: &'static str = "mlhub";

#[derive(Clone, Debug)]
pub struct FederatedIdentity {
    pub issuer: String,
    pub subject: String,
    pub metadata: Option<Value>,
    pub tenant_id: String,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub last_seen: TimeStamp
}

#[derive(Clone, Debug)]
pub struct NewFederatedIdentityProps {
    pub issuer: String, 
    pub subject: String,
    pub tenant_id: String,
    pub metadata: Option<Value>,
}

impl FederatedIdentity {
    pub fn new(props: NewFederatedIdentityProps) -> Self {
        let now = TimeStamp::now();
        
        Self {
            issuer: props.issuer,
            subject: props.subject,
            tenant_id: props.tenant_id,
            metadata: props.metadata,
            created_at: now.clone(),
            last_modified: now.clone(),
            last_seen: now.clone(),
        }
    }
}