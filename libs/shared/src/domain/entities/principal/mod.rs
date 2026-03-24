use crate::domain::entities::timestamp::TimeStamp;

#[derive(Clone, Debug)]
pub enum PrincipalKind {
    User,
    System,
}

#[derive(Clone, Debug)]
pub struct Principal {
    pub id: String,
    pub kind: PrincipalKind,
    pub tenant_id: Option<String>,
    pub created_at: TimeStamp,
    pub last_seen: TimeStamp,
    pub last_modified: TimeStamp,
}