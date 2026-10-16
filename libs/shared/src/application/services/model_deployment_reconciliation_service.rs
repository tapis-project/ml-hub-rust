// Built-in
use std::sync::Arc;

// Application
use crate::application::inputs::deployment::{
    FindForReconciliationInput, ReconcileModelDeploymentInput,
};
use crate::application::ports::deployment::{
    ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError,
};
use crate::application::ports::model::{ExternalModelRepository, ExternalModelRepositoryError};
use crate::application::services::deployment_argument_service::{
    DeploymentArgumentService, DeploymentArgumentServiceError,
};
use crate::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use crate::application::workflows::reconciliation::{
    FailureDisposition, ReconciliationAction, ReconciliationOutcome,
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentError, ModelDeploymentId, ReconciliationRequirement, State,
};

// Domain
use crate::domain::entities::principal::Kind;

use crate::domain::entities::site::SiteContext;
// Shared Kernel
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::value_objects::TimeStamp;

// Project libs
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};

// External
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use thiserror::Error;

pub struct ModelDeploymentReconciliationService {
    site_context: SiteContext,
    deployment_argument_service: DeploymentArgumentService,
    model_deployment_service: ModelDeploymentService,
    external_model_repo: Arc<dyn ExternalModelRepository>,
    client_provider: Arc<dyn ModelDeploymentReconcilerProvider>,
}

