use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetArtifactPublicationPath {
    pub publication_id: String
}