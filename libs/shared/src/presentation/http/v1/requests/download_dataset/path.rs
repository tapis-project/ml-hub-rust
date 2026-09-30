use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadDatasetPath {
    pub platform: String,
    pub dataset_id: String,
}
