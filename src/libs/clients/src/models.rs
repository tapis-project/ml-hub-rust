use std::path::PathBuf;
use shared::presentation::http::v1::requests::models;
use shared::presentation::http::v1::requests::discover_models;
use shared::presentation::http::v1::requests::artifacts;
use shared::domain::entities;
use shared::application::inputs;
use serde::Serialize;
use async_trait;
use crate::client::Client;

// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::responses::ClientJsonResponse;

#[async_trait::async_trait]
pub trait ListModelsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_models(&self, _request: &models::ListModelsByPlatformRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetModelClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_model(&self, _request: &models::GetModelByPlatformRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait IngestModelClient: Client {
    async fn ingest_model(&self, _request: &models::IngestModelRequest, _ingest_path: PathBuf) -> Result<(), ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait DiscoverModelsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn discover_models(&self, _request: &discover_models::DiscoverModelsByPlatformRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishModelClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_model(
        &self,
        _extracted_artifact_path: &PathBuf,
        _artifact: &entities::artifact::Artifact,
        _metadata: Option<&entities::model_metadata::ModelMetadata>,
        _request: &artifacts::PublishArtifactServiceRequest
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishModelMetadataClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_model_metadata(&self, _metadata: &entities::model_metadata::ModelMetadata, _request: &artifacts::PublishArtifactServiceRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

/// Converts platform specific metadata into MLHub model metadata
pub trait ModelMetadataConversionClient: Client {
    fn from_platform_metadata<T>(&self, _metadata: T) -> Result<inputs::model_metadata::ModelMetadata, ClientError>
        where T: Serialize
    {
        return Err(ClientError::Unimplemented);
    }

    fn to_platform_metadata<T>(&self, _metadata: inputs::model_metadata::ModelMetadata) -> Result<T, ClientError>
        where T: Serialize
    {
        return Err(ClientError::Unimplemented);
    }
}