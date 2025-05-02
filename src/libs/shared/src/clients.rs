use crate::inference::web::dto as inference;
use crate::training::web::dto as training;
use crate::models::web::dto as models;
use crate::datasets::web::dto as datasets;
use crate::artifacts::{ArtifactGenerator, StagedArtifact};
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
use serde::Serialize;
use serde_json::Value;
// use std::future::Future;

#[derive(Serialize)]
pub struct ClientJsonResponse {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>
}

impl ClientJsonResponse {
    pub fn new(status: Option<u16>, message: Option<String>, result: Option<Value>, metadata: Option<Value>) -> Self {
        return Self {
            status,
            message,
            result,
            metadata
        }
    }
}

pub struct ClientStagedArtifactResponse {
    pub staged_artifact: StagedArtifact,
    pub status: Option<u16>,
}

impl ClientStagedArtifactResponse {
    pub fn new(staged_artifact: StagedArtifact, status: Option<u16>) -> Self {
        return Self {
            staged_artifact,
            status,
        }
    }
}

pub trait ModelsClient: ArtifactGenerator {
    fn list_models(&self, request: &models::ListModelsRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_model(&self, request: &models::GetModelRequest) -> Result<ClientJsonResponse, ClientError>;
    fn download_model(&self, request: &models::DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError>;
    fn discover_models(&self, request: &models::DiscoverModelsRequest) -> Result<ClientJsonResponse, ClientError>;
    fn publish_model(&self, request: &models::PublishModelRequest) -> Result<ClientJsonResponse, ClientError>;
    // fn publish_model(&self, request: &requests::PublishModelRequest) -> impl Future<Output=Result<ClientJsonResponse, ClientError>>;
}

pub trait DatasetsClient: ArtifactGenerator {
    fn list_datasets(&self, request: &datasets::ListDatasetsRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_dataset(&self, request: &datasets::GetDatasetRequest) -> Result<ClientJsonResponse, ClientError>;
    fn download_dataset(&self, request: &datasets::DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError>;
    fn publish_dataset(&self, request: &datasets::PublishDatasetRequest) -> Result<ClientJsonResponse, ClientError>;
}

pub trait InferenceClient {
    fn create_inference_server(&self, request: &inference::CreateInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn deploy_inference_server(&self, request: &inference::CreateInferenceRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_inference_server_docs(&self, request: &inference::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_inference_server_interface(&self, request: &inference::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn list_inference_server_interfaces(&self, request: &inference::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn run_inference(&self, request: &inference::RunInferenceRequest) -> Result<ClientJsonResponse, ClientError>;
}

pub trait TrainingClient {
    fn create_training_server(&self, request: &training::CreateTrainingRequest) -> Result<ClientJsonResponse, ClientError>;
    fn deploy_training_server(&self, request: &inference::CreateInferenceRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_training_server_docs(&self, request: &inference::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_training_server_interface(&self, request: &inference::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn list_training_server_interfaces(&self, request: &inference::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn start_training(&self, request: &training::StartTrainingRequest) -> Result<ClientJsonResponse, ClientError>;
}