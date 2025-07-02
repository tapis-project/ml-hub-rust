use std::path::PathBuf;
use shared::inference::presentation::http::v1::dto as inference;
use shared::training::presentation::http::v1::dto as training;
use shared::models::presentation::http::v1::dto as models;
use shared::datasets::presentation::http::v1::dto as datasets;
use crate::artifacts::ArtifactGenerator;
use serde::Serialize;
use async_trait;
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::responses::{ClientJsonResponse, ClientStagedArtifactResponse};

#[async_trait::async_trait]
pub trait ListModelsClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_models(&self, _request: &models::ListModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetModelClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_model(&self, _request: &models::GetModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait IngestModelClient: ArtifactGenerator + Send + Sync {
    async fn ingest_model(&self, _request: &models::IngestModelRequest, _ingest_path: PathBuf) -> Result<(), ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait DiscoverModelsClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn discover_models(&self, _request: &models::DiscoverModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishModelClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_model(&self, _request: &models::PublishModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait ListDatasetsClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_datasets(&self, _request: &datasets::ListDatasetsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetDatasetClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_dataset(&self, _request: &datasets::GetDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait IngestDatasetClient: ArtifactGenerator + Send + Sync {
    async fn ingest_dataset(&self, _request: &datasets::IngestDatasetRequest,  _ingest_path: PathBuf) -> Result<(), ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait DownloadDatasetClient: ArtifactGenerator + Send + Sync {
    async fn download_dataset(&self, _request: &datasets::DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishDatasetClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_dataset(&self, _request: &datasets::PublishDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait CreateInferenceServerClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn create_inference_server(&self, _request: &inference::CreateInferenceServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait CreateTrainingServerClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn create_training_server(&self, _request: &training::CreateTrainingServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}