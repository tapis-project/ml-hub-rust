use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::ports::model::{ExternalModelRepository, ExternalModelRepositoryError},
    domain::{
        entities::{
            deployment_strategy::client_strategy_set::ClientStrategySet,
            model::external_model::{
                DeploymentStrategyReference, ExternalModel, ExternalModelError,
            },
        },
        services::deployment_strategy::{resolve_viable_strategies, StrategyEvaluationError},
    },
};

#[derive(Debug, Error)]
pub enum ExternalModelIngestionServiceError {
    #[error(transparent)]
    Repository(#[from] ExternalModelRepositoryError),

    #[error(transparent)]
    Domain(#[from] ExternalModelError),

    #[error(transparent)]
    Strategy(#[from] StrategyEvaluationError),
}

pub struct ExternalModelIngestionService {
    repository: Arc<dyn ExternalModelRepository>,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>,
}

impl ExternalModelIngestionService {
    const RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        repository: Arc<dyn ExternalModelRepository>,
        client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    ) -> Self {
        Self {
            repository,
            client_strategy_sets,
        }
    }

    pub async fn ingest_external_model(
        &self,
        candidate: ExternalModel,
    ) -> Result<ExternalModel, ExternalModelIngestionServiceError> {
        let existing = retry_async(
            || {
                self.repository
                    .find_by_provider_and_locator(candidate.provider(), candidate.locator())
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        let updating = existing.is_some();

        let mut external_model = match existing {
            Some(mut model) => {
                model.refresh(candidate.metadata().clone());
                model
            }
            None => candidate,
        };

        let mut references = Vec::new();
        for set in self.client_strategy_sets.iter() {
            for strategy in resolve_viable_strategies(&external_model, set.strategies())? {
                let strategy = strategy.into_inner();
                references.push(DeploymentStrategyReference::new(
                    strategy.name,
                    strategy.platform,
                ));
            }
        }

        external_model.replace_deployment_strategies(references);

        // Update and return external model
        if updating {
            retry_async(
                || self.repository.update(&external_model),
                &Self::RETRY_POLICY,
                None,
            )
            .await?;

            return Ok(external_model);
        }

        // Nothing to update. Save and return external model
        retry_async(
            || self.repository.save(&external_model),
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        Ok(external_model)
    }
}
