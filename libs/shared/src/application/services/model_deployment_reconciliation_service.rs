// Built-in
use std::sync::Arc;

// Application
use crate::application::inputs::deployment::{
    FindForReconciliationInput, ReconcileModelDeploymentInput, UpdateModelDeploymentInput,
};
use crate::application::ports::deployment::{
    ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError,
};
use crate::application::ports::model::ExternalModelRepository;
use crate::application::services::deployment_argument_service::{
    DeploymentArgumentService, DeploymentArgumentServiceError,
};
use crate::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use crate::application::workflows::reconciliation::{ReconciliationAction, ReconciliationOutcome};
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
use log::error;
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
        // Ensure only system-type actors can take this action
        if ctx.actor_kind() != &Kind::System {
            return Err(ReconciliationDispatchError::InvalidActorKind(
                "Only System actors can dispatch model deployment reconcilers".into(),
            ));
        }

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

        let evaluated_at = TimeStamp::now();

        // Resolve reconciliation action
        let Some(requirement) = deployment.resolve_reconciliation_requirement(&evaluated_at) else {
            return Ok(ReconciliationResult {
                requirement: None,
                action: None,
                deployment: None,
            });
        };

        let action = match requirement {
            ReconciliationRequirement::Stop => ReconciliationAction::Stop,
            ReconciliationRequirement::Undeploy => ReconciliationAction::Undeploy,
            ReconciliationRequirement::Start => {
                // Fetch the arguments for this deployment
                let decrypted_args = self
                    .deployment_argument_service
                    .get_decrypted_arguments_for_deployment(deployment.id())
                    .await?;

                ReconciliationAction::Start {
                    payload: decrypted_args,
                }
            }
        };

        // Initialize reconciliation client
        let client = match self
            .client_provider
            .provide(&deployment.deployment_option_snapshot, &self.site_context)
            .await
        {
            Ok(c) => c,
            Err(e) => {
                self.handle_deployment_failure(
                    &mut deployment,
                    "Internal Error: Failed to initialize deployment client",
                )
                .await;

                return Err(e)?;
            }
        };

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
                self.handle_deployment_failure(
                    &mut deployment,
                    "The ExternalModel for this deployment cannot be found",
                )
                .await;

                return Err(ReconciliationDispatchError::ModelRetrievalFailed(format!(
                    "ExternalModel {} not found",
                    deployment.external_model_id
                )));
            }
            Err(error) => {
                self.handle_deployment_failure(
                    &mut deployment,
                    "Internal Error: Failed to fetch ExternalModel",
                )
                .await;

                return Err(ReconciliationDispatchError::ModelRetrievalFailed(
                    error.to_string(),
                ));
            }
        };

        if deployment.is_expired(&TimeStamp::now()) {
            return Ok(ReconciliationResult {
                requirement: None,
                action: None,
                deployment: None,
            });
        }

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
                }
            }
            ReconciliationOutcome::Stopped(payload) => {
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
                }
            }
            ReconciliationOutcome::Undeployed(payload) => {
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
                }
            }
            ReconciliationOutcome::Failed(payload) => {
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
                }
            }
            ReconciliationOutcome::Unknown(payload) => {
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
                }
            }
            ReconciliationOutcome::NoOp => ReconciliationResult {
                deployment: None,
                requirement: Some(requirement),
                action: None,
            },
        };

        Ok(result)
    }

    /// Handles deployment updates on failure.
    async fn handle_deployment_failure(
        &self,
        deployment: &mut ModelDeployment,
        reason: &str,
    ) -> () {
        if let Err(e) = deployment.mark_as_failed(Some(reason.into())) {
            error!(
                "Domain Error: Failed to mark deployment '{}' as failed: {}",
                deployment.id(),
                e.to_string()
            );
            return;
        };

        if let Err(e) = self
            .model_deployment_service
            .update(UpdateModelDeploymentInput {
                deployment: deployment.clone(),
            })
            .await
        {
            error!(
                "Infrastructure Error: Failed to update status for model deployment '{}': {}",
                deployment.id(),
                e.to_string()
            );
        };
    }
}

#[derive(Debug, Error)]
pub enum ReconciliationDispatchError {
    #[error("StaleEvent: {0}")]
    StaleEvent(String),

    #[error("Failed to retrieve deployment: {0}")]
    ModelDeploymentRetrievalFailed(#[from] ModelDeploymentServiceError),

    #[error("Failed to find model associated with deployment: {0}")]
    ModelRetrievalFailed(String),

    #[error("Model deployment domain invariant violation: {0}")]
    ModelDeploymentDomainInvariantViolation(#[from] ModelDeploymentError),

    #[error("Failed to initalize reconciliation client: {0}")]
    ReconciliationClientInitilizationFailed(#[from] ModelDeploymentReconcilerProviderError),

    #[error("Invalid Actor Kind: {0}")]
    InvalidActorKind(String),

    #[error("Argument service error: {0}")]
    DeploymentArgumentServiceError(#[from] DeploymentArgumentServiceError),
}

pub struct ReconciliationResult {
    pub deployment: Option<ModelDeployment>,
    pub requirement: Option<ReconciliationRequirement>,
    pub action: Option<ReconciliationActionTaken>,
}

pub enum ReconciliationActionTaken {
    Stopped,
    Started,
    Undeployed,
}

pub struct DispatchReconcilerInput {
    pub deployment_id: ModelDeploymentId,
    pub deployment_revision: u32,
}
