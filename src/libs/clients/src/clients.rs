use std::path::PathBuf;
use shared::inference::presentation::http::v1::dto as inference;
use shared::training::presentation::http::v1::dto as training;
use shared::models::presentation::http::v1::dto as models;
use shared::datasets::presentation::http::v1::dto as datasets;
use crate::artifacts::ArtifactGenerator;
use serde::Serialize;
// Re-exporting here to make the api cleaner and more predictable. Everything
// clients needs to implement should come from this module. Removing the 'pub'
// keyword below will break this modules api for consumers
pub use crate::errors::ClientError;
pub use crate::responses::{ClientJsonResponse, ClientStagedArtifactResponse};

pub trait ListModelsClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn list_models(&self, _request: &models::ListModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait GetModelClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn get_model(&self, _request: &models::GetModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait IngestModelClient: ArtifactGenerator {
    fn ingest_model(&self, _request: &models::IngestModelRequest, _target_path: PathBuf) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait DownloadModelClient: ArtifactGenerator {
    fn download_model(&self, _request: &models::DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait DiscoverModelsClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn discover_models(&self, _request: &models::DiscoverModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait PublishModelClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn publish_model(&self, _request: &models::PublishModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait ListDatasetsClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn list_datasets(&self, _request: &datasets::ListDatasetsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait GetDatasetClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn get_dataset(&self, _request: &datasets::GetDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait IngestDatasetClient: ArtifactGenerator {
    fn ingest_dataset(&self, _request: &datasets::IngestDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait DownloadDatasetClient: ArtifactGenerator {
    fn download_dataset(&self, _request: &datasets::DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait PublishDatasetClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn publish_dataset(&self, _request: &datasets::PublishDatasetRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait CreateInferenceServerClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn create_inference_server(&self, _request: &inference::CreateInferenceServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

pub trait CreateTrainingServerClient {
    type Data: Serialize;
    type Metadata: Serialize;

    fn create_training_server(&self, _request: &training::CreateTrainingServerRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}