use std::path::PathBuf;
use shared::presentation::http::v1::dto::inference;
use shared::presentation::http::v1::dto::training;
use shared::presentation::http::v1::dto::models;
use shared::presentation::http::v1::dto::datasets;
use shared::presentation::http::v1::dto::artifacts;
use shared::domain::entities;
use serde::Serialize;
use async_trait;
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::responses::ClientJsonResponse;

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
pub trait IngestModelClient: Send + Sync {
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

    async fn publish_model(
        &self,
        _extracted_artifact_path: &PathBuf,
        _artifact: &entities::artifact::Artifact,
        _metadata: &entities::model_metadata::ModelMetadata,
        _request: &artifacts::PublishArtifactRequest
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishModelMetadataClient: Send + Sync {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_model_metadata(&self, _metadata: &entities::model_metadata::ModelMetadata, _request: &artifacts::PublishArtifactRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
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
pub trait IngestDatasetClient: Send + Sync {
    async fn ingest_dataset(&self, _request: &datasets::IngestDatasetRequest,  _ingest_path: PathBuf) -> Result<(), ClientError> {
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