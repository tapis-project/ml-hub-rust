use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::{
        inputs::discover_models::SearchExternalModelsInput,
        outputs::model::ExternalModelListOutput,
        ports::model::{ExternalModelRepository, ExternalModelRepositoryError},
    },
    domain::entities::model::external_model::ExternalModel,
    shared_kernel::identifiers::ExternalModelId,
};

#[derive(Debug, Error)]
pub enum ExternalModelDiscoveryServiceError {
    #[error("ExternalModel not found")]
    NotFound,

    #[error(transparent)]
    Repository(#[from] ExternalModelRepositoryError),
}

pub struct ExternalModelDiscoveryService {
    repository: Arc<dyn ExternalModelRepository>,
}

impl ExternalModelDiscoveryService {
    const RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(repository: Arc<dyn ExternalModelRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_external_model(
        &self,
        id: &ExternalModelId,
    ) -> Result<ExternalModel, ExternalModelDiscoveryServiceError> {
        retry_async(|| self.repository.find_by_id(id), &Self::RETRY_POLICY, None)
            .await?
            .ok_or(ExternalModelDiscoveryServiceError::NotFound)
    }

    pub async fn discover_external_models(
        &self,
        input: &SearchExternalModelsInput,
    ) -> Result<ExternalModelListOutput, ExternalModelDiscoveryServiceError> {
        let page = retry_async(|| self.repository.search(input), &Self::RETRY_POLICY, None).await?;

        Ok(ExternalModelListOutput {
            external_models: page.external_models,
            count: page.count,
            cursor: page.cursor,
        })
    }
}
