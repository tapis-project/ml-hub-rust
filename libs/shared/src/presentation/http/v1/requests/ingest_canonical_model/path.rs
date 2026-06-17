use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestCanonicalModelPath {
    pub author: String,
    pub name: String,
}