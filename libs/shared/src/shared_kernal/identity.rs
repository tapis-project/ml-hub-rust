use crate::domain::entities::principal::{Principal, Kind};
use crate::shared_kernal::tenancy::GLOBAL_TENANT;

pub const MLHUB_SERVICE_PRINCIPAL_ID: &'static str = "mlhub";

#[derive(Debug, Clone)]
pub struct IdentityContext {
    actor: Actor,
    token: String
}

impl IdentityContext {
    pub fn new(actor: Actor, token: String) -> Self {
        Self {
            actor,
            token
        }
    }

    // Creates a service account identity context
    pub fn system() -> Self {
        Self {
            actor: Actor::system(),
            token: "".into(),
        }
    }

    pub fn actor_principal_id(&self) -> &String {
        &self.actor.principal_id()
    }

    pub fn actor_tenant_id(&self) -> &String {
        &self.actor.tenant_id()
    }

    pub fn actor_kind(&self) -> &Kind {
        &self.actor.kind()
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[derive(Debug, Clone)]
pub struct Actor {
    principal_id: String,
    tenant_id: String,
    kind: Kind,
}

impl Actor {
    pub fn principal_id(&self) -> &String {
        &self.principal_id
    }

    pub fn tenant_id(&self) -> &String {
        &self.tenant_id
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn system() -> Self {
        Self {
            principal_id: MLHUB_SERVICE_PRINCIPAL_ID.into(),
            tenant_id: GLOBAL_TENANT.into(),
            kind: Kind::System,
        }
    }
}

impl From<Principal> for Actor {
    fn from(value: Principal) -> Self {
        Self {
            principal_id: value.id.clone(),
            tenant_id: value.tenant_id.clone(),
            kind: value.kind.clone()
        }
    }
}