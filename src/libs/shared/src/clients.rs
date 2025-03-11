use crate::requests;
use crate::artifacts::{StagedArtifact, ArtifactGenerator};
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
use serde::Serialize;
use serde_json::Value;

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
    fn list_models(&self, request: &requests::ListModelsRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_model(&self, request: &requests::GetModelRequest) -> Result<ClientJsonResponse, ClientError>;
    fn download_model(&self, request: &requests::DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError>;
    fn discover_models(&self, request: &requests::DiscoverModelsRequest) -> Result<ClientJsonResponse, ClientError>;
}

pub trait DatasetsClient: ArtifactGenerator {
    fn list_datasets(&self, request: &requests::ListDatasetsRequest) -> Result<ClientJsonResponse, ClientError>;
    fn get_dataset(&self, request: &requests::GetDatasetRequest) -> Result<ClientJsonResponse, ClientError>;
    fn download_dataset(&self, request: &requests::DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError>;
}

pub trait InferenceClient {
    fn create_inference_server(&self, request: &requests::CreateInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn create_inference(&self, request: &requests::CreateInferenceRequest) -> Result<ClientJsonResponse, ClientError>;
    fn start_inference_server(&self, request: &requests::StartInferenceServerRequest) -> Result<ClientJsonResponse, ClientError>;
    fn run_inference(&self, request: &requests::RunInferenceRequest) -> Result<ClientJsonResponse, ClientError>;
}

pub trait TrainingClient {
    fn create_training(&self, request: &requests::CreateTrainingRequest) -> Result<ClientJsonResponse, ClientError>;
    fn start_training(&self, request: &requests::StartTrainingRequest) -> Result<ClientJsonResponse, ClientError>;
}