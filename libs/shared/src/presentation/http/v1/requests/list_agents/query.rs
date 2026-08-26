use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct ListAgentsQueryParams {
    #[serde(default)]
    pub scope: Scope,
}

#[derive(Deserialize, Default, ToSchema)]
pub enum Scope {
    #[default]
    Owned,
    Shared,
}
