use crate::domain::entities::model_metadata::ModelMetadata;
use crate::application::errors::ApplicationError;
use crate::application::inputs::model_metadata::{UpdateModelMetadataArtifactId};
use crate::application::inputs::discover_models::SearchModelsInput;
use crate::shared_kernal::identity::IdentityContext;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ModelMetadataRepository: Send + Sync {
    // async fn save(&self, input: &CreateModelMetadata, ctx: &IdentityContext) -> Result<(), ApplicationError>;
    async fn upsert(&self, metadata: &ModelMetadata, ctx: &IdentityContext) -> Result<(), ApplicationError>;
    async fn find_by_author_and_name(&self, author: &String, name: &String, tenant_id: &String) -> Result<Option<ModelMetadata>, ApplicationError>;
    async fn find_all_by_author(&self, author: &String, tenant_id: &String) -> Result<Vec<ModelMetadata>, ApplicationError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<ModelMetadata>, ApplicationError>;
    async fn search(&self, input: &SearchModelsInput, tenant_ids: &Vec<String>) -> Result<ModelSearchResult, ApplicationError>;
    async fn update_artifact_id(&self, input: &UpdateModelMetadataArtifactId) -> Result<(), ApplicationError>;
}

pub struct ModelSearchResult {
    pub models: Vec<ModelMetadata>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}