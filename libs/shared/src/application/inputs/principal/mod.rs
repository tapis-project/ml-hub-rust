use crate::domain::entities::identity::FederatedIdentity;

pub struct FindByFederatedIdentity {
    pub identity: FederatedIdentity,
}

pub struct GetOrCreateFromFederatedIdentity {
    pub principal_id: String,
    pub identity: FederatedIdentity,
}