use std::sync::Arc;

use log::error;
use once_cell::sync::Lazy;
use retry_utils::{retry_async, ExponentialBackoff, FixedBackoff, Jitter, Retry, RetryPolicy};
use thiserror::Error;

use crate::application::inputs::deployment::{
    DeployWithOptionInput, FilterInput, FindForReconciliationInput, StartModelDeploymentInput,
    StopModelDeploymentInput, UndeployModelDeploymentInput, UpdateModelDeploymentInput,
};
use crate::application::outputs::deployment::{
    DeployModelWithOptionOutput, StartModelDeploymentOutput, StopModelDeploymentOutput,
    UndeployModelDeploymentOutput,
};
use crate::application::ports::deployment::{
    ModelDeploymentRepository, ModelDeploymentRepositoryError,
};
use crate::application::ports::deployment_option::{
    DeploymentOptionRepository, DeploymentOptionRepositoryError,
};
use crate::application::ports::events::payloads::ModelDeploymentStartedPayload;
use crate::application::ports::events::{Event, EventPublisher, Payload};
use crate::application::ports::hpc_cluster::{HpcClusterRepository, HpcClusterRepositoryError};
use crate::application::ports::model::{ExternalModelRepository, ExternalModelRepositoryError};
use crate::application::workflows::deployment::{
    UpdateDesiredStateWorkflow, UpdateDesiredStateWorkflowInput,
};
use crate::application::workflows::Workflow;
use crate::domain::entities::deployment::{
    CreateFromOptionProps, DeploymentOptionSnapshot, DeploymentTargetSnapshot, DesiredState,
    HpcClusterQueueSnapshot, ModelDeployment, ModelDeploymentError, ModelDeploymentId,
    ReplicaGroup,
};
use crate::domain::entities::deployment_option::deployment_parameters::DeploymentParameterError;
use crate::domain::entities::deployment_option::DeploymentTarget;
use crate::domain::services::{
    ModelDeploymentDomainServiceError, ModelDeploymentService as ModelDeploymentDomainService,
};
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::TimeStamp;

use super::deployment_argument_service::{
    DeploymentArgumentService, DeploymentArgumentServiceError,
};

#[derive(Clone, Debug, Error)]
pub enum ModelDeploymentServiceError {
    #[error("Revision mismatch: Expected to find revision `{0}` but found `{1}`")]
    RevisionMismatch(String, String),

