use crate::domain::entities::model_metadata::ModelMetadata;
use crate::application::errors::ApplicationError;
use crate::application::inputs::model_metadata::{UpsertModelMetadata, UpdateModelMetadataArtifactId};
use crate::application::inputs::discover_models::DiscoverModelsInput;
use crate::application::outputs::discover_models::DiscoverModelsOutput;
use crate::shared_kernal::identity::IdentityContext;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ModelMetadataRepository: Send + Sync {
    async fn upsert(&self, input: &UpsertModelMetadata, ctx: &IdentityContext) -> Result<(), ApplicationError>;
    async fn find_by_author_and_name(&self, author: &String, name: &String, tenant_id: &String) -> Result<Option<ModelMetadata>, ApplicationError>;
    async fn find_all_by_author(&self, author: &String, tenant_id: &String) -> Result<Vec<ModelMetadata>, ApplicationError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<ModelMetadata>, ApplicationError>;
    async fn filter_model_metadata_by_criteria(&self, input: &DiscoverModelsInput, tenant_ids: &Vec<String>) -> Result<DiscoverModelsOutput, ApplicationError>;
    async fn update_artifact_id(&self, input: &UpdateModelMetadataArtifactId) -> Result<(), ApplicationError>;
}