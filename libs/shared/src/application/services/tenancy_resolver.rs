//! The TenancyResolver resolves the tenant id based on the intent which is expressed
//! in terms of Scope. If Scope is Global then the resolve to the GLOBAL_TENANT.
//! If Scope is Tenant, resolve to the provided tenant id.

use crate::{application::inputs::common::Scope, domain::entities::tenancy::GLOBAL_TENANT};

pub struct TenancyResolver;

impl TenancyResolver {
    pub fn resolve_from_scope(scope: &Scope, tenant_id: &String) -> String {
        match scope {
            Scope::Global => GLOBAL_TENANT.into(),
            Scope::Tenant => tenant_id.clone(),
        }
    }
}