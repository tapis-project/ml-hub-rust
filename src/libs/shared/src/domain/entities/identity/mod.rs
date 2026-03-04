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
    pub metadata: Value,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
}

impl FederatedIdentity {
    pub fn realm(&self) -> Realm {
        Realm {
            authority: self.authority.clone(),
            issuer: self.issuer.clone(),
        }
    }
}