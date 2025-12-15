use std::sync::Arc;
use crate::retry::{retry_async, RetryPolicy, FixedBackoff, Retry};
use crate::application::errors::ApplicationError;
use crate::application::ports::repositories::{ArtifactRepository, ModelMetadataRepository};
use crate::application::inputs::model_metadata::{AssociateModelMetadata, CreateModelMetadata, UpdateModelMetadataArtifactId};
use crate::application::inputs::discover_models::DiscoverModelsInput;
use crate::application::outputs::discover_models::DiscoverModelsOutput;
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

    #[error("Metadata not found: {0}")]
    MetdataNotFound(String),

    #[error("{0}")]
    DomainServiceError(#[from] ModelMetadataDomainServiceError),

    #[error("Metadata already exists for Artifact '{0}'")]
    DuplicateMetadataError(String),
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

    pub async fn associate_metadata_with_artifact(&self, input: AssociateModelMetadata) -> Result<(), ModelMetadataServiceError> {
        // Get the artifact_id from the input
        let artifact_id = input.artifact_id.clone();

        let find_artifact = || self.artifact_repo.get_by_id(&artifact_id);

        // Find the artifact by id
        let artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY)
            .await?
            .ok_or_else(|| ModelMetadataServiceError::ArtifactNotFound(format!("Artifact with id {} does not exist", &artifact_id)))?;

        // Ensure no metadata already exists for this artifact
        let find_metadata = || self.model_metadata_repo.find_by_artifact_id(&artifact_id);

        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        let metadata = match maybe_metadata {
            Some(m) => m,
            None => return Err(ModelMetadataServiceError::MetdataNotFound(format!("No metadata found with author {} and name {}", input.author, input.name)))
        };

        // Determine if we are allowed to create the metadata for this artifact
        ModelMetadataDomainService::associate_metadata_with_artifact(&artifact, metadata)?;

        let update_input = UpdateModelMetadataArtifactId::from(input);
        
        let update_metadata = || self.model_metadata_repo.update_artifact_id(&update_input);

        retry_async(update_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        return Ok(())
    }

    pub async fn create_model_metadata(&self, input: CreateModelMetadata) -> Result<(), ModelMetadataServiceError> {
        let create_metadata = || self.model_metadata_repo.save(&input);

        retry_async(create_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        return Ok(())
    }

    pub async fn discover_models(&self, input: DiscoverModelsInput) -> Result<DiscoverModelsOutput, ModelMetadataServiceError> {
        let discover_models = || self.model_metadata_repo.filter_model_metadata_by_criteria(&input);
       
       // Find model metadata by discovery criteria
        let output = retry_async(discover_models, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ModelMetadataServiceError::RepoError(err))?;

        Ok(output)
    }
}
