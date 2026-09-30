use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDatasetsByPlatformPath {
    pub platform: String,
}
