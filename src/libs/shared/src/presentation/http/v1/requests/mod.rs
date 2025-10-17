pub mod datasets;
pub mod models;
pub mod training;
pub mod inference;
pub mod artifacts;
pub mod headers;
pub mod filtering;
pub mod archive;
pub mod artifact_ingestions;
pub mod artifact_publications;
pub mod skills;
pub mod domains;
pub mod discover_models;

use serde_json::Value;

pub type Parameters = std::collections::hash_map::HashMap<String, Value>;


