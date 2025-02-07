use crate::requests;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ClientResponse {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>,
}

#[derive(Debug)]
pub struct ClientError {
    message: String
}

impl ClientError {
    pub fn new(message: String) -> Self {
        ClientError {
            message,
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.message)
    }
}

impl Error for ClientError {}

pub trait ModelsClient {
    fn list_models(&self, request: &requests::ListModelsRequest) -> Result<ClientResponse, ClientError>;
    fn get_model(&self, request: &requests::GetModelRequest) -> Result<ClientResponse, ClientError>;
    fn download_model(&self, request: &requests::DownloadModelRequest) -> Result<ClientResponse, ClientError>;
}

pub trait DatasetsClient {
    fn list_datasets(&self, request: &requests::ListDatasetsRequest) -> Result<ClientResponse, ClientError>;
    fn get_dataset(&self, request: &requests::GetDatasetRequest) -> Result<ClientResponse, ClientError>;
    fn download_dataset(&self, request: &requests::DownloadDatasetRequest) -> Result<ClientResponse, ClientError>;
}

pub trait InferenceClient {
    fn create_inference(&self, request: &requests::CreateInferenceRequest) -> Result<ClientResponse, ClientError>;
    fn start_inference_server(&self, request: &requests::StartInferenceServerRequest) -> Result<ClientResponse, ClientError>;
    fn run_inference(&self, request: &requests::RunInferenceRequest) -> Result<ClientResponse, ClientError>;
}

pub trait TrainingClient {
    fn create_training(&self, request: &requests::CreateTrainingRequest) -> Result<ClientResponse, ClientError>;
    fn start_training(&self, request: &requests::StartTrainingRequest) -> Result<ClientResponse, ClientError>;
}


