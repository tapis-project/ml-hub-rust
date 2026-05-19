use std::sync::Arc;
use crate::retry::{retry_async, RetryPolicy, FixedBackoff, Retry};
use crate::application::errors::ApplicationError;
use crate::application::ports::repositories::{ArtifactRepository, DatasetMetadataRepository};
use crate::application::inputs::dataset_metadata::{AssociateDatasetMetadata, CreateDatasetMetadata, UpdateDatasetMetadataArtifactId};
use crate::application::inputs::discover_datasets::DiscoverDatasetsInput;
use crate::domain::entities::dataset_metadata::DatasetMetadata as DatasetMetadata;
use crate::domain::services::{
    DatasetMetadataService as DatasetMetadataDomainService,
    DatasetMetadataServiceError as DatasetMetadataDomainServiceError
};
use thiserror::Error;
use once_cell::sync::Lazy;
// use crate::logging::GlobalLogger;

#[derive(Debug, Error)]
pub enum DatasetMetadataServiceError {
    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Artifact not found: {0}")]
    ArtifactNotFound(String),

    #[error("Metadata not found: {0}")]
    MetdataNotFound(String),

    #[error("{0}")]
    DomainServiceError(#[from] DatasetMetadataDomainServiceError),

    #[error("Metadata already exists for Artifact '{0}'")]
    DuplicateMetadataError(String),
}

pub struct DatasetMetadataService {
    dataset_metadata_repo: Arc<dyn DatasetMetadataRepository>,
    artifact_repo: Arc<dyn ArtifactRepository>
}

impl DatasetMetadataService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    pub fn new(
        dataset_metadata_repo: Arc<dyn DatasetMetadataRepository>,
        artifact_repo: Arc<dyn ArtifactRepository>
    ) -> Self {
        Self {
            dataset_metadata_repo,
            artifact_repo
        }
    }

    pub async fn associate_metadata_with_artifact(&self, input: AssociateDatasetMetadata) -> Result<(), DatasetMetadataServiceError> {
        // Get the artifact_id from the input
        let artifact_id = input.artifact_id.clone();

        let find_artifact = || self.artifact_repo.get_by_id(&artifact_id);

        // Find the artifact by id
        let artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY)
            .await?
            .ok_or_else(|| DatasetMetadataServiceError::ArtifactNotFound(format!("Artifact with id {} does not exist", &artifact_id)))?;

        // Ensure no metadata already exists for this artifact
        let find_metadata = || self.dataset_metadata_repo.find_by_artifact_id(&artifact_id);

        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        let metadata = match maybe_metadata {
            Some(m) => m,
            None => return Err(DatasetMetadataServiceError::MetdataNotFound(format!("No metadata found with author {} and name {}", input.author, input.name)))
        };

        // Determine if we are allowed to create the metadata for this artifact
        DatasetMetadataDomainService::associate_metadata_with_artifact(&artifact, metadata)?;

        let update_input = UpdateDatasetMetadataArtifactId::from(input);
        
        let update_metadata = || self.dataset_metadata_repo.update_artifact_id(&update_input);

        retry_async(update_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        return Ok(())
    }

    pub async fn create_dataset_metadata(&self, input: CreateDatasetMetadata) -> Result<(), DatasetMetadataServiceError> {
        let create_metadata = || self.dataset_metadata_repo.save(&input);

        retry_async(create_metadata, &Self::REPO_RETRY_POLICY)
            .await?;

        return Ok(())
    }

    pub async fn discover_datasets(&self, input: DiscoverDatasetsInput) -> Result<Vec<DatasetMetadata>, DatasetMetadataServiceError> {
        let discover_metadata = || self.dataset_metadata_repo.filter_dataset_metadata_by_criteria(&input);
       
       // Find dataset metadata by discovery criteria
        let metadata_entries = retry_async(discover_metadata, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| DatasetMetadataServiceError::RepoError(err))?;

        Ok(metadata_entries)
    }
}
