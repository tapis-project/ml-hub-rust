use crate::domain::entities;
use crate::domain::entities::principal::PrincipalError;
use crate::infra::common::mongo::{ToBsonDateTime, ToTimeStamp};
use crate::infra::identity::mongo::documents::FederatedIdentity;

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const PRINCIPAL_COLLECTION: &str = "PRINCIPALS";

// Documents
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Kind {
    User,
    System,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: String,
    pub kind: Kind,
    pub tenant_id: String,
    pub created_at: DateTime,
    pub last_seen: DateTime,
    pub last_modified: DateTime,
}

// Mappings (Entity -> Document)
impl From<entities::principal::Kind> for Kind {
    fn from(value: entities::principal::Kind) -> Self {
        match value {
            entities::principal::Kind::User => Kind::User,
            entities::principal::Kind::System => Kind::System,
        }
    }
}

impl From<entities::principal::Principal> for Principal {
    fn from(value: entities::principal::Principal) -> Self {
        Self {
            _id: None,
            id: value.id,
            kind: Kind::from(value.kind),
            tenant_id: value.tenant_id,
            created_at: value.created_at.to_bson(),
            last_seen: value.last_seen.to_bson(),
            last_modified: value.last_modified.to_bson(),
        }
    }
}

// Mappings (Document -> Entity)
impl From<Kind> for entities::principal::Kind {
    fn from(value: Kind) -> Self {
        match value {
            Kind::User => entities::principal::Kind::User,
            Kind::System => entities::principal::Kind::System,
        }
    }
}

impl TryFrom<(Principal, FederatedIdentity)> for entities::principal::Principal {
    type Error = PrincipalError;
    
    fn try_from(value: (Principal, FederatedIdentity)) -> Result<Self, Self::Error> {
        let principal = value.0;

        let props = entities::principal::RehydrateProps {
            id: principal.id,
            kind: entities::principal::Kind::from(principal.kind),
            tenant_id: principal.tenant_id,
            identity: entities::identity::FederatedIdentity::from(value.1.clone()),
            created_at: principal.created_at.to_timestamp(),
            last_seen: principal.last_seen.to_timestamp(),
            last_modified: principal.last_modified.to_timestamp(),
        };

        entities::principal::Principal::rehydrate(props)
    }
}