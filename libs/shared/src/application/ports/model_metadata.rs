use crate::domain::entities::model_metadata::ModelMetadata;
use crate::application::errors::ApplicationError;
use crate::application::inputs::model_metadata::{UpsertModelMetadata, UpdateModelMetadataArtifactId};
use crate::application::inputs::discover_models::DiscoverModelsInput;
use crate::application::outputs::discover_models::DiscoverModelsOutput;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ModelMetadataRepository: Send + Sync {
    async fn upsert(&self, input: &UpsertModelMetadata) -> Result<(), ApplicationError>;
    async fn find_by_name_and_author(&self, name: &String, author: &String, tenant_id: &String) -> Result<Option<ModelMetadata>, ApplicationError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<ModelMetadata>, ApplicationError>;
    async fn filter_model_metadata_by_criteria(&self, input: &DiscoverModelsInput, tenant_ids: &Vec<String>) -> Result<DiscoverModelsOutput, ApplicationError>;
    async fn update_artifact_id(&self, input: &UpdateModelMetadataArtifactId) -> Result<(), ApplicationError>;
    // async fn list(&self) -> Result<Vec<ModelMetadata>, ApplicationError>;
}