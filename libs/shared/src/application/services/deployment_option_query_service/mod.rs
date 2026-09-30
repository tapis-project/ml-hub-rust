use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::{
        inputs::deployment_option::ListDeploymentOptionsInput,
        outputs::deployment_option::{
            DeploymentOptionListOutput, DeploymentOptionOutput, HpcClusterQueueTargetOutput,
        },
        ports::{
            deployment_option::{DeploymentOptionRepository, DeploymentOptionRepositoryError},
            hpc_cluster::{HpcClusterRepository, HpcClusterRepositoryError},
            model::{ExternalModelRepository, ExternalModelRepositoryError},
        },
    },
    domain::entities::{
        deployment_option::DeploymentTarget,
        hpc_cluster::{HpcCluster, HpcClusterId},
    },
    shared_kernel::{context::RequestContext, identifiers::ExternalModelId},
};

#[derive(Debug, Error)]
pub enum DeploymentOptionQueryServiceError {
    #[error("ExternalModel not found")]
    ExternalModelNotFound,

    #[error("DeploymentOption data integrity error: {0}")]
    DataIntegrity(String),

    #[error(transparent)]
    DeploymentOptionRepository(#[from] DeploymentOptionRepositoryError),

    #[error(transparent)]
    ExternalModelRepository(#[from] ExternalModelRepositoryError),

    #[error(transparent)]
    HpcClusterRepository(#[from] HpcClusterRepositoryError),
}

pub struct DeploymentOptionQueryService {
    deployment_option_repository: Arc<dyn DeploymentOptionRepository>,
    external_model_repository: Arc<dyn ExternalModelRepository>,
    hpc_cluster_repository: Arc<dyn HpcClusterRepository>,
}

impl DeploymentOptionQueryService {
    const RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        deployment_option_repository: Arc<dyn DeploymentOptionRepository>,
        external_model_repository: Arc<dyn ExternalModelRepository>,
        hpc_cluster_repository: Arc<dyn HpcClusterRepository>,
    ) -> Self {
        Self {
            deployment_option_repository,
            external_model_repository,
            hpc_cluster_repository,
        }
    }

    pub async fn list_external_model_deployment_options(
        &self,
        _ctx: &RequestContext,
        external_model_id: &ExternalModelId,
        input: &ListDeploymentOptionsInput,
    ) -> Result<DeploymentOptionListOutput, DeploymentOptionQueryServiceError> {
        let external_model = retry_async(
            || self.external_model_repository.find_by_id(external_model_id),
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        if external_model.is_none() {
            return Err(DeploymentOptionQueryServiceError::ExternalModelNotFound);
        }

        let page = retry_async(
            || {
                self.deployment_option_repository
                    .list_by_external_model_id(external_model_id, input)
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        if page.deployment_options.is_empty() {
            return Ok(DeploymentOptionListOutput {
                deployment_options: Vec::new(),
                cursor: page.cursor,
                count: page.count,
            });
        }

        let cluster_ids = page
            .deployment_options
            .iter()
            .map(|option| match option.deployment_target() {
                DeploymentTarget::HpcClusterQueue(reference) => *reference.hpc_cluster_id(),
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let hpc_clusters = retry_async(
            || self.hpc_cluster_repository.find_by_ids(&cluster_ids),
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        let hpc_clusters = hpc_clusters
            .into_iter()
            .map(|cluster| (*cluster.id(), cluster))
            .collect::<HashMap<HpcClusterId, HpcCluster>>();
        let mut deployment_options = Vec::with_capacity(page.deployment_options.len());

        for deployment_option in page.deployment_options {
            let target_reference = match deployment_option.deployment_target() {
                DeploymentTarget::HpcClusterQueue(reference) => reference,
            };

            let hpc_cluster = hpc_clusters
                .get(target_reference.hpc_cluster_id())
                .ok_or_else(|| {
                    DeploymentOptionQueryServiceError::DataIntegrity(format!(
                        "HPC cluster {} does not exist",
                        target_reference.hpc_cluster_id()
                    ))
                })?;

            let queue = hpc_cluster
                .queues()
                .iter()
                .find(|queue| queue.id() == target_reference.batch_scheduler_queue_id())
                .ok_or_else(|| {
                    DeploymentOptionQueryServiceError::DataIntegrity(format!(
                        "Batch scheduler queue {} does not exist in HPC cluster {}",
                        target_reference.batch_scheduler_queue_id(),
                        hpc_cluster.id()
                    ))
                })?;

            let available = hpc_cluster.enabled() && queue.enabled();
            let target = HpcClusterQueueTargetOutput {
                hpc_cluster_id: *hpc_cluster.id(),
                hpc_cluster_name: hpc_cluster.name().into(),
                data_center: *hpc_cluster.data_center(),
                hpc_cluster_enabled: hpc_cluster.enabled(),
                batch_scheduler_queue_id: *queue.id(),
                batch_scheduler_queue_name: queue.name().into(),
                batch_scheduler_queue_enabled: queue.enabled(),
            };

            deployment_options.push(DeploymentOptionOutput {
                deployment_option,
                target,
                available,
            });
        }

        Ok(DeploymentOptionListOutput {
            deployment_options,
            cursor: page.cursor,
            count: page.count,
        })
    }
}

#[cfg(test)]
#[path = "deployment_option_query_service.test.rs"]
mod deployment_option_query_service_test;
