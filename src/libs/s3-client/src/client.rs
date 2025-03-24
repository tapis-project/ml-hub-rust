use reqwest::blocking::Client as ReqwestClient;
use shared::artifacts::ArtifactGenerator;
use shared::clients::{
    ClientStagedArtifactResponse,
    ClientError,
    ClientJsonResponse,
    DatasetsClient,
    ModelsClient,
};
use shared::requests::{
    GetModelRequest,
    ListModelsRequest,
    DownloadModelRequest,
    ListDatasetsRequest,
    GetDatasetRequest,
    DownloadDatasetRequest,
    DiscoverModelsRequest,
    PersistDatasetRequest,
    PersistModelRequest
};
use shared::logging::SharedLogger;

#[derive(Debug)]
pub struct S3Client {
    client: ReqwestClient,
    logger: SharedLogger
}

impl ArtifactGenerator for S3Client {}

impl ModelsClient for S3Client {
    fn list_models(
        &self,
        _request: &ListModelsRequest,
    ) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }
    
    fn get_model(&self, _request: &GetModelRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn download_model(&self, _request: &DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn discover_models(&self, _request: &DiscoverModelsRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Discover models not implemented")))
    }

    fn persist_model(&self, _request: &PersistModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }
}

impl DatasetsClient for S3Client {
    fn list_datasets(&self, _request: &ListDatasetsRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn get_dataset(&self, _request: &GetDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn download_dataset(&self, _request: &DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn persist_dataset(&self, _request: &PersistDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }
}

impl S3Client {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
            logger: SharedLogger::new(),
        }
    }
}
