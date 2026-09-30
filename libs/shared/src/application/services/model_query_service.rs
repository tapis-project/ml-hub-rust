use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    application::{
        inputs::model::ListModelsInput,
        outputs::model::{ModelListOutput, ModelWithExternalModel},
        ports::model::{
            ExternalModelRepository, ExternalModelRepositoryError, ModelPage, ModelRepository,
            ModelRepositoryError,
        },
    },
    domain::entities::model::Model,
    shared_kernel::{context::RequestContext, enums::Visibility},
};

#[derive(Debug, Error)]
pub enum ModelQueryServiceError {
    #[error("Model not found")]
    NotFound,

    #[error("Data integrity error: Model references a missing ExternalModel")]
    MissingExternalModel,

    #[error(transparent)]
    ModelRepository(#[from] ModelRepositoryError),

    #[error(transparent)]
    ExternalModelRepository(#[from] ExternalModelRepositoryError),
}

pub struct ModelQueryService {
    model_repository: Arc<dyn ModelRepository>,
    external_model_repository: Arc<dyn ExternalModelRepository>,
}

impl ModelQueryService {
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

    pub async fn get_model(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<ModelWithExternalModel, ModelQueryServiceError> {
        let model = retry_async(
            || self.model_repository.find_by_id(ctx.actor_tenant_id(), id),
            &Self::RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(ModelQueryServiceError::NotFound)?;

        if model.owner() != ctx.actor_principal_id()
            && !matches!(model.visibility(), Visibility::Public)
        {
            return Err(ModelQueryServiceError::NotFound);
        }

        let external_model = retry_async(
            || {
                self.external_model_repository
                    .find_by_id(model.external_model_id())
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(ModelQueryServiceError::MissingExternalModel)?;

        Ok(ModelWithExternalModel {
            model,
            external_model,
        })
    }

    pub async fn list_owned(
        &self,
        ctx: &RequestContext,
        input: &ListModelsInput,
    ) -> Result<ModelListOutput, ModelQueryServiceError> {
        let page = retry_async(
            || {
                self.model_repository.list_by_owner(
                    ctx.actor_tenant_id(),
                    ctx.actor_principal_id(),
                    input,
                )
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        self.hydrate(page).await
    }

    pub async fn list_shared(
        &self,
        ctx: &RequestContext,
        input: &ListModelsInput,
    ) -> Result<ModelListOutput, ModelQueryServiceError> {
        let page = retry_async(
            || {
                self.model_repository.list_shared(
                    ctx.actor_tenant_id(),
                    ctx.actor_principal_id(),
                    input,
                )
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        self.hydrate(page).await
    }

    async fn hydrate(&self, page: ModelPage) -> Result<ModelListOutput, ModelQueryServiceError> {
        let ids = page
            .models
            .iter()
            .map(|model| *model.external_model_id())
            .collect::<Vec<_>>();

        let external_models = retry_async(
            || self.external_model_repository.find_by_ids(&ids),
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        let mut external_by_id = external_models
            .into_iter()
            .map(|model| (*model.id(), model))
            .collect::<HashMap<_, _>>();

        let models = page
            .models
            .into_iter()
            .map(|model: Model| {
                let external_model = external_by_id
                    .remove(model.external_model_id())
                    .ok_or(ModelQueryServiceError::MissingExternalModel)?;
                Ok::<ModelWithExternalModel, ModelQueryServiceError>(ModelWithExternalModel {
                    model,
                    external_model,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ModelListOutput {
            models,
            count: page.count,
            cursor: page.cursor,
        })
    }
}

#[cfg(test)]
#[path = "model_query_service.test.rs"]
mod model_query_service_test;
