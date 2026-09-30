use crate::client::Client;
use async_trait;
use serde::Serialize;
use shared::domain::entities;
use shared::presentation::http::v1::requests::{
    artifacts, discover_models, get_model_by_platform, ingest_model, list_models_by_platform,
};
use std::path::PathBuf;

// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::responses::ClientJsonResponse;

#[async_trait::async_trait]
pub trait ListModelsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_models(
        &self,
        _request: &list_models_by_platform::ListModelsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetModelClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_model(
        &self,
        _request: &get_model_by_platform::GetModelByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait IngestModelClient: Client {
    async fn ingest_model(
        &self,
        _request: &ingest_model::IngestModelRequest,
        _ingest_path: PathBuf,
    ) -> Result<(), ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait DiscoverModelsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn discover_models(
        &self,
        _request: &discover_models::DiscoverModelsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishModelArtifactClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_model_artifact(
        &self,
        _extracted_artifact_path: &PathBuf,
        _artifact: &entities::artifact::Artifact,
        _metadata: Option<&entities::model::Model>,
        _request: &artifacts::PublishArtifactServiceRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait PublishModelClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_model(
        &self,
        _model: &entities::model::Model,
        _request: &artifacts::PublishArtifactServiceRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

/// Converts platform-specific metadata into an MLHub external model.
pub trait ModelConversionClient: Client {
    fn from_platform_metadata<T>(
        &self,
        _metadata: T,
    ) -> Result<entities::model::external_model::ExternalModel, ClientError>
    where
        T: Serialize,
    {
        return Err(ClientError::Unimplemented);
    }

    fn to_platform_metadata<T>(
        &self,
        _metadata: entities::model::external_model::ExternalModel,
    ) -> Result<T, ClientError>
    where
        T: Serialize,
    {
        return Err(ClientError::Unimplemented);
    }
}
