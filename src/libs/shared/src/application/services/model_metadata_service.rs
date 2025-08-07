use std::sync::Arc;
use crate::retry::{retry_async, RetryPolicy, FixedBackoff, Retry};
use crate::application::errors::ApplicationError;
use crate::application::ports::repositories::{ArtifactRepository, ModelMetadataRepository};
use crate::application::inputs::model_metadata::CreateModelMetadata;
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::services::{
    ModelMetadataService as ModelMetadataDomainService,
    ModelMetadataServiceError as ModelMetadataDomainServiceError
};
use thiserror::Error;
use once_cell::sync::Lazy;
// use crate::logging::GlobalLogger;

#[derive(Debug, Error)]
pub enum ModelMetadataServiceError {
    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Artifact not found: {0}")]
    ArtifactNotFound(String),

    #[error("{0}")]
    DomainServiceError(#[from] ModelMetadataDomainServiceError),
}

pub struct ModelMetadataService {
    model_metadata_repo: Arc<dyn ModelMetadataRepository>,
    artifact_repo: Arc<dyn ArtifactRepository>
}

impl ModelMetadataService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    pub fn new(
        model_metadata_repo: Arc<dyn ModelMetadataRepository>,
        artifact_repo: Arc<dyn ArtifactRepository>
    ) -> Self {
        Self {
            model_metadata_repo,
            artifact_repo
        }
    }

    pub async fn create_metadata(&self, input: CreateModelMetadata) -> Result<(), ModelMetadataServiceError> {
        // Get the artifact_id from the input
        let artifact_id = input.artifact_id.clone();

        // Convert from service input to domain entitiy
        let metadata = ModelMetadata::try_from(input.clone())?;

        let find_artifact = || self.artifact_repo.find_by_id(artifact_id.clone());

        // Find the artifact by id
        let artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY)
            .await?
            .ok_or_else(|| ModelMetadataServiceError::ArtifactNotFound(format!("Artifact with id {} does not exist", &artifact_id)))?;

        // Determine if we are allowed to create the metadata for this artifact
        ModelMetadataDomainService::create(&artifact, metadata)?;

        let create_metadata = || self.model_metadata_repo.save(&input);

        retry_async(create_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        return Ok(())
    }
}
