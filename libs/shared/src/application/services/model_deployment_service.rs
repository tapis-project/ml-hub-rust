use crate::application::inputs::deployment::{
    DeployWithStrategyInput, FilterInput, FindForReconciliationInput, StartModelDeploymentInput,
    StopModelDeploymentInput, UndeployModelDeploymentInput, UpdateModelDeploymentInput,
};
use crate::application::outputs::deployment::{
    DeployModelWithStrategyOutput, StartModelDeploymentOutput, StopModelDeploymentOutput,
    UndeployModelDeploymentOutput,
};
use crate::application::ports::artifacts::ArtifactRepository;
use crate::application::ports::cipher::{Cipher, CipherError};
use crate::application::ports::deployment::{
    ModelDeploymentRepository, ModelDeploymentRepositoryError,
};
use crate::application::ports::deployment_strategy::{
    DeploymentStrategyProvider, GetStrategyByPlatformAndNameInput,
};
use crate::application::ports::events::payloads::ModelDeploymentStateDriftDetectedPayload;
use crate::application::ports::events::{Event, EventPublisher, Payload};
use crate::application::ports::model::{
    ExternalModelRepository, ExternalModelRepositoryError, ModelRepository, ModelRepositoryError,
};
use crate::application::workflows::deployment::{
    UpdateDesiredStateWorkflow, UpdateDesiredStateWorkflowInput,
};
use crate::application::workflows::Workflow;
use crate::domain::entities::deployment::{
    DeployWithStrategyProps, DesiredState, ModelDeployment, ModelDeploymentError, ModelReference,
    ReplicaGroup,
};
use crate::domain::entities::deployment_strategy::strategy::StrategyError;
use crate::domain::services::{
    ModelDeploymentDomainServiceError, ModelDeploymentService as ModelDeploymentDomainService,
};
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::enums::Visibility;
use log::error;
use once_cell::sync::Lazy;
use retry_utils::{retry_async, ExponentialBackoff, FixedBackoff, Jitter, Retry, RetryPolicy};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use super::deployment_argument_service::{
    DeploymentArgumentService, DeploymentArgumentServiceError,
};

#[derive(Debug, Error)]
pub enum ModelDeploymentServiceError {
    #[error("Revision mismatch: Expected to find revision `{0}` but found `{1}`")]
    RevisionMismatch(String, String),

    #[error("State mismatch: Expected to find state `{0}` but found `{1}`")]
    StateMismatch(String, String),

    #[error("Desired state mismatch: Expected to find desired state `{0}` but found `{1}`")]
    DesiredStateMismatch(String, String),

