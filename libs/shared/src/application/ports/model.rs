use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    application::{
        inputs::{discover_models::SearchExternalModelsInput, model::ListModelsInput},
        ports::errors::InfrastructureError,
    },
    domain::entities::model::{
        external_model::{ExternalModel, ModelLocator, ModelProvider},
        Model,
    },
    shared_kernel::identifiers::ExternalModelId,
};

#[derive(Debug, Error)]
pub enum ModelRepositoryError {
    #[error("Model already in your collection")]
    ModelAlreadyInCollection,

    #[error("Artifact is already associated with a Model")]
    ArtifactAlreadyAssociated,

    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[derive(Debug, Error)]
pub enum ExternalModelRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[derive(Debug)]
pub struct ModelPage {
    pub models: Vec<Model>,
    pub count: Option<u64>,
    pub cursor: Option<String>,
}

#[derive(Debug)]
pub struct ExternalModelPage {
    pub external_models: Vec<ExternalModel>,
    pub count: Option<u64>,
    pub cursor: Option<String>,
}

#[async_trait]
pub trait ModelRepository: Send + Sync {
    async fn save(&self, model: &Model) -> Result<(), ModelRepositoryError>;
    async fn update(&self, model: &Model) -> Result<(), ModelRepositoryError>;
    async fn find_by_id(
        &self,
        tenant_id: &str,
        id: Uuid,
    ) -> Result<Option<Model>, ModelRepositoryError>;
    async fn find_by_external_model_id(
        &self,
        tenant_id: &str,
        owner: &str,
        external_model_id: &ExternalModelId,
    ) -> Result<Option<Model>, ModelRepositoryError>;
    async fn find_by_artifact_id(
        &self,
        artifact_id: &Uuid,
    ) -> Result<Option<Model>, ModelRepositoryError>;
    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
        input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError>;
    async fn list_shared(
        &self,
        tenant_id: &str,
        owner: &str,
        input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError>;
}

#[async_trait]
pub trait ExternalModelRepository: Send + Sync {
    async fn save(&self, model: &ExternalModel) -> Result<(), ExternalModelRepositoryError>;
    async fn update(&self, model: &ExternalModel) -> Result<(), ExternalModelRepositoryError>;
    async fn find_by_id(
        &self,
        id: &ExternalModelId,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError>;
    async fn find_by_ids(
        &self,
        ids: &[ExternalModelId],
    ) -> Result<Vec<ExternalModel>, ExternalModelRepositoryError>;
    async fn find_by_provider_and_locator(
        &self,
        provider: &ModelProvider,
        locator: &ModelLocator,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError>;
    async fn search(
        &self,
        input: &SearchExternalModelsInput,
    ) -> Result<ExternalModelPage, ExternalModelRepositoryError>;
}
