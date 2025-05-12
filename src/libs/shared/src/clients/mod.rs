pub mod responses;
pub mod artifacts;

use crate::inference::presentation::http::v1::dto as inference;
use crate::training::presentation::http::v1::dto as training;
use crate::models::presentation::http::v1::dto as models;
use crate::datasets::presentation::http::v1::dto as datasets;
use crate::clients::artifacts::ArtifactGenerator;
use serde::Serialize;
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::clients::responses::{ClientJsonResponse, ClientStagedArtifactResponse};



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