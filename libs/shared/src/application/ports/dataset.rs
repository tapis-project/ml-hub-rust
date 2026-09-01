use crate::{application::ports::errors::InfrastructureError, domain::entities::dataset::Dataset};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DatasetRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait DatasetRepository: Send + Sync {
    async fn save(&self, dataset: &Dataset) -> Result<(), DatasetRepositoryError>;

    async fn find_by_id(
        &self,
        tenant_id: &str,
        id: Uuid,
    ) -> Result<Option<Dataset>, DatasetRepositoryError>;

    async fn find_by_huggingface_repo_locator(
        &self,
        tenant_id: &str,
        owner: &str,
        repo_id: &str,
        sha: &str,
    ) -> Result<Option<Dataset>, DatasetRepositoryError>;

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<Dataset>, DatasetRepositoryError>;

    async fn list_shared_with_user(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<Dataset>, DatasetRepositoryError>;
}
