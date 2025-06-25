use std::path::PathBuf;
use serde_json::Value;
use clients::{
    ClientError, ClientErrorScope, ClientJsonResponse, ClientStagedArtifactResponse, IngestModelClient as _
};
use shared::models::presentation::http::v1::dto::{
    DiscoverModelsRequest, GetModelRequest, IngestModelRequest, ListModelsRequest, PublishModelRequest
};
use huggingface_client::client::HuggingFaceClient;
use github_lfs_client::client::GithubLfsClient;
use git_lfs_client::client::GitLfsClient;
use patra_client::client::PatraClient;

pub enum ListModelsClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient)
}

impl clients::ListModelsClient for ListModelsClient {
    type Data = Value;
    type Metadata = Value;

    fn list_models(&self, request: &ListModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>{
        let resp: ClientJsonResponse<Value, Value> = match self {
            ListModelsClient::HuggingFace(c) => c.list_models(request)?,
            ListModelsClient::Patra(c) => c.list_models(request)?,
        };

        Ok(resp)
    }
}

pub enum GetModelClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient)
}

impl clients::GetModelClient for GetModelClient {
    type Data = Value;
    type Metadata = Value;

    fn get_model(&self, request: &GetModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>{
        let resp: ClientJsonResponse<Value, Value> = match self {
            GetModelClient::HuggingFace(c) => c.get_model(request)?,
            GetModelClient::Patra(c) => c.get_model(request)?,
        };

        Ok(resp)
    }
}

pub enum IngestModelClient {
    Github(GithubLfsClient),
    Git(GitLfsClient),
    HuggingFace(HuggingFaceClient)
}

impl IngestModelClient {
    pub fn ingest_model(&self, request: &IngestModelRequest, target_path: PathBuf) -> Result<ClientStagedArtifactResponse, ClientError> {
        match self {
            IngestModelClient::HuggingFace(c) => c.ingest_model(request, target_path),
            IngestModelClient::Git(c) => c.ingest_model(request, target_path),
            IngestModelClient::Github(c) => c.ingest_model(request, target_path)
        }
    }
}

pub enum DiscoverModelsClient {
    Patra(PatraClient)
}

impl clients::DiscoverModelsClient for DiscoverModelsClient {
    type Data = Value;
    type Metadata = Value;
    fn discover_models(&self, request: &DiscoverModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp = match self {
            DiscoverModelsClient::Patra(c) => c.discover_models(request)?
        };

        return Ok(resp);
    }
}

pub enum PublishModelClient {
}

impl clients::PublishModelClient for PublishModelClient {
    type Data = Value;
    type Metadata = Value;
    fn publish_model(&self, _request: &PublishModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: Result<_, ClientError> = match self {
            _ => Err(ClientError::NotFound { msg: "No clients available for publishing".into(), scope: ClientErrorScope::Client })
        };

        resp
    }
}