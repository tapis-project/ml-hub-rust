use std::path::PathBuf;

use serde::Serialize;

use crate::client::Client;
use crate::errors::ClientError;
use crate::responses::ClientJsonResponse;
use shared::presentation::http::v1::requests::{
    get_dataset_by_platform::GetDatasetByPlatformRequest, ingest_dataset::IngestDatasetRequest,
    list_datasets_by_platform::ListDatasetsByPlatformRequest,
    publish_dataset::PublishDatasetRequest,
};

#[async_trait::async_trait]
pub trait ListDatasetsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_datasets(
        &self,
        _request: &ListDatasetsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        Err(ClientError::Unimplemented)
    }
}

#[async_trait::async_trait]
pub trait GetDatasetClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_dataset(
        &self,
        _request: &GetDatasetByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        Err(ClientError::Unimplemented)
    }
}

#[async_trait::async_trait]
pub trait IngestDatasetClient: Client {
    async fn ingest_dataset(
        &self,
        _request: &IngestDatasetRequest,
        _ingest_path: PathBuf,
    ) -> Result<(), ClientError> {
        Err(ClientError::Unimplemented)
    }
}

#[async_trait::async_trait]
pub trait PublishDatasetClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn publish_dataset(
        &self,
        _request: &PublishDatasetRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        Err(ClientError::Unimplemented)
    }
}
