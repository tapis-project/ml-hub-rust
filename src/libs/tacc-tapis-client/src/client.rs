// use crate::operations::files::{
//     MkdirResponse,
//     mkdir,
//     // insert
// };
// use crate::utils::token_from_headers;
// use crate::tokens::decode_jwt;
use std::path::PathBuf;
use async_trait;
use platforms::Platform;
use serde_json::Value;
use clients::{
    Capability, Client, ClientError, ClientJsonResponse, PublishModelClient
    // ClientErrorScope
};
use shared::domain::entities::{
    artifact::Artifact,
    model_metadata::ModelMetadata
};
use shared::presentation::http::v1::requests::artifacts;
use shared::logging::SharedLogger;

#[derive(Debug)]
pub struct TapisClient {
    logger: SharedLogger
}

#[async_trait::async_trait]
impl Client for TapisClient {
    fn platform(&self) -> Option<Platform> {
        Some(Platform::TaccTapis)
    }
    
    fn capabilities(&self) -> Option<Vec<Capability>> {
        Some(vec![])
    }
}

#[async_trait::async_trait]
impl PublishModelClient for TapisClient {
    type Data = Value;
    type Metadata = Value;

    async fn publish_model(&self, extracted_artifact_path: &PathBuf, _artifact: &Artifact, metadata: &ModelMetadata, request: &artifacts::PublishArtifactServiceRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

impl TapisClient {
    pub fn new() -> Self {
        Self {
            logger: SharedLogger::new(),
        }
    }
}
