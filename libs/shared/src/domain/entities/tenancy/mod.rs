#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: String,
}

pub trait TenantScopedResource {
    fn tenant_id(&self) -> String;
}

