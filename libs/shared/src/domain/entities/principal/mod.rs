use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::identity::FederatedIdentity;

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum PrincipalError {
    #[error("The Principal's tenant id and the federated identities tenant id do not match")]
    TenantMismatch,
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
    pub tenant_id: String,
    active_identity: FederatedIdentity,
    pub created_at: TimeStamp,
    pub last_seen_at: TimeStamp,
}

impl Principal {
    pub fn new_user(props: NewUserPrincipalProps) -> Result<Self, PrincipalError> {
        if props.tenant_id != props.identity.tenant_id.clone() {
            return Err(PrincipalError::TenantMismatch)
        }

        let now = TimeStamp::now();
        
        Ok(Self {
            id: props.id,
            kind: Kind::User,
            tenant_id: props.tenant_id,
            active_identity: props.identity,
            created_at: now.clone(),
            last_seen_at: now.clone(),
        })
    }

    pub fn active_identity(&self) -> FederatedIdentity {
        return self.active_identity.clone()
    }

    pub fn rehydrate(props: RehydrateProps) -> Result<Self, PrincipalError> {
        if props.tenant_id != props.identity.tenant_id {
            return Err(PrincipalError::TenantMismatch)
        }

        Ok(Self {
            id: props.id,
            kind: props.kind,
            tenant_id: props.tenant_id,
            active_identity: props.identity,
            created_at: props.created_at,
            last_seen_at: props.last_seen_at,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NewUserPrincipalProps {
    pub id: String,
    pub tenant_id: String,
    pub identity: FederatedIdentity,
}

#[derive(Clone, Debug)]
pub struct RehydrateProps {
    pub id: String,
    pub kind: Kind,
    pub tenant_id: String,
    pub identity: FederatedIdentity,
    pub created_at: TimeStamp,
    pub last_seen_at: TimeStamp,
}