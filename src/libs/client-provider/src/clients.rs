use async_trait;
use clients::{ClientError, ClientErrorScope, ClientJsonResponse, IngestModelClient as _};
use git_lfs_client::client::GitLfsClient;
use github_lfs_client::client::GithubLfsClient;
use huggingface_client::client::HuggingFaceClient;
use patra_client::client::PatraClient;
use serde_json::Value;
use shared::presentation::http::v1::dto::models::{
    DiscoverModelsRequest,
    GetModelRequest,
    IngestModelRequest,
    ListModelsRequest,
};
use shared::presentation::http::v1::dto::artifacts::PublishArtifactRequest;
use std::path::PathBuf;

pub enum ListModelsClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient),
}

#[async_trait::async_trait]
impl clients::ListModelsClient for ListModelsClient {
    type Data = Value;
    type Metadata = Value;

    async fn list_models(
        &self,
        request: &ListModelsRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: ClientJsonResponse<Value, Value> = match self {
            ListModelsClient::HuggingFace(c) => c.list_models(request).await?,
            ListModelsClient::Patra(c) => c.list_models(request).await?,
        };

        Ok(resp)
    }
}

pub enum GetModelClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient),
}

#[async_trait::async_trait]
impl clients::GetModelClient for GetModelClient {
    type Data = Value;
    type Metadata = Value;

    async fn get_model(
        &self,
        request: &GetModelRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: ClientJsonResponse<Value, Value> = match self {
            GetModelClient::HuggingFace(c) => c.get_model(request).await?,
            GetModelClient::Patra(c) => c.get_model(request).await?,
        };

        Ok(resp)
    }
}

pub enum IngestModelClient {
    Github(GithubLfsClient),
    Git(GitLfsClient),
    HuggingFace(HuggingFaceClient),
}

impl IngestModelClient {
    pub async fn ingest_model(
        &self,
        request: &IngestModelRequest,
        ingest_path: PathBuf,
    ) -> Result<(), ClientError> {
        match self {
            IngestModelClient::HuggingFace(c) => c.ingest_model(request, ingest_path).await,
            IngestModelClient::Git(c) => c.ingest_model(request, ingest_path).await,
            IngestModelClient::Github(c) => c.ingest_model(request, ingest_path).await,
        }
    }
}

pub enum IngestDatasetClient {
    
}

impl IngestDatasetClient {
    pub async fn ingest_dataset(
        &self,
        _request: &IngestModelRequest,
        _ingest_path: PathBuf,
    ) -> Result<(), ClientError> {
        Err(ClientError::Unimplemented)
    }
}

pub enum DiscoverModelsClient {
    Patra(PatraClient),
}

#[async_trait::async_trait]
impl clients::DiscoverModelsClient for DiscoverModelsClient {
    type Data = Value;
    type Metadata = Value;
    async fn discover_models(
        &self,
        request: &DiscoverModelsRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp = match self {
            DiscoverModelsClient::Patra(c) => c.discover_models(request).await?,
        };

        return Ok(resp);
    }
}

pub enum PublishModelClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient)
}

#[async_trait::async_trait]
impl clients::PublishModelClient for PublishModelClient {
    type Data = Value;
    type Metadata = Value;
    async fn publish_model(&self, _request: &PublishArtifactRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: Result<_, ClientError> = match self {
            _ => Err(ClientError::NotFound {
                msg: "No clients available for publishing".into(),
                scope: ClientErrorScope::Client,
            }),
        };

        resp
    }
}
