use std::{rc::Rc, sync::Arc};

use evaluations::{Arguments, Evaluator, EvaluatorError};
use nonempty::NonEmpty;
use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::ports::{
        deployment_option::{DeploymentOptionRepository, DeploymentOptionRepositoryError},
        model::{ExternalModelRepository, ExternalModelRepositoryError},
    },
    domain::{
        entities::{
            deployment_option::{
                DeploymentOption, DeploymentOptionError, DeploymentTarget,
                HpcClusterQueueReference, NewDeploymentOptionProps, ServingRuntime,
            },
            deployment_strategy::client_strategy_set::ClientStrategySet,
            hpc_cluster::HpcCluster,
            model::external_model::{
                DeploymentStrategyReference, ExternalModel, ExternalModelError,
            },
        },
        services::deployment_strategy::{resolve_viable_strategies, StrategyEvaluationError},
    },
    shared_kernel::enums::DeploymentModality,
};

const FLEXSERV_DEPLOYMENT_OPTION_EVALUATION: &str = "FlexServ Compatible Deployment Option";

#[derive(Debug, Error)]
pub enum ExternalModelIngestionServiceError {
    #[error(transparent)]
    Repository(#[from] ExternalModelRepositoryError),

    #[error(transparent)]
    Domain(#[from] ExternalModelError),

    #[error(transparent)]
    Strategy(#[from] StrategyEvaluationError),

    #[error(transparent)]
    Evaluation(#[from] EvaluatorError),

    #[error(transparent)]
    DeploymentOptionDomain(#[from] DeploymentOptionError),

    #[error(transparent)]
    DeploymentOptionRepository(#[from] DeploymentOptionRepositoryError),
}

pub struct ExternalModelIngestionService {
    repository: Arc<dyn ExternalModelRepository>,
    deployment_option_repository: Arc<dyn DeploymentOptionRepository>,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    evaluator: Evaluator,
    hpc_clusters: Vec<HpcCluster>,
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
        deployment_option_repository: Arc<dyn DeploymentOptionRepository>,
        client_strategy_sets: Arc<Vec<ClientStrategySet>>,
        evaluator: Evaluator,
        hpc_clusters: Vec<HpcCluster>,
    ) -> Self {
        Self {
            repository,
            deployment_option_repository,
            client_strategy_sets,
            evaluator,
            hpc_clusters,
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

        let deployment_options = self.calculate_deployment_options(&external_model).await?;

        if updating {
            retry_async(
                || self.repository.update(&external_model),
                &Self::RETRY_POLICY,
                None,
            )
            .await?;
        } else {
            retry_async(
                || self.repository.save(&external_model),
                &Self::RETRY_POLICY,
                None,
            )
            .await?;
        }

        retry_async(
            || {
                self.deployment_option_repository
                    .replace_for_external_model(external_model.id(), &deployment_options)
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        Ok(external_model)
    }

    async fn calculate_deployment_options(
        &self,
        external_model: &ExternalModel,
    ) -> Result<Vec<DeploymentOption>, ExternalModelIngestionServiceError> {
        let candidates = {
            let mut candidates = Vec::new();
            let model_argument = Rc::new(external_model.clone());

            for cluster in &self.hpc_clusters {
                for queue in cluster.queues() {
                    let mut arguments = Arguments::new();
                    arguments.insert("model".into(), model_argument.clone());
                    arguments.insert("queue".into(), Rc::new(queue.clone()));

                    if !self
                        .evaluator
                        .evaluate(FLEXSERV_DEPLOYMENT_OPTION_EVALUATION, &arguments)?
                    {
                        continue;
                    }

                    candidates.push(DeploymentOption::new(NewDeploymentOptionProps {
                        external_model_id: *external_model.id(),
                        supported_deployment_modalities: NonEmpty::new(DeploymentModality::Batch),
                        deployment_target: DeploymentTarget::HpcClusterQueue(
                            HpcClusterQueueReference::new(*cluster.id(), *queue.id()),
                        ),
                        serving_runtime: ServingRuntime::FlexServ,
                    })?);
                }
            }

            candidates
        };

        let mut existing = retry_async(
            || {
                self.deployment_option_repository
                    .find_by_external_model_id(external_model.id())
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        let mut reconciled = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            let Some(index) = existing
                .iter()
                .position(|option| option.has_same_semantic_identity(&candidate))
            else {
                reconciled.push(candidate);
                continue;
            };

            let mut option = existing.remove(index);
            option.refresh(candidate.supported_deployment_modalities().clone())?;
            reconciled.push(option);
        }

        Ok(reconciled)
    }
}
