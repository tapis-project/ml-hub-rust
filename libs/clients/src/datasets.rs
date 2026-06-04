use std::path::PathBuf;
use shared::presentation::http::v1::requests::datasets;
use serde::Serialize;
use async_trait;
use crate::client::Client;
use crate::errors::ClientError;
use crate::responses::ClientJsonResponse;

#[async_trait::async_trait]
pub trait ListDatasetsClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn list_datasets(&self, _request: &datasets::ListDatasetsByPlatformRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

#[async_trait::async_trait]
pub trait GetDatasetClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn get_dataset(&self, _request: &datasets::GetDatasetByPlatformRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
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