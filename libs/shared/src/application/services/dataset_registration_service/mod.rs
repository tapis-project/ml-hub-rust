use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::{
        inputs::dataset::RegisterDatasetInput,
        ports::dataset::{DatasetRepository, DatasetRepositoryError},
    },
    domain::entities::dataset::{Dataset, DatasetError, DatasetItemError, DatasetLocatorError},
    shared_kernel::context::RequestContext,
};

#[derive(Debug, Error)]
pub enum DatasetRegistrationServiceError {
    #[error("Dataset repository error: {0}")]
    Repository(#[from] DatasetRepositoryError),

    #[error("Dataset domain error: {0}")]
    Domain(#[from] DatasetError),

    #[error("Dataset item error: {0}")]
    Item(#[from] DatasetItemError),

    #[error("Dataset locator error: {0}")]
    Locator(#[from] DatasetLocatorError),
}

pub struct DatasetRegistrationService {
    dataset_repository: Arc<dyn DatasetRepository>,
}

impl DatasetRegistrationService {
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
    ) -> Result<Dataset, DatasetRegistrationServiceError> {
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
}

#[cfg(test)]
#[path = "dataset_registration_service.test.rs"]
mod dataset_registration_service_test;
