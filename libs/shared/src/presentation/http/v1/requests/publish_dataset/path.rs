use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishDatasetPath {
    pub platform: String,
    pub dataset_id: String,
}
