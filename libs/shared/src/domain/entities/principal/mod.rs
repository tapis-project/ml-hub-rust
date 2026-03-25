use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::identity::FederatedIdentity;

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum PrincipalError {
    #[error("Cannot create a Principal without at least one FederatedIdentity")]
    MissingFederatedIdentity
}

#[derive(Clone, Debug)]
pub enum Kind {
    User,
    System,
}

#[derive(Clone, Debug)]
pub struct Principal {
    pub id: String,
    pub kind: Kind,
    pub tenant_id: Option<String>,
    identities: Vec<FederatedIdentity>,
    pub created_at: TimeStamp,
    pub last_seen: TimeStamp,
    pub last_modified: TimeStamp,
}

impl Principal {
    pub fn new_user(props: NewUserPrincipalProps) -> Result<Self, PrincipalError> {
        if props.identities.len() < 1 {
            return Err(PrincipalError::MissingFederatedIdentity)
        }

        let now = TimeStamp::now();
        
        Ok(Self {
            id: props.id,
            kind: Kind::User,
            tenant_id: Some(props.tenant_id),
            identities: props.identities,
            created_at: now.clone(),
            last_modified: now.clone(),
            last_seen: now.clone(),
        })
    }

    pub fn identites(&self) -> Vec<FederatedIdentity> {
        return self.identities.clone()
    }

    pub fn rehydrate(props: RehydrateProps) -> Self {
        Self {
            id: props.id,
            kind: props.kind,
            tenant_id: props.tenant_id,
            identities: props.identities,
            created_at: props.created_at,
            last_modified: props.last_modified,
            last_seen: props.last_seen,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewUserPrincipalProps {
    pub id: String,
    pub tenant_id: String,
    pub identities: Vec<FederatedIdentity>,
}

#[derive(Clone, Debug)]
pub struct RehydrateProps {
    pub id: String,
    pub kind: Kind,
    pub tenant_id: Option<String>,
    pub identities: Vec<FederatedIdentity>,
    pub created_at: TimeStamp,
    pub last_seen: TimeStamp,
    pub last_modified: TimeStamp,
}