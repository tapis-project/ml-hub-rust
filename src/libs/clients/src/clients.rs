use serde_json::Value;
use shared::clients::{
    ClientError,
    ClientJsonResponse,
    ClientStagedArtifactResponse,
    DownloadModelClient as _
};
// use shared::artifacts::{
//     ArtifactGenerator,
// };
use shared::models::web::v1::dto::{
    DiscoverModelsRequest, DownloadModelRequest, GetModelRequest, ListModelsRequest, PublishModelRequest
};
use huggingface_client::client::HuggingFaceClient;
use github_lfs_client::client::GithubLfsClient;
use git_lfs_client::client::GitLfsClient;
use patra_client::client::PatraClient;

pub enum ListModelsClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient)
}

impl shared::clients::ListModelsClient for ListModelsClient {
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

impl shared::clients::GetModelClient for GetModelClient {
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

pub enum DownloadModelClient {
    Github(GithubLfsClient),
    Git(GitLfsClient),
    HuggingFace(HuggingFaceClient)
}

impl DownloadModelClient {
    pub fn download_model(&self, request: &DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        match self {
            DownloadModelClient::HuggingFace(c) => c.download_model(request),
            DownloadModelClient::Git(c) => c.download_model(request),
            DownloadModelClient::Github(c) => c.download_model(request)
        }
    }
}

pub enum DiscoverModelsClient {
    Patra(PatraClient)
}

impl shared::clients::DiscoverModelsClient for DiscoverModelsClient {
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

impl shared::clients::PublishModelClient for PublishModelClient {
    type Data = Value;
    type Metadata = Value;
    fn publish_model(&self, _request: &PublishModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: Result<_, ClientError> = match self {
            _ => Err(ClientError::new(String::from("No clients available for publishing")))
        };

        resp
    }
}