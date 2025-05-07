use reqwest::blocking::Client as ReqwestClient;
use shared::artifacts::ArtifactGenerator;
use shared::clients::{
    ClientStagedArtifactResponse,
    ClientError,
    ClientJsonResponse,
    DatasetsClient,
    ModelsClient,
};
use shared::models::web::v1::dto::{
    DiscoverModelsRequest,
    DownloadModelRequest,
    GetModelRequest,
    ListModelsRequest,
    PublishModelRequest,
};
use shared::datasets::web::v1::dto::{
    DownloadDatasetRequest,
    GetDatasetRequest,
    ListDatasetsRequest,
    PublishDatasetRequest,
};
// use shared::requests::param_to_string;
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

    fn publish_model(&self, _request: &PublishModelRequest) -> Result<ClientJsonResponse, ClientError> {
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

    fn publish_dataset(&self, _request: &PublishDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
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
