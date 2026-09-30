use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IngestDatasetPath {
    pub platform: String,
    pub dataset_id: String,
}
