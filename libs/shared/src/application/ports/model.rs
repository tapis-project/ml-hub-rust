use crate::application::inputs::discover_models::SearchModelsInput;
use crate::application::inputs::model::UpdateModelArtifactId;
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::model::Model;
use crate::shared_kernel::context::RequestContext;

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ModelRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait ModelRepository: Send + Sync {
    // async fn save(&self, input: &CreateModel, ctx: &RequestContext) -> Result<(), ApplicationError>;
    async fn upsert(&self, model: &Model, ctx: &RequestContext)
        -> Result<(), ModelRepositoryError>;
    async fn find_by_author_and_name(
        &self,
        author: &String,
        name: &String,
        tenant_id: &String,
    ) -> Result<Option<Model>, ModelRepositoryError>;
    async fn find_all_by_author(
        &self,
        author: &String,
        tenant_id: &String,
    ) -> Result<Vec<Model>, ModelRepositoryError>;
    async fn find_by_artifact_id(
        &self,
        artifact_id: &Uuid,
    ) -> Result<Option<Model>, ModelRepositoryError>;
    async fn search(
        &self,
        input: &SearchModelsInput,
        tenant_ids: &Vec<String>,
    ) -> Result<ModelSearchResult, ModelRepositoryError>;
    async fn update_artifact_id(
        &self,
        input: &UpdateModelArtifactId,
    ) -> Result<(), ModelRepositoryError>;
}

pub struct ModelSearchResult {
    pub models: Vec<Model>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}
