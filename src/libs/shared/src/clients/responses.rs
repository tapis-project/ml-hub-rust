use crate::presentation::http::v1::dto::StagedArtifact;
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientJsonResponse<Data: Serialize, Metadata: Serialize> {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Data>,
    pub metadata: Option<Metadata>
}

impl <Data: Serialize, Metadata: Serialize>ClientJsonResponse<Data, Metadata> {
    pub fn new(status: Option<u16>, message: Option<String>, result: Option<Data>, metadata: Option<Metadata>) -> Self {
        return Self {
            status,
            message,
            result,
            metadata
        }
    }
}

pub struct ClientStagedArtifactResponse {
    pub staged_artifact: StagedArtifact,
    pub status: Option<u16>,
}

impl ClientStagedArtifactResponse {
    pub fn new(staged_artifact: StagedArtifact, status: Option<u16>) -> Self {
        return Self {
            staged_artifact,
            status,
        }
    }
}