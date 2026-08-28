use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct ListAgentsQueryParams {
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub include_endpoints: bool,
}

#[derive(Deserialize, Default, ToSchema)]
pub enum Scope {
    #[default]
    Owned,
    Shared,
}

#[cfg(test)]
#[path = "query.test.rs"]
mod query_test;
