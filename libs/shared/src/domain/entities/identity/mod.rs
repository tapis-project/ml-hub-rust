use serde_json::Value;
use crate::domain::entities::timestamp::TimeStamp;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Authority {
    Tapis
}

impl Authority {
    pub fn all() -> Vec<Authority> {
        vec![
            Self::Tapis
        ]
    }
}

pub struct Realm {
    pub authority: Authority,
    pub issuer: String
}

#[derive(Clone, Debug)]
pub struct FederatedIdentity {
    pub authority: Authority,
    pub issuer: String, 
    pub subject: String,
    pub metadata: Option<Value>,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
}

#[derive(Clone, Debug)]
pub struct NewFederatedIdentityProps {
    pub authority: Authority,
    pub issuer: String, 
    pub subject: String,
    pub metadata: Option<Value>,
}

impl FederatedIdentity {
    pub fn new(props: NewFederatedIdentityProps) -> Self {
        let now = TimeStamp::now();
        
        Self {
            authority: props.authority,
            issuer: props.issuer,
            subject: props.subject,
            metadata: props.metadata,
            created_at: now.clone(),
            last_modified: now.clone()
        }
    }
}

impl FederatedIdentity {
    pub fn realm(&self) -> Realm {
        Realm {
            authority: self.authority.clone(),
            issuer: self.issuer.clone(),
        }
    }
}