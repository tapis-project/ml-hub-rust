use std::path::PathBuf;
use shared::presentation::http::v1::requests::inference;
use shared::presentation::http::v1::requests::training;
use shared::presentation::http::v1::requests::models;
use shared::presentation::http::v1::requests::datasets;
use shared::presentation::http::v1::requests::artifacts;
use shared::domain::entities;
use serde::Serialize;
use async_trait;
use strum_macros::{EnumString, EnumIter, Display};

// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::responses::ClientJsonResponse;
use platforms::Platform;

#[derive(Eq, PartialEq, EnumIter, EnumString, Display)]
pub enum Capability {
    ListModels,
    GetModel,
    IngestModel,
    DiscoverModels,
    PublishModel,
    PublishModelMetadata,
    ListDatasets,
    GetDataset,
    IngestDataset,
    DiscoverDatasets,
    PublishDataset,
    PublishDatasetMetadata
}

#[async_trait::async_trait]
pub trait Client: Send + Sync {
    /// Returns the platform platform this client belongs to
    fn platform(&self) -> Option<Platform>;

    /// Lists the capabilities of the client
    fn capabilities(&self) -> Option<Vec<Capability>>;

    /// Determines if a client as a capability
    fn has_capability(&self, capability: &Capability) -> bool {
        if let Some(capabilities) = self.capabilities() {
            return capabilities.contains(capability)
        }

        return false
    }
}

#[async_trait::async_trait]
pub trait ListModelsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_models(&self, _request: &models::ListModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetModelClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_model(&self, _request: &models::GetModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
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

    async fn discover_models(&self, _request: &models::DiscoverModelsByPlatformRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
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

#[async_trait::async_trait]
pub trait ListDatasetsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_datasets(&self, _request: &datasets::ListDatasetsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetDatasetClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_dataset(&self, _request: &datasets::GetDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait IngestDatasetClient: Client {
    async fn ingest_dataset(&self, _request: &datasets::IngestDatasetRequest,  _ingest_path: PathBuf) -> Result<(), ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishDatasetClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_dataset(&self, _request: &datasets::PublishDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait CreateInferenceServerClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn create_inference_server(&self, _request: &inference::CreateInferenceServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait CreateTrainingServerClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn create_training_server(&self, _request: &training::CreateTrainingServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}