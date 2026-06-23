use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::presentation::http::v1::requests::common::Scope;

#[derive(Deserialize, Debug, Clone, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct GetModelByAuthorAndNameQueryParams {
    #[serde(default = "default_scope")]
    #[param(value_type = Scope, inline)]
    /// Selector for global vs tenant-scoped models
    pub scope: Scope
}

fn default_scope () -> Scope { Scope::Tenant }