    #[error("Model Deployment repository error: {0}")]
    ModelDeploymentRepoError(#[from] ModelDeploymentRepositoryError),

    #[error("DeploymentOption repository error: {0}")]
    DeploymentOptionRepositoryError(#[from] DeploymentOptionRepositoryError),

    #[error("HPC cluster repository error: {0}")]
    HpcClusterRepositoryError(#[from] HpcClusterRepositoryError),

    #[error("ExternalModel repository error: {0}")]
    ExternalModelRepositoryError(#[from] ExternalModelRepositoryError),

    #[error("Argument persistence error: {0}")]
    ArgumentPersistenceError(#[from] DeploymentArgumentServiceError),

    #[error("Invalid deployment parameters: {0}")]
    InvalidParameters(DeploymentParameterError),

    #[error("Model Deployment not found: {0}")]
    DeploymentNotFound(String),

    #[error("DeploymentOption not found: {0}")]
    MissingDeploymentOption(String),

    #[error("ExternalModel not found: {0}")]
    MissingExternalModel(String),

    #[error("HPC cluster not found: {0}")]
    MissingHpcCluster(String),

    #[error("Batch scheduler queue not found: {0}")]
    MissingBatchSchedulerQueue(String),

    #[error("Deployment option is currently unavailable")]
    DeploymentOptionUnavailable,

    #[error("Model deployment error: {0}")]
    ModelDeploymentError(#[from] ModelDeploymentError),

    #[error("Model deployment domain error: {0}")]
    ModelDeploymentDomainError(#[from] ModelDeploymentDomainServiceError),
}

pub struct ModelDeploymentService {
    deployment_argument_service: DeploymentArgumentService,
    model_deployment_repository: Arc<dyn ModelDeploymentRepository>,
    deployment_option_repository: Arc<dyn DeploymentOptionRepository>,
    external_model_repository: Arc<dyn ExternalModelRepository>,
    hpc_cluster_repository: Arc<dyn HpcClusterRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ModelDeploymentService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    const EVENT_PUBLISHER_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::ExponentialBackoff(ExponentialBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
            base: Some(2),
            max_delay: 500,
            jitter: Some(Jitter::Full),
        })
    });

    pub fn new(
        deployment_argument_service: DeploymentArgumentService,
        model_deployment_repository: Arc<dyn ModelDeploymentRepository>,
        deployment_option_repository: Arc<dyn DeploymentOptionRepository>,
        external_model_repository: Arc<dyn ExternalModelRepository>,
        hpc_cluster_repository: Arc<dyn HpcClusterRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            deployment_argument_service,
            model_deployment_repository,
            deployment_option_repository,
            external_model_repository,
            hpc_cluster_repository,
            event_publisher,
        }
    }

    pub async fn find_by_id(
        &self,
        deployment_id: &ModelDeploymentId,
    ) -> Result<Option<ModelDeployment>, ModelDeploymentServiceError> {
        let filter = FilterInput {
            deployment_id: Some(*deployment_id),
            state: None,
            revision: None,
        };

        let deployment = retry_async(
            || self.model_deployment_repository.find(&filter),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(deployment)
    }

    pub async fn list_by_owner(
        &self,
        input: ListModelDeploymentsByOwnerInput,
        ctx: &RequestContext,
    ) -> Result<Vec<ModelDeployment>, ModelDeploymentServiceError> {
        let deployments = retry_async(
            || {
                self.model_deployment_repository
                    .find_by_owner(ctx.actor_tenant_id(), &input.owner)
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(deployments)
    }

    pub async fn find_for_reconciliation(
        &self,
        input: FindForReconciliationInput,
    ) -> Result<ModelDeployment, ModelDeploymentServiceError> {
        let filter = FilterInput {
            deployment_id: Some(input.deployment_id),
            state: None,
            revision: None,
        };

        let deployment = retry_async(
            || self.model_deployment_repository.find(&filter),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or_else(|| {
            ModelDeploymentServiceError::DeploymentNotFound(input.deployment_id.to_string())
        })?;

        if deployment.revision() != &input.revision {
            return Err(ModelDeploymentServiceError::RevisionMismatch(
                input.revision.to_string(),
                deployment.revision().to_string(),
            ));
        }

        Ok(deployment)
    }

    pub async fn find_all_reconcilable(
        &self,
    ) -> Result<Vec<ModelDeployment>, ModelDeploymentServiceError> {
        let deployments = retry_async(
            || self.model_deployment_repository.find_reconcilable(),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(deployments)
    }

    pub async fn deploy_model_with_option(
        &self,
        ctx: &RequestContext,
        input: DeployWithOptionInput,
    ) -> Result<DeployModelWithOptionOutput, ModelDeploymentServiceError> {
        let deployment_option = retry_async(
            || {
                self.deployment_option_repository
                    .find_by_id(&input.deployment_option_id)
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or_else(|| {
            ModelDeploymentServiceError::MissingDeploymentOption(
                input.deployment_option_id.to_string(),
            )
        })?;

        retry_async(
            || {
                self.external_model_repository
                    .find_by_id(deployment_option.external_model_id())
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or_else(|| {
            ModelDeploymentServiceError::MissingExternalModel(
                deployment_option.external_model_id().to_string(),
            )
        })?;

        let target_reference = match deployment_option.deployment_target() {
            DeploymentTarget::HpcClusterQueue(reference) => reference,
        };

        let cluster = retry_async(
            || {
                self.hpc_cluster_repository
                    .find_by_id(target_reference.hpc_cluster_id())
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or_else(|| {
            ModelDeploymentServiceError::MissingHpcCluster(
                target_reference.hpc_cluster_id().to_string(),
            )
        })?;

        let queue = cluster
            .queues()
            .iter()
            .find(|queue| queue.id() == target_reference.batch_scheduler_queue_id())
            .ok_or_else(|| {
                ModelDeploymentServiceError::MissingBatchSchedulerQueue(
                    target_reference.batch_scheduler_queue_id().to_string(),
                )
            })?;

        if !cluster.enabled() || !queue.enabled() {
            return Err(ModelDeploymentServiceError::DeploymentOptionUnavailable);
        }

        let snapshot = DeploymentOptionSnapshot {
            deployment_option_id: *deployment_option.id(),
            external_model_id: *deployment_option.external_model_id(),
            serving_runtime: *deployment_option.serving_runtime(),
            target: DeploymentTargetSnapshot::HpcClusterQueue(HpcClusterQueueSnapshot {
                hpc_cluster_id: *cluster.id(),
                batch_scheduler_queue_id: *queue.id(),
                cluster_host: cluster.host().into(),
                queue_name: queue.name().into(),
            }),
        };

        let replicas = ReplicaGroup {
            count: input.replicas.unwrap_or(1),
            parallelism_strategies: input.parallelism_strategies.unwrap_or_default(),
        };

        let supplied_parameters = input
            .arguments
            .iter()
            .map(|argument| (argument.parameter_name.clone(), argument.value.clone()))
            .collect::<Vec<_>>();

        let deployment_result = ModelDeploymentDomainService::deploy_with_option(
            CreateFromOptionProps {
                name: input.name,
                description: input.description,
                tenant_id: ctx.actor_tenant_id().clone(),
                owner: ctx.actor_principal_id().clone(),
                last_message: Some("Model deployment request received".into()),
                visibility: Visibility::Private,
                deployment_modality: input.deployment_modality,
                deployment_interface: None,
                replicas,
                metadata: None,
            },
            &deployment_option,
            snapshot,
            &cluster,
            queue,
            &supplied_parameters,
        )
        .map_err(|error| match error {
            ModelDeploymentDomainServiceError::DeploymentParameterError(error) => {
                ModelDeploymentServiceError::InvalidParameters(error)
            }
            error => ModelDeploymentServiceError::ModelDeploymentDomainError(error),
        })?;

        let prepared_arguments = self
            .deployment_argument_service
            .prepare_arguments(&deployment_result.resolved_parameters)
            .await?;

        let deployment = deployment_result.deployment;

        retry_async(
            || {
                self.model_deployment_repository
                    .save_with_arguments(&deployment, &prepared_arguments)
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        let payload = ModelDeploymentStartedPayload {
            deployment_id: *deployment.id(),
            message: Some("Model deployment start request reciceved".into()),
            deployment_revision: *deployment.revision(),
        };

        let event =
            Event::from_payload(&Payload::ModelDeploymentStartedPayload(payload), None, None);

        if let Err(publish_error) = retry_async(
            || self.event_publisher.publish(&event),
            &Self::EVENT_PUBLISHER_RETRY_POLICY,
            None,
        )
        .await
        {
            error!(
                "Failed to publish state drift event for deployment {}: {}",
                deployment.id(),
                publish_error
            );
        }

        Ok(DeployModelWithOptionOutput { deployment })
    }

    pub async fn start_model_deployment(
        &self,
        input: StartModelDeploymentInput,
    ) -> Result<StartModelDeploymentOutput, ModelDeploymentServiceError> {
        let deployment = self
            .update_desired_state(
                input.deployment_id,
                DesiredState::Running,
                "Requested model deployment start",
            )
            .await?;

        Ok(StartModelDeploymentOutput { deployment })
    }

    pub async fn stop_model_deployment(
        &self,
        input: StopModelDeploymentInput,
    ) -> Result<StopModelDeploymentOutput, ModelDeploymentServiceError> {
        let deployment = self
            .update_desired_state(
                input.deployment_id,
                DesiredState::Stopped,
                "Requested model deployment stop",
            )
            .await?;

        Ok(StopModelDeploymentOutput { deployment })
    }

    pub async fn undeploy_model_deployment(
        &self,
        input: UndeployModelDeploymentInput,
    ) -> Result<UndeployModelDeploymentOutput, ModelDeploymentServiceError> {
        let deployment = self
            .update_desired_state(
                input.deployment_id,
                DesiredState::NotDeployed,
                "Requested model undeployment",
            )
            .await?;

        Ok(UndeployModelDeploymentOutput { deployment })
    }

    pub async fn update(
        &self,
        input: UpdateModelDeploymentInput,
    ) -> Result<(), ModelDeploymentServiceError> {
        retry_async(
            || self.model_deployment_repository.update(&input.deployment),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(())
    }

    pub async fn record_observation(
        &self,
        _ctx: &RequestContext,
        deployment_id: &ModelDeploymentId,
        observed_at: &TimeStamp,
    ) -> Result<(), ModelDeploymentServiceError> {
        retry_async(
            || {
                self.model_deployment_repository
                    .record_observation(deployment_id, observed_at)
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(())
    }

    async fn update_desired_state(
        &self,
        deployment_id: ModelDeploymentId,
        desired_state: DesiredState,
        message: &str,
    ) -> Result<ModelDeployment, ModelDeploymentServiceError> {
        let workflow = UpdateDesiredStateWorkflow::new(
            self.model_deployment_repository.clone(),
            self.event_publisher.clone(),
        );

        workflow
            .run(UpdateDesiredStateWorkflowInput {
                deployment_id,
                desired_state,
                last_message: Some(message.into()),
            })
            .await
            .map_err(Into::into)
    }
}

pub struct ListModelDeploymentsByOwnerInput {
    pub owner: String,
}
