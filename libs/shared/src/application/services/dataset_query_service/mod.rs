use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    application::{
        inputs::dataset::ListDatasetsInput,
        outputs::dataset::{DatasetListOutput, DatasetQueryOutput},
        ports::dataset::{DatasetRepository, DatasetRepositoryError},
    },
    domain::entities::dataset::Dataset,
    shared_kernel::{constants::GLOBAL_TENANT, context::RequestContext, enums::Visibility},
};

#[derive(Debug, Error)]
pub enum DatasetQueryServiceError {
    #[error("Dataset repository error: {0}")]
    Repository(#[from] DatasetRepositoryError),

    #[error("Dataset not found")]
    NotFound,
}

pub struct DatasetQueryService {
    dataset_repository: Arc<dyn DatasetRepository>,
}

impl DatasetQueryService {
    const REPOSITORY_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(dataset_repository: Arc<dyn DatasetRepository>) -> Self {
        Self { dataset_repository }
    }

    pub async fn get_dataset(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<DatasetQueryOutput, DatasetQueryServiceError> {
        let dataset = retry_async(
            || {
                self.dataset_repository
                    .find_by_id(ctx.actor_tenant_id(), id)
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(DatasetQueryServiceError::NotFound)?;

        if &dataset.owner != ctx.actor_principal_id()
            && !matches!(dataset.visibility, Visibility::Public)
        {
            return Err(DatasetQueryServiceError::NotFound);
        }

        Ok(dataset)
    }

    pub async fn find_by_huggingface_repo_locator(
        &self,
        ctx: &RequestContext,
        repo_id: &str,
        sha: &str,
    ) -> Result<Option<Dataset>, DatasetQueryServiceError> {
        Ok(retry_async(
            || {
                self.dataset_repository.find_by_huggingface_repo_locator(
                    ctx.actor_tenant_id(),
                    ctx.actor_principal_id(),
                    repo_id,
                    sha,
                )
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }

    pub async fn list_for_user(
        &self,
        ctx: &RequestContext,
        input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetQueryServiceError> {
        Ok(retry_async(
            || {
                self.dataset_repository.list_by_owner(
                    ctx.actor_tenant_id(),
                    ctx.actor_principal_id(),
                    input,
                )
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }

    pub async fn list_global(
        &self,
        _ctx: &RequestContext,
        input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetQueryServiceError> {
        Ok(retry_async(
            || self.dataset_repository.list_by_tenant(GLOBAL_TENANT, input),
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }

    pub async fn list_shared_with_user(
        &self,
        ctx: &RequestContext,
        input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetQueryServiceError> {
        Ok(retry_async(
            || {
                self.dataset_repository.list_shared_with_user(
                    ctx.actor_tenant_id(),
                    ctx.actor_principal_id(),
                    input,
                )
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }
}

#[cfg(test)]
#[path = "dataset_query_service.test.rs"]
mod dataset_query_service_test;
