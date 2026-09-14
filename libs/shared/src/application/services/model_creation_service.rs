use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::{
        inputs::model::CreateModelInput,
        outputs::model::ModelWithExternalModel,
        ports::model::{
            ExternalModelRepository, ExternalModelRepositoryError, ModelRepository,
            ModelRepositoryError,
        },
    },
    domain::entities::model::{Model, ModelError},
    shared_kernel::context::RequestContext,
};

#[derive(Debug, Error)]
pub enum ModelCreationServiceError {
    #[error("Model already in your collection")]
    ModelAlreadyInCollection,

    #[error("ExternalModel not found")]
    ExternalModelNotFound,

    #[error(transparent)]
    ModelRepository(#[from] ModelRepositoryError),

    #[error(transparent)]
    ExternalModelRepository(#[from] ExternalModelRepositoryError),

    #[error(transparent)]
    Domain(#[from] ModelError),
}

pub struct ModelCreationService {
    model_repository: Arc<dyn ModelRepository>,
    external_model_repository: Arc<dyn ExternalModelRepository>,
}

impl ModelCreationService {
    const RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        model_repository: Arc<dyn ModelRepository>,
        external_model_repository: Arc<dyn ExternalModelRepository>,
    ) -> Self {
        Self {
            model_repository,
            external_model_repository,
        }
    }

    pub async fn create_model(
        &self,
        ctx: &RequestContext,
        input: CreateModelInput,
    ) -> Result<ModelWithExternalModel, ModelCreationServiceError> {
        let external_model = retry_async(
            || {
                self.external_model_repository
                    .find_by_id(&input.external_model_id)
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(ModelCreationServiceError::ExternalModelNotFound)?;

        let existing = retry_async(
            || {
                self.model_repository.find_by_external_model_id(
                    ctx.actor_tenant_id(),
                    ctx.actor_principal_id(),
                    &input.external_model_id,
                )
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        if existing.is_some() {
            return Err(ModelCreationServiceError::ModelAlreadyInCollection);
        }

        let model = Model::create(
            ctx.actor_tenant_id().clone(),
            ctx.actor_principal_id().clone(),
            input.name,
            input.description,
            input.external_model_id,
            input.visibility,
        )?;

        let save_result = retry_async(
            || self.model_repository.save(&model),
            &Self::RETRY_POLICY,
            None,
        )
        .await;

        match save_result {
            Ok(()) => Ok(ModelWithExternalModel {
                model,
                external_model,
            }),
            Err(ModelRepositoryError::ModelAlreadyInCollection) => {
                Err(ModelCreationServiceError::ModelAlreadyInCollection)
            }
            Err(error) => Err(error.into()),
        }
    }
}

#[cfg(test)]
#[path = "model_creation_service.test.rs"]
mod model_creation_service_test;
