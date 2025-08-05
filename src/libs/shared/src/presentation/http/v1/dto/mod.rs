pub mod datasets;
pub mod models;
pub mod training;
pub mod inference;
pub mod artifacts;
pub mod headers;
pub mod filtering;
pub mod archive;

use serde_json::Value;

pub type Parameters = std::collections::hash_map::HashMap<String, Value>;


