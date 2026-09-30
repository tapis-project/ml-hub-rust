use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Debug, Deserialize, IntoParams)]
pub struct ListAgentRecordsQueryParams {
    #[serde(default)]
    #[param(inline)]
    pub scope: Scope,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub enum Scope {
    Owned,
    Shared,
}

impl Default for Scope {
    fn default() -> Self {
        Self::Owned
    }
}

#[cfg(test)]
#[path = "query.test.rs"]
mod list_agent_records_query_test;
