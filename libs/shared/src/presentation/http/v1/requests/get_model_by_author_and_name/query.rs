use serde::Deserialize;
use utoipa::IntoParams;

use crate::presentation::http::v1::requests::common::Scope;

#[derive(Deserialize, Debug, Clone, IntoParams)]
pub struct GetModelByAuthorAndNameQueryParams {
    #[serde(default = "default_scope")]
    /// Selector for global vs tenant-scoped models
    pub scope: Scope
}

fn default_scope () -> Scope { Scope::Tenant }