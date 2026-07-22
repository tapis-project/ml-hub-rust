use crate::domain::entities::model_metadata::ModelMetadata;
use crate::application::inputs::model_metadata::{UpdateModelMetadataArtifactId};
use crate::application::inputs::discover_models::SearchModelsInput;
use crate::shared_kernel::context::RequestContext;
use crate::application::ports::errors::CommonRepositoryError;

use uuid::Uuid;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelMetadataRepositoryError {
    #[error(transparent)]
    Persistence(#[from] CommonRepositoryError),
}

#[async_trait]
pub trait ModelMetadataRepository: Send + Sync {
    // async fn save(&self, input: &CreateModelMetadata, ctx: &RequestContext) -> Result<(), ApplicationError>;
    async fn upsert(&self, metadata: &ModelMetadata, ctx: &RequestContext) -> Result<(), ModelMetadataRepositoryError>;
    async fn find_by_author_and_name(&self, author: &String, name: &String, tenant_id: &String) -> Result<Option<ModelMetadata>, ModelMetadataRepositoryError>;
    async fn find_all_by_author(&self, author: &String, tenant_id: &String) -> Result<Vec<ModelMetadata>, ModelMetadataRepositoryError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<ModelMetadata>, ModelMetadataRepositoryError>;
    async fn search(&self, input: &SearchModelsInput, tenant_ids: &Vec<String>) -> Result<ModelSearchResult, ModelMetadataRepositoryError>;
    async fn update_artifact_id(&self, input: &UpdateModelMetadataArtifactId) -> Result<(), ModelMetadataRepositoryError>;
}

pub struct ModelSearchResult {
    pub models: Vec<ModelMetadata>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}