impl ModelDeploymentReconciliationService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        site_context: SiteContext,
        deployment_argument_service: DeploymentArgumentService,
        model_deployment_service: ModelDeploymentService,
        external_model_repo: Arc<dyn ExternalModelRepository>,
        client_provider: Arc<dyn ModelDeploymentReconcilerProvider>,
    ) -> Self {
        Self {
            site_context,
            deployment_argument_service,
            model_deployment_service,
            external_model_repo,
            client_provider,
        }
    }

    pub async fn dispatch_reconciler(
        &self,
        ctx: &RequestContext,
        input: &DispatchReconcilerInput,
    ) -> Result<ReconciliationResult, ReconciliationDispatchError> {
        debug!(
            "Dispatching model deployment reconciliation; deployment_id='{}'; requested_revision={}; request_id='{}'",
            input.deployment_id,
            input.deployment_revision,
            ctx.request_id()
        );

        // Ensure only system-type actors can take this action
        if ctx.actor_kind() != &Kind::System {
            warn!(
                "Rejected model deployment reconciliation from non-system actor; deployment_id='{}'; actor_kind='{:?}'; request_id='{}'",
                input.deployment_id,
                ctx.actor_kind(),
                ctx.request_id()
            );

            return Err(ReconciliationDispatchError::InvalidActorKind(
                "Only System actors can dispatch model deployment reconcilers".into(),
            ));
        }

        debug!(
            "Fetching model deployment '{}' at revision {} for reconciliation",
            input.deployment_id, input.deployment_revision
        );

        // Fetch deployment
        let input = FindForReconciliationInput {
            deployment_id: input.deployment_id.clone(),
            revision: input.deployment_revision.clone(),
        };

        let maybe_deployment = self
            .model_deployment_service
            .find_for_reconciliation(input)
            .await;

        let mut deployment = match maybe_deployment {
            Ok(d) => Ok(d),
            Err(e) => match e {
                ModelDeploymentServiceError::DeploymentNotFound(_) => Err(
                    ReconciliationDispatchError::StaleEvent("Deployment not found".into()),
                ),
                ModelDeploymentServiceError::RevisionMismatch(expected, actual) => {
                    Err(ReconciliationDispatchError::StaleEvent(format!(
                        "Revision mismatch: Expected revision {0}. Actual revision: {1}",
                        expected, actual
                    )))
                }
                other => Err(ReconciliationDispatchError::from(other)),
            },
        }?;

        debug!(
            "Loaded model deployment '{}' for reconciliation; revision={}; state='{:?}'; desired_state='{:?}'; external_model_id='{}'",
            deployment.id(),
            deployment.revision(),
            deployment.state,
            deployment.desired_state,
            deployment.external_model_id
        );

        let evaluated_at = TimeStamp::now();

        // Resolve reconciliation action
        let Some(requirement) = deployment.resolve_reconciliation_requirement(&evaluated_at) else {
            debug!(
                "Model deployment '{}' does not currently require reconciliation; state='{:?}'; desired_state='{:?}'; expired={}",
                deployment.id(),
                deployment.state,
                deployment.desired_state,
                deployment.is_expired(&evaluated_at)
            );

            return Ok(ReconciliationResult {
                requirement: None,
                action: None,
                deployment: None,
                failure_disposition: None,
            });
        };

        debug!(
            "Resolved '{:?}' reconciliation requirement for model deployment '{}'",
            requirement,
            deployment.id()
        );

        let action = match requirement {
            ReconciliationRequirement::Stop => ReconciliationAction::Stop,
            ReconciliationRequirement::Undeploy => ReconciliationAction::Undeploy,
            ReconciliationRequirement::Start => {
                debug!(
                    "Loading encrypted deployment arguments for model deployment '{}'",
                    deployment.id()
                );

                // Fetch the arguments for this deployment
                let decrypted_args = self
                    .deployment_argument_service
                    .get_decrypted_arguments_for_deployment(deployment.id())
                    .await?;

                let parameter_names = decrypted_args
                    .iter()
                    .map(|argument| argument.parameter_name.as_str())
                    .collect::<Vec<_>>();

                debug!(
                    "Loaded {} deployment argument(s) for model deployment '{}'; parameter_names={:?}",
                    decrypted_args.len(),
                    deployment.id(),
                    parameter_names
                );

                ReconciliationAction::Start {
                    payload: decrypted_args,
                }
            }
        };

        debug!(
            "Resolving reconciliation client for model deployment '{}'; serving_runtime='{:?}'; target='{:?}'",
            deployment.id(),
            deployment.deployment_option_snapshot.serving_runtime,
            deployment.deployment_option_snapshot.target
        );

        // Initialize reconciliation client
        let client = match self
            .client_provider
            .provide(&deployment.deployment_option_snapshot, &self.site_context)
            .await
        {
            Ok(c) => c,
            Err(e) => {
                return Err(e)?;
            }
        };

        debug!(
            "Resolved reconciliation client for model deployment '{}'",
            deployment.id()
        );

        debug!(
            "Fetching ExternalModel '{}' for model deployment '{}'",
            deployment.external_model_id,
            deployment.id()
        );

        let maybe_external_model = retry_async(
            || {
                self.external_model_repo
                    .find_by_id(&deployment.external_model_id)
            },
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await;

        // Fetch the external model for this model
        let external_model = match maybe_external_model {
            Ok(Some(external_model)) => external_model,
            Ok(None) => {
                return Err(ReconciliationDispatchError::ExternalModelNotFound(format!(
                    "ExternalModel {} not found",
                    deployment.external_model_id
                )));
            }
            Err(error) => {
                return Err(ReconciliationDispatchError::ExternalModelRepository(error));
            }
        };

        debug!(
            "Loaded ExternalModel '{}' for model deployment '{}'; provider='{}'",
            external_model.id(),
            deployment.id(),
            external_model.provider()
        );

        if deployment.is_expired(&TimeStamp::now()) {
            info!(
                "Skipping reconciliation for expired model deployment '{}'",
                deployment.id()
            );

            return Ok(ReconciliationResult {
                requirement: None,
                action: None,
                deployment: None,
                failure_disposition: None,
            });
        }

        debug!(
            "Invoking reconciliation client for model deployment '{}' with requirement '{:?}'",
            deployment.id(),
            requirement
        );

        // Reconcile
        let outcome = client
            .reconcile(ReconcileModelDeploymentInput {
                action,
                deployment: deployment.clone(),
                external_model,
            })
            .await;

        let result = match outcome {
            ReconciliationOutcome::Started(payload) => {
                debug!(
                    "Reconciliation client started model deployment '{}'; resulting_state='{:?}'; message={:?}",
                    deployment.id(),
                    payload.state,
                    payload.message
                );

                let revised = deployment
                    .revise()
                    .transition_to_state(payload.state, payload.message.clone())?
                    .apply_metadata_delta(payload.metadata.unwrap_or_default())
                    .apply_target_endpoint_delta(payload.target_endpoint.unwrap_or_default())
                    .apply_replica_group_delta(payload.replicas.unwrap_or_default())
                    .finish();

                ReconciliationResult {
                    deployment: Some(revised),
                    requirement: Some(requirement),
                    action: Some(ReconciliationActionTaken::Started),
                    failure_disposition: None,
                }
            }
            ReconciliationOutcome::Stopped(payload) => {
                debug!(
                    "Reconciliation client stopped model deployment '{}'; message={:?}",
                    deployment.id(),
                    payload.message
                );

                let revised = deployment
                    .revise()
                    .transition_to_state(State::Stopped, payload.message.clone())?
                    .apply_metadata_delta(payload.metadata.unwrap_or_default())
                    .apply_target_endpoint_delta(payload.target_endpoint.unwrap_or_default())
                    .apply_replica_group_delta(payload.replicas.unwrap_or_default())
                    .finish();

                ReconciliationResult {
                    deployment: Some(revised),
                    requirement: Some(requirement),
                    action: Some(ReconciliationActionTaken::Stopped),
                    failure_disposition: None,
                }
            }
            ReconciliationOutcome::Undeployed(payload) => {
                debug!(
                    "Reconciliation client undeployed model deployment '{}'; message={:?}",
                    deployment.id(),
                    payload.message
                );

                let mut revised = deployment.revise();

                revised.transition_to_state(State::NotDeployed, payload.message.clone())?;

                if let Some(metadata_delta) = payload.metadata {
                    revised.apply_metadata_delta(metadata_delta);
                }

                revised.apply_target_endpoint_delta(payload.target_endpoint.unwrap_or_default());

                let revised = revised.finish();

                ReconciliationResult {
                    deployment: Some(revised),
                    requirement: Some(requirement),
                    action: Some(ReconciliationActionTaken::Undeployed),
                    failure_disposition: None,
                }
            }
            ReconciliationOutcome::Failed(payload) => {
                error!(
                    "Reconciliation client failed model deployment '{}'; current_state='{:?}'; message={:?}",
                    deployment.id(),
                    deployment.state,
                    payload.message
                );

                if payload.disposition == FailureDisposition::Recoverable {
                    return Ok(ReconciliationResult {
                        deployment: Some(deployment),
                        requirement: Some(requirement),
                        action: None,
                        failure_disposition: Some(FailureDisposition::Recoverable),
                    });
                }

                if deployment.state == State::Failed {
                    warn!(
                        "Model deployment '{}' is already failed; skipping duplicate Failed-to-Failed transition",
                        deployment.id()
                    );

                    return Ok(ReconciliationResult {
                        deployment: Some(deployment),
                        requirement: Some(requirement),
                        action: None,
                        failure_disposition: Some(FailureDisposition::Terminal),
                    });
                }

                let mut revised = deployment.revise();

                revised.transition_to_state(State::Failed, payload.message.clone())?;

                if let Some(metadata_delta) = payload.metadata {
                    revised.apply_metadata_delta(metadata_delta);
                }

                revised.apply_target_endpoint_delta(payload.target_endpoint.unwrap_or_default());

                let revised = revised.finish();

                ReconciliationResult {
                    deployment: Some(revised),
                    requirement: Some(requirement),
                    action: None,
                    failure_disposition: Some(FailureDisposition::Terminal),
                }
            }
            ReconciliationOutcome::Unknown(payload) => {
                debug!(
                    "Reconciliation client returned an unknown outcome for model deployment '{}'; message={:?}",
                    deployment.id(),
                    payload.message
                );

                let mut revised = deployment.revise();

                revised.transition_to_state(State::Unknown, payload.message.clone())?;

                if let Some(metadata_delta) = payload.metadata {
                    revised.apply_metadata_delta(metadata_delta);
                }

                revised.apply_target_endpoint_delta(payload.target_endpoint.unwrap_or_default());

                let revised = revised.finish();

                ReconciliationResult {
                    deployment: Some(revised),
                    requirement: Some(requirement),
                    action: None,
                    failure_disposition: None,
                }
            }
            ReconciliationOutcome::NoOp => {
                debug!(
                    "Reconciliation client returned no operation for model deployment '{}'",
                    deployment.id()
                );

                ReconciliationResult {
                    deployment: None,
                    requirement: Some(requirement),
                    action: None,
                    failure_disposition: None,
                }
            }
        };

        debug!(
            "Model deployment '{}' reconciliation dispatch completed",
            deployment.id()
        );

        Ok(result)
    }
}

