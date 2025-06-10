use crate::common::domain::entities::{Artifact, ArtifactIngestion, ArtifactIngestionStatus};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("{0}")]
    InvalidIngestionState(String)
}

pub struct ArtifactService {}

impl ArtifactService {
    /// Adds the final path of the ingestion to the artifact
    pub fn finish_artifact<'a>(artifact: &'a mut Artifact, ingestion: &ArtifactIngestion) -> Result<&'a mut Artifact, ArtifactServiceError> {
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