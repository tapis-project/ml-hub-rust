use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDatasetByPlatformPath {
    pub platform: String,
    pub dataset_id: String,
}
