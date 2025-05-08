use crate::inference::web::v1::dto as inference;
use crate::training::web::v1::dto as training;
use crate::models::web::v1::dto as models;
use crate::datasets::web::v1::dto as datasets;
use crate::artifacts::{ArtifactGenerator, StagedArtifact};
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientJsonResponse<Data: Serialize, Metadata: Serialize> {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Data>,
    pub metadata: Option<Metadata>
}

impl <Data: Serialize, Metadata: Serialize>ClientJsonResponse<Data, Metadata> {
    pub fn new(status: Option<u16>, message: Option<String>, result: Option<Data>, metadata: Option<Metadata>) -> Self {
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

pub trait ListModelsClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn list_models(&self, _request: &models::ListModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait GetModelClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn get_model(&self, _request: &models::GetModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait DownloadModelClient: ArtifactGenerator {
    fn download_model(&self, _request: &models::DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait DiscoverModelsClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn discover_models(&self, _request: &models::DiscoverModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait PublishModelClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn publish_model(&self, _request: &models::PublishModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait ListDatasetsClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn list_datasets(&self, _request: &datasets::ListDatasetsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait GetDatasetClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn get_dataset(&self, _request: &datasets::GetDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait DownloadDatasetClient: ArtifactGenerator {
    fn download_dataset(&self, _request: &datasets::DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait PublishDatasetClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn publish_dataset(&self, _request: &datasets::PublishDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait CreateInferenceServerClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn create_inference_server(&self, _request: &inference::CreateInferenceServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}

pub trait CreateTrainingServerClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn create_training_server(&self, _request: &training::CreateTrainingServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::new(String::from("unimplemented")))
    }
}