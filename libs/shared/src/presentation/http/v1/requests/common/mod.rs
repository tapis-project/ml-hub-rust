pub mod tasks;
pub mod headers;
pub mod archive;
pub mod filtering;

use serde_json::Value;

pub type Parameters = std::collections::hash_map::HashMap<String, Value>;