#[derive(Debug, Error)]
pub enum ReconciliationDispatchError {
    #[error("StaleEvent: {0}")]
    StaleEvent(String),

    #[error("Failed to retrieve deployment: {0}")]
    ModelDeploymentRetrievalFailed(#[from] ModelDeploymentServiceError),

    #[error("Failed to find model associated with deployment: {0}")]
    ExternalModelNotFound(String),

    #[error("Failed to load model associated with deployment: {0}")]
    ExternalModelRepository(#[from] ExternalModelRepositoryError),

    #[error("Model deployment domain invariant violation: {0}")]
    ModelDeploymentDomainInvariantViolation(#[from] ModelDeploymentError),

    #[error("Failed to initalize reconciliation client: {0}")]
    ReconciliationClientInitilizationFailed(#[from] ModelDeploymentReconcilerProviderError),

    #[error("Invalid Actor Kind: {0}")]
    InvalidActorKind(String),

    #[error("Argument service error: {0}")]
    DeploymentArgumentServiceError(#[from] DeploymentArgumentServiceError),
}

impl ReconciliationDispatchError {
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::ModelDeploymentRetrievalFailed(
                ModelDeploymentServiceError::ModelDeploymentRepoError(_)
            ) | Self::DeploymentArgumentServiceError(_)
                | Self::ExternalModelRepository(_)
        )
    }
}

pub struct ReconciliationResult {
    pub deployment: Option<ModelDeployment>,
    pub requirement: Option<ReconciliationRequirement>,
    pub action: Option<ReconciliationActionTaken>,
    pub failure_disposition: Option<FailureDisposition>,
}

#[derive(Debug)]
pub enum ReconciliationActionTaken {
    Stopped,
    Started,
    Undeployed,
}

pub struct DispatchReconcilerInput {
    pub deployment_id: ModelDeploymentId,
    pub deployment_revision: u32,
}
