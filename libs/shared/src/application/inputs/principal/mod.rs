use crate::domain::entities::identity::FederatedIdentity;

pub struct FindByFederatedIdentity {
    identity: FederatedIdentity,
    tenant: String,
}