    #[error("Model repository error: {0}")]
    ModelRepositoryError(#[from] ModelRepositoryError),

    #[error("Model Deployment repository error: {0}")]
    ModelDeploymentRepoError(#[from] ModelDeploymentRepositoryError),

    #[error("Argument persistence error: {0}")]
    ArgumentPersistenceError(#[from] DeploymentArgumentServiceError),

    #[error("Model Deployment not found: {0}")]
    DeploymentNotFound(String),

    #[error("Model deployment error: {0}")]
    ModelDeploymentError(#[from] ModelDeploymentError),

    #[error("Model deployment domain error: {0}")]
    ModelDeploymentDomainError(#[from] ModelDeploymentDomainServiceError),

    #[error("Invalid Strategy: {0}")]
    InvalidStrategy(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(#[from] StrategyError),

    #[error("Argument encryption error: {0}")]
    ArgumentEncryptionError(#[from] CipherError),

    #[error("Model not found: {0}")]
    MissingModel(String),

    #[error("ExternalModel not found: {0}")]
    MissingExternalModel(String),

    #[error("ExternalModel repository error: {0}")]
    ExternalModelRepositoryError(#[from] ExternalModelRepositoryError),
}

pub struct ModelDeploymentService {
    deployment_argument_service: DeploymentArgumentService,
    model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
    model_repo: Arc<dyn ModelRepository>,
    external_model_repo: Arc<dyn ExternalModelRepository>,
    // TODO Leave _artifact_repo unused for now. See the link below
    // https://github.com/tapis-project/ml-hub-rust/issues/73
    _artifact_repo: Arc<dyn ArtifactRepository>,
    event_publisher: Arc<dyn EventPublisher>,
    deployment_strategy_provider: Arc<dyn DeploymentStrategyProvider>,
    cipher: Arc<dyn Cipher>,
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
        model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
        model_repo: Arc<dyn ModelRepository>,
        external_model_repo: Arc<dyn ExternalModelRepository>,
        artifact_repo: Arc<dyn ArtifactRepository>,
        event_publisher: Arc<dyn EventPublisher>,
        deployment_strategy_provider: Arc<dyn DeploymentStrategyProvider>,
        cipher: Arc<dyn Cipher>,
    ) -> Self {
        Self {
            deployment_argument_service,
            model_deployment_repo,
            model_repo,
            external_model_repo,
            _artifact_repo: artifact_repo,
            event_publisher,
            deployment_strategy_provider,
            cipher,
        }
    }

    pub async fn list_by_owner(
        &self,
        input: ListModelDeploymentsByOwnerInput,
        ctx: &RequestContext,
    ) -> Result<Vec<ModelDeployment>, ModelDeploymentServiceError> {
        let find_deployments = || {
            self.model_deployment_repo
                .find_by_owner(&ctx.actor_tenant_id(), &input.owner)
        };

        let deployments = retry_async(find_deployments, &Self::REPO_RETRY_POLICY, None).await?;

        Ok(deployments)
    }

    /// Fetches a model deployment only if its current `revision`, `state`, and
    /// `desired_state` match the expected values provided in `input`.
    ///
    /// Returns an error if the deployment does not exist or if any of the expected
    /// values differ from those stored.
    pub async fn find_for_reconciliation(
        &self,
        input: FindForReconciliationInput,
    ) -> Result<ModelDeployment, ModelDeploymentServiceError> {
        let filter = FilterInput {
            deployment_id: Some(input.deployment_id),
            state: None,
            revision: None,
        };

        let find_model_deployment = || self.model_deployment_repo.find(&filter);

        // Fetch the deployment
        let maybe_model_deployment =
            retry_async(find_model_deployment, &Self::REPO_RETRY_POLICY, None).await?;

        let deployment = match maybe_model_deployment {
            Some(d) => d,
            None => {
                return Err(ModelDeploymentServiceError::DeploymentNotFound(
                    input.deployment_id.to_string(),
                ))
            }
        };

        if deployment.revision() != &input.revision {
            return Err(ModelDeploymentServiceError::RevisionMismatch(
                input.revision.clone().to_string(),
                deployment.revision().to_string(),
            ));
        }

        if deployment.state != input.state {
            return Err(ModelDeploymentServiceError::StateMismatch(
                String::from(input.state),
                String::from(deployment.state.clone()),
            ));
        }

        if deployment.desired_state != input.desired_state {
            return Err(ModelDeploymentServiceError::DesiredStateMismatch(
                String::from(input.desired_state),
                String::from(deployment.desired_state.clone()),
            ));
        }

        Ok(deployment)
    }

    pub async fn deploy_model_with_strategy(
        &self,
        ctx: &RequestContext,
        input: DeployWithStrategyInput,
    ) -> Result<DeployModelWithStrategyOutput, ModelDeploymentServiceError> {
        // Find the strategy by platform and name
        let maybe_strategy = self
            .deployment_strategy_provider
            .get_strategy_by_platform_and_name(GetStrategyByPlatformAndNameInput {
                platform: input.platform.clone(),
                name: input.strategy_name.clone(),
            })
            .await;

        let strategy = match maybe_strategy {
            Some(s) => s,
            None => {
                return Err(ModelDeploymentServiceError::InvalidStrategy(format!(
                    "Strategy with name '{}' does not exist",
                    &input.strategy_name
                )))
            }
        };

        let find_model = || {
            self.model_repo
                .find_by_id(ctx.actor_tenant_id(), input.model_id)
        };

        // Fetch the model for this deployment.
        let maybe_model = retry_async(find_model, &Self::REPO_RETRY_POLICY, None).await?;

        let model = match maybe_model {
            Some(mm) => mm,
            None => {
                return Err(ModelDeploymentServiceError::MissingModel(
                    input.model_id.to_string(),
                ))
            }
        };

        if model.owner() != ctx.actor_principal_id()
            && !matches!(model.visibility(), Visibility::Public)
        {
            return Err(ModelDeploymentServiceError::MissingModel(
                input.model_id.to_string(),
            ));
        }

        let external_model = retry_async(
            || {
                self.external_model_repo
                    .find_by_id(model.external_model_id())
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or_else(|| {
            ModelDeploymentServiceError::MissingExternalModel(model.external_model_id().to_string())
        })?;

        let supports_strategy = external_model
            .metadata()
            .derived()
            .deployment_strategies()
            .iter()
            .any(|reference| {
                reference.name() == strategy.name && reference.platform() == &input.platform
            });

        if !supports_strategy {
            return Err(ModelDeploymentServiceError::InvalidStrategy(format!(
                "Strategy '{}' is not supported by this ExternalModel",
                input.strategy_name
            )));
        }

        // TODO Uncomment when ready. Details found in the issue below
        // https://github.com/tapis-project/ml-hub-rust/issues/73
        // let artifact_id = model.artifact_id
        //     .ok_or_else(|| ApplicationError::ModelDeploymentFailed(String::from("The model for this deployment is missing the artifact id.")))?;

        // let artifact = retry_async(|| self.artifact_repo.get_by_id(&artifact_id), &Self::REPO_RETRY_POLICY, None)
        //     .await?
        //     .ok_or_else(|| ApplicationError::ModelDeploymentFailed(format!("Artifact not found for model. Artifact required for deployment")))?;

        let replica_group = ReplicaGroup {
            count: input.replicas.unwrap_or(1),
            parallelism_strategies: input.parallelism_strategies.unwrap_or(vec![]),
        };

        // Initialize props to deploy with strategy
        let props = DeployWithStrategyProps {
            id: Uuid::now_v7(),
            name: input.name,
            description: input.description,
            tenant_id: ctx.actor_tenant_id().clone(),
            platform: input.platform.clone(),
            owner: ctx.actor_principal_id().clone(),
            model: ModelReference {
                model_id: *model.id(),
            },
            deployment_modality: input.deployment_modality.clone(),
            last_message: Some("Model deployment request recieved".into()),
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: replica_group,
            metadata: None,
        };

        // Validate invariants for domain deployment
        let deployment = ModelDeploymentDomainService::new(self.cipher.clone())
            .deploy_model_with_strategy(&external_model, props, &strategy)
            .await?;

        // Save the deployment
        retry_async(
            || self.model_deployment_repo.save(&deployment),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        // Save the arguments
        self.deployment_argument_service
            .save(&deployment, &strategy, &input.arguments)
            .await?;

        // Build state drift event payload
        let payload = ModelDeploymentStateDriftDetectedPayload {
            deployment_id: deployment.id,
            message: Some("Model deployment initiated with StateDriftDetected event".into()),
            deployment_revision: deployment.revision().clone(),
            desired_state: deployment.desired_state.clone(),
            actual_state: deployment.state.clone(),
        };

        // Build the event from the payload
        let event = Event::from_payload(
            &Payload::ModelDeploymentStateDriftDetectedPayload(payload),
            None,
        );

        // Closure for publishing model deployment
        let publish_state_drift_event = || self.event_publisher.publish(&event);

        // Publish the state drift event
        match retry_async(
            publish_state_drift_event,
            &Self::EVENT_PUBLISHER_RETRY_POLICY,
            None,
        )
        .await
        {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Failed to publish state drift event for deployment {}: {}",
                    &deployment.id,
                    e.to_string()
                );
            }
        };

        Ok(DeployModelWithStrategyOutput { deployment })
    }

    pub async fn start_model_deployment(
        &self,
        input: StartModelDeploymentInput,
    ) -> Result<StartModelDeploymentOutput, ModelDeploymentServiceError> {
        let workflow = UpdateDesiredStateWorkflow::new(
            self.model_deployment_repo.clone(),
            self.event_publisher.clone(),
        );

        let modified_deployment = workflow
            .run(UpdateDesiredStateWorkflowInput {
                deployment_id: input.deployment_id.clone(),
                desired_state: DesiredState::Running,
                last_message: Some("Requested model deployment start".into()),
            })
            .await?;

        Ok(StartModelDeploymentOutput {
            deployment: modified_deployment,
        })
    }

    pub async fn stop_model_deployment(
        &self,
        input: StopModelDeploymentInput,
    ) -> Result<StopModelDeploymentOutput, ModelDeploymentServiceError> {
        let workflow = UpdateDesiredStateWorkflow::new(
            self.model_deployment_repo.clone(),
            self.event_publisher.clone(),
        );

        let modified_deployment = workflow
            .run(UpdateDesiredStateWorkflowInput {
                deployment_id: input.deployment_id.clone(),
                desired_state: DesiredState::Stopped,
                last_message: Some("Requested model deployment start".into()),
            })
            .await?;

        Ok(StopModelDeploymentOutput {
            deployment: modified_deployment,
        })
    }

    pub async fn undeploy_model_deployment(
        &self,
        input: UndeployModelDeploymentInput,
    ) -> Result<UndeployModelDeploymentOutput, ModelDeploymentServiceError> {
        let workflow = UpdateDesiredStateWorkflow::new(
            self.model_deployment_repo.clone(),
            self.event_publisher.clone(),
        );

        let modified_deployment = workflow
            .run(UpdateDesiredStateWorkflowInput {
                deployment_id: input.deployment_id.clone(),
                desired_state: DesiredState::NotDeployed,
                last_message: Some("Requested model deployment start".into()),
            })
            .await?;

        Ok(UndeployModelDeploymentOutput {
            deployment: modified_deployment,
        })
    }

    pub async fn update(
        &self,
        input: UpdateModelDeploymentInput,
    ) -> Result<(), ModelDeploymentServiceError> {
        // Update the deployment
        retry_async(
            || self.model_deployment_repo.update(&input.deployment),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        Ok(())
    }
}

pub struct ListModelDeploymentsByOwnerInput {
    pub owner: String,
}
