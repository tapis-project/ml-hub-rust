use async_trait;
use clients::{
    Client, 
    Capability, 
    ClientError, 
    ClientJsonResponse, 
    IngestModelClient as _,
    ModelConversionClient as _,
};
use git_lfs_client::client::GitLfsClient;
use github_lfs_client::client::GithubLfsClient;
use huggingface_client::client::HuggingFaceClient;
use patra_client::client::PatraClient;
use serde_json::Value;
use shared::presentation::http::v1::requests::{
    get_dataset_by_platform::GetDatasetByPlatformRequest,
    get_model_by_platform::GetModelByPlatformRequest,
    ingest_model::IngestModelRequest,
    list_datasets_by_platform::ListDatasetsByPlatformRequest,
    list_models_by_platform::ListModelsByPlatformRequest,
};
use shared::presentation::http::v1::requests::discover_models::DiscoverModelsByPlatformRequest;
use shared::presentation::http::v1::requests::artifacts::PublishArtifactServiceRequest;
use shared::domain::entities::artifact::Artifact;
use shared::domain::entities::model::Model;
use std::path::PathBuf;

pub enum ListModelsClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient),
}

impl ListModelsClient {
    const CAPABILITY: Capability = Capability::ListModels;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for ListModelsClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    
        fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::ListModelsClient for ListModelsClient {
    type Data = Value;
    type Metadata = Value;

    async fn list_models(
        &self,
        request: &ListModelsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: ClientJsonResponse<Value, Value> = match self {
            ListModelsClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.list_models(request).await?
            },
            ListModelsClient::Patra(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.list_models(request).await?
            },
        };

        Ok(resp)
    }
}

pub enum GetModelClient {
    HuggingFace(HuggingFaceClient),
    Patra(PatraClient),
}

impl GetModelClient {
    const CAPABILITY: Capability = Capability::GetModel;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for GetModelClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::GetModelClient for GetModelClient {
    type Data = Value;
    type Metadata = Value;

    async fn get_model(
        &self,
        request: &GetModelByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: ClientJsonResponse<Value, Value> = match self {
            GetModelClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.get_model(request).await?
            },
            GetModelClient::Patra(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.get_model(request).await?
            },
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
    const CAPABILITY: Capability = Capability::IngestModel;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for IngestModelClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

impl IngestModelClient {
    pub async fn ingest_model(
        &self,
        request: &IngestModelRequest,
        ingest_path: PathBuf,
    ) -> Result<(), ClientError> {
        match self {
            IngestModelClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.ingest_model(request, ingest_path).await
            }
            IngestModelClient::Git(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.ingest_model(request, ingest_path).await
            }
            IngestModelClient::Github(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.ingest_model(request, ingest_path).await
            }
        }
    }
}


pub enum DiscoverModelsClient {
    Patra(PatraClient),
}

impl DiscoverModelsClient {
    const CAPABILITY: Capability = Capability::DiscoverModels;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for DiscoverModelsClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::DiscoverModelsClient for DiscoverModelsClient {
    type Data = Value;
    type Metadata = Value;
    async fn discover_models(
        &self,
        request: &DiscoverModelsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp = match self {
            DiscoverModelsClient::Patra(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }
                
                c.discover_models(request).await?
            }
        };
        
        return Ok(resp);
    }
}

pub enum PublishModelArtifactClient {
    HuggingFace(HuggingFaceClient),
}

impl PublishModelArtifactClient {
    const CAPABILITY: Capability = Capability::PublishModelArtifact;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for PublishModelArtifactClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::PublishModelArtifactClient for PublishModelArtifactClient {
    type Data = Value;
    type Metadata = Value;
    async fn publish_model_artifact(&self, extracted_artifact_path: &PathBuf, artifact: &Artifact, model: Option<&Model>, request: &PublishArtifactServiceRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> = match self {
            PublishModelArtifactClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }
                
                c.publish_model_artifact(extracted_artifact_path, artifact, model, request).await
            }
        };
        
        resp
    }
}

pub enum PublishModelClient {
    Patra(PatraClient)
}

impl PublishModelClient {
    const CAPABILITY: Capability = Capability::PublishModel;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for PublishModelClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::PublishModelClient for PublishModelClient {
    type Data = Value;
    type Metadata = Value;
    
    async fn publish_model(&self, model: &Model, request: &PublishArtifactServiceRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> = match self {
            PublishModelClient::Patra(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }
                
                c.publish_model(model, request).await
            }
        };
        
        resp
    }
}

pub enum ModelConversionClient {
    HuggingFace(HuggingFaceClient)
}

impl ModelConversionClient {
    const CAPABILITY: Capability = Capability::ConvertModel;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for ModelConversionClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

impl ModelConversionClient {
    pub fn from_platform_metadata<T>(&self, metadata: T, author: String, tenant_id: String) -> Result<shared::domain::entities::model::Model, ClientError>
    where T: serde::Serialize
    {
        let resp = match self {
            ModelConversionClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }
                
                
                c.from_platform_metadata(metadata, author, tenant_id)
            }
        };
        
        resp
    }
    
    pub fn to_platform_metadata<T>(&self, _metadata: shared::domain::entities::model::Model) -> Result<T, ClientError>
    where T: serde::Serialize 
    {
        Err(ClientError::Unimplemented)
    }
}

pub enum ListDatasetsClient {
    HuggingFace(HuggingFaceClient),
}

impl ListDatasetsClient {
    const CAPABILITY: Capability = Capability::ListDatasets;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for ListDatasetsClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::ListDatasetsClient for ListDatasetsClient {
    type Data = Value;
    type Metadata = Value;

    async fn list_datasets(
        &self,
        request: &ListDatasetsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: ClientJsonResponse<Value, Value> = match self {
            ListDatasetsClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.list_datasets(request).await?
            }
        };

        Ok(resp)
    }
}

pub enum GetDatasetClient {
    HuggingFace(HuggingFaceClient),
}

impl GetDatasetClient {
    const CAPABILITY: Capability = Capability::GetDataset;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for GetDatasetClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
}

#[async_trait::async_trait]
impl clients::GetDatasetClient for GetDatasetClient {
    type Data = Value;
    type Metadata = Value;

    async fn get_dataset(
        &self,
        request: &GetDatasetByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let resp: ClientJsonResponse<Value, Value> = match self {
            GetDatasetClient::HuggingFace(c) => {
                if !c.has_capability(&Self::CAPABILITY) {
                    return Err(ClientError::Unimplemented)
                }

                c.get_dataset(request).await?
            }
        };

        Ok(resp)
    }
}

pub enum IngestDatasetClient {
    
}

impl IngestDatasetClient {
    const CAPABILITY: Capability = Capability::IngestDataset;
}

// This impl for the enum is merely to satisfy the compiler
impl Client for IngestDatasetClient {
    fn platform(&self) -> Option<platforms::Platform> { None }
    fn capabilities(&self) -> Option<Vec<Capability>> { None }
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
