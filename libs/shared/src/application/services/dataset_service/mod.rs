use crate::{
    application::{
        inputs::dataset::RegisterDatasetInput,
        outputs::dataset::DatasetQueryOutput,
        ports::dataset::{DatasetRepository, DatasetRepositoryError},
    },
    domain::entities::dataset::{Dataset, DatasetError, DatasetItemError, DatasetLocatorError},
    shared_kernel::{context::RequestContext, enums::Visibility},
};
use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DatasetServiceError {
    #[error("Dataset repository error: {0}")]
    Repository(#[from] DatasetRepositoryError),

    #[error("Dataset domain error: {0}")]
    Domain(#[from] DatasetError),

    #[error("Dataset item error: {0}")]
    Item(#[from] DatasetItemError),

    #[error("Dataset locator error: {0}")]
    Locator(#[from] DatasetLocatorError),

    #[error("Dataset not found")]
    NotFound,
}

pub struct DatasetService {
    dataset_repository: Arc<dyn DatasetRepository>,
}

impl DatasetService {
    const REPOSITORY_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(dataset_repository: Arc<dyn DatasetRepository>) -> Self {
        Self { dataset_repository }
    }

    pub async fn register_dataset(
        &self,
        ctx: &RequestContext,
        input: RegisterDatasetInput,
    ) -> Result<Dataset, DatasetServiceError> {
        let dataset = Dataset::register(
            ctx.actor_tenant_id().clone(),
            ctx.actor_principal_id().clone(),
            input.tags,
            input.provider.try_into()?,
            input
                .items
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            input.size,
            input.visibility.into(),
        )?;

        retry_async(
            || self.dataset_repository.save(&dataset),
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(dataset)
    }

    pub async fn get_dataset(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<DatasetQueryOutput, DatasetServiceError> {
        let dataset = retry_async(
            || {
                self.dataset_repository
                    .find_by_id(ctx.actor_tenant_id(), id)
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(DatasetServiceError::NotFound)?;

        if &dataset.owner != ctx.actor_principal_id()
            && !matches!(dataset.visibility, Visibility::Public)
        {
            return Err(DatasetServiceError::NotFound);
        }

        Ok(dataset)
    }

    pub async fn find_by_huggingface_repo_locator(
        &self,
        ctx: &RequestContext,
        repo_id: &str,
        sha: &str,
    ) -> Result<Option<Dataset>, DatasetServiceError> {
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
    ) -> Result<Vec<DatasetQueryOutput>, DatasetServiceError> {
        Ok(retry_async(
            || {
                self.dataset_repository
                    .list_by_owner(ctx.actor_tenant_id(), ctx.actor_principal_id())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }

    pub async fn list_shared_with_user(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetServiceError> {
        Ok(retry_async(
            || {
                self.dataset_repository
                    .list_shared_with_user(ctx.actor_tenant_id(), ctx.actor_principal_id())
            },
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }
}

#[cfg(test)]
#[path = "dataset_service.test.rs"]
mod dataset_service_test;
