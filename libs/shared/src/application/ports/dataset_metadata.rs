use crate::domain::entities::dataset_metadata::DatasetMetadata;
use crate::application::errors::ApplicationError;
use crate::application::inputs::dataset_metadata::{CreateDatasetMetadata, UpdateDatasetMetadataArtifactId};
use crate::application::inputs::discover_datasets::DiscoverDatasetsInput;
use crate::application::outputs::discover_datasets::DiscoverDatasetsOutput;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait DatasetMetadataRepository: Send + Sync {
    async fn save(&self, input: &CreateDatasetMetadata) -> Result<(), ApplicationError>;
    async fn get_by_name_and_author(&self, name: &String, author: &String) -> Result<Option<DatasetMetadata>, ApplicationError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<DatasetMetadata>, ApplicationError>;
    async fn filter_dataset_metadata_by_criteria(&self, input: &DiscoverDatasetsInput) -> Result<DiscoverDatasetsOutput, ApplicationError>;
    async fn update_artifact_id(&self, input: &UpdateDatasetMetadataArtifactId) -> Result<(), ApplicationError>;
    // async fn list(&self) -> Result<Vec<DatasetMetadata>, ApplicationError>;
}