use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::artifact_ingestion::{ArtifactIngestion, ArtifactIngestionStatus};
use thiserror::Error;

use super::entities::model_metadata::ModelMetadata;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("{0}")]
    InvalidIngestionState(String)
}

pub struct ArtifactService {}

impl ArtifactService {
    /// Adds the final path of the ingestion to the artifact
    pub fn finish_artifact_ingestion<'a>(artifact: &'a mut Artifact, ingestion: &ArtifactIngestion) -> Result<&'a mut Artifact, ArtifactServiceError> {
        if ingestion.status != ArtifactIngestionStatus::Finished {
            return Err(ArtifactServiceError::InvalidIngestionState("Artifact ingestion must be Finished before setting the download url of an artifact".into()))
        }

        match &ingestion.artifact_path {
            Some(path) => {
                artifact.set_path(path.clone());
                Ok(artifact)
            },
            None => {
                Err(ArtifactServiceError::InvalidIngestionState("Cannot set the artifact's path because the ingestion is missing a value for field artifact_path".into()))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ModelMetadataServiceError {
    #[error("Cannot create metadata for an artifact that is not fully ingested")]
    ArtifactNotReady,
}

pub struct ModelMetadataService {}

impl ModelMetadataService {
    /// Verifies the the artifact exists and that the artifact has is fully
    /// ingested or uploaded
    pub fn create<'a>(artifact: &Artifact, _metadata: ModelMetadata) -> Result<(), ModelMetadataServiceError> {
        if !artifact.is_fully_ingested() {
            return Err(ModelMetadataServiceError::ArtifactNotReady);
        }

        return Ok(());
    }
}