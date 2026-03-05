use uuid::Uuid;
use crate::domain::entities::identity::FederatedIdentity;
use crate::domain::entities::timestamp::TimeStamp;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    User,
    Admin,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub identities: Vec<FederatedIdentity>,
    pub last_seen: TimeStamp,
    pub last_modified: TimeStamp,
}