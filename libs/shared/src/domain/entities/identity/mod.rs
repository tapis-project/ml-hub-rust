use serde_json::Value;
use crate::domain::entities::timestamp::TimeStamp;

#[derive(Clone, Debug)]
pub struct FederatedIdentity {
    pub issuer: String,
    pub subject: String,
    pub metadata: Option<Value>,
    pub tenant_id: String,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
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
            last_modified: now.clone()
        }
    }
}