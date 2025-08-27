use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetArtifactIngestionPath {
    pub ingestion_id: String
}