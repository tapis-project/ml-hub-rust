pub mod path;

use serde::{Deserialize, Serialize};
use serde_json::to_vec;

use std::collections::HashMap;

use crate::presentation::http::v1::requests::common::headers::Headers;
use crate::presentation::http::v1::requests::artifacts::IngestArtifactRequest;
use crate::application::inputs::artifacts as artifact_inputs;
use crate::errors::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestModelRequest {
    pub headers: Headers,
    pub path: path::IngestModelPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactRequest,
}

// Mapping to Application Inputs
impl TryFrom<IngestModelRequest> for artifact_inputs::IngestArtifactInput {
    type Error = Error;
    fn try_from(value: IngestModelRequest) -> Result<Self, Self::Error> {
        let serialized_client_request = to_vec(&value)
            .map_err(|err| Error::new(format!("Failed serialize the full client request: {}", err.to_string())))?;
        
        Ok(Self {
            artifact_type: artifact_inputs::ArtifactType::Model,
            platform: value.path.platform,
            platform_artifact_id: value.path.model_id,
            webhook_url: value.body.webhook_url,
            serialized_client_request
        })
    }
}