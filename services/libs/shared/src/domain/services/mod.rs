pub mod deployment_strategy;

use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::artifact_ingestion::{ArtifactIngestion, ArtifactIngestionStatus};
use thiserror::Error;

use crate::domain::entities::artifact::ArtifactType;
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::entities::deployment::{ModelDeployment, ModelDeploymentProps};

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

    #[error("Invalid artifact type. Artifact must be of type 'Model'")]
    InvalidArtifactType
}

pub struct ModelMetadataService {}

impl ModelMetadataService {
    /// Verifies the the artifact exists and that the artifact has is fully
    /// ingested or uploaded
    pub fn associate_metadata_with_artifact<'a>(artifact: &Artifact, _metadata: ModelMetadata) -> Result<(), ModelMetadataServiceError> {
        if !artifact.is_fully_ingested() {
            return Err(ModelMetadataServiceError::ArtifactNotReady);
        }

        if artifact.artifact_type != ArtifactType::Model {
            return Err(ModelMetadataServiceError::InvalidArtifactType);
        }

        return Ok(());
    }
}

#[derive(Debug, Error)]
pub enum ModelDeploymentServiceError {
    #[error("Cannot create model deployment for model {0}/{1}. Artifact for the selected model must be fully ingested")]
    ArtifactIngestionRequired(String, String),

    #[error("Provided ModelMetadata and Artifact have different ids: Model metadata artifact id: {0}. Artifact id {1}")]
    MismatchedArtifactIds(String, String),

    #[error("The artifact associated with this deployment's model metadata is not a Model artifact")]
    InvalidArtifactType,
}

pub struct ModelDeploymentService {}

impl ModelDeploymentService {
    pub fn create_model_deployment(
        model_metadata: &ModelMetadata,
        // TODO Uncomment the line below when ready. Details found in the issue below 
        // https://github.com/tapis-project/ml-hub-rust/issues/73
        // artifact: &Artifact,
        props: ModelDeploymentProps
    ) -> Result<ModelDeployment, ModelDeploymentServiceError> {
        // TODO Uncomment all lines below when ready. Details found in the issue below 
        // https://github.com/tapis-project/ml-hub-rust/issues/73
        // if model_metadata.artifact_id.is_none() {
        //     return Err(ModelDeploymentServiceError::ArtifactIngestionRequired(props.model.author, props.model.name))
        // };

        // if model_metadata.artifact_id != Some(artifact.id) {
        //     return Err(ModelDeploymentServiceError::MismatchedArtifactIds(model_metadata.artifact_id.and_then(|id| Some(id.to_string())).unwrap_or(String::from("NULL")), artifact.id.to_string()))
        // };

        // if artifact.artifact_type != ArtifactType::Model {
        //     return Err(ModelDeploymentServiceError::InvalidArtifactType)
        // };
        
        // if !artifact.is_fully_ingested() {
        //     return Err(ModelDeploymentServiceError::ArtifactIngestionRequired(props.model.author, props.model.name))
        // };

        Ok(ModelDeployment::new(props))
    }
}