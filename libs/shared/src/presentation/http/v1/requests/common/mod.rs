pub mod tasks;
pub mod headers;
pub mod archive;
pub mod filtering;

use serde::Deserialize;
use serde_json::Value;
use strum_macros::EnumString;
use utoipa::ToSchema;

use crate::application::inputs::common as inputs;

pub type Parameters = std::collections::hash_map::HashMap<String, Value>;

#[derive(Clone, Debug, Deserialize, EnumString, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Tenant,
    Global
}

impl From<Scope> for inputs::Scope {
    fn from(value: Scope) -> Self {
        match value {
            Scope::Global => inputs::Scope::Global,
            Scope::Tenant => inputs::Scope::Tenant,
        }
    }
}