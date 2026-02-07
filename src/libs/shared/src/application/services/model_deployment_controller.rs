use std::sync::Arc;
use crate::application::inputs::deployment::{FindForReconciliationInput, ReconcileModelDeploymentInput};
use crate::application::ports::events::{
    Payload,
    EventPublisher,
};
use crate::application::ports::events::payloads::{ModelDeploymentDeletedPayload, ModelDeploymentStartedPayload, ModelDeploymentStateDriftDetectedPayload, ModelDeploymentStoppedPayload};
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::services::model_deployment_service::{ModelDeploymentService, ModelDeploymentServiceError};
use crate::application::workflows::reconciliation::{ReconciliationAction, ReconciliationError, ReconciliationOutcome};
use crate::application::ports::deployment::{ModelDeploymentPlatformReconcilerProvider, ModelDeploymentPlatformReconcilerProviderError};
use crate::domain::entities::deployment::{DesiredState, ModelDeployment, ModelDeploymentError, State};
use thiserror::Error;
use crate::application::retries::{
    retry_async,
    ExponentialBackoff,
    FixedBackoff,
    Jitter,
    Retry,
    RetryPolicy,
};
use log::error;
use once_cell::sync::Lazy;


#[derive(Debug, Error)]
pub enum ModelDeploymentControllerError {
    #[error("StaleEvent: {0}")]
    StaleEvent(String),

    #[error("Failed to retrieve deployment: {0}")]
    ModelDeploymentRetrievalFailed(#[from] ModelDeploymentServiceError),

    #[error("Failed to find model metadata associated with deployment: {0}")]
    ModelMetadataRetrievalFailed(String),

    #[error("Model deployment error: {0}")]
    ModelDeploymentError(#[from] ModelDeploymentError),

    #[error("Failed to initalize reconciliation client")]
    ReconciliationClientInitilizationFailed(#[from] ModelDeploymentPlatformReconcilerProviderError),

    #[error("Reconciliation Failed: {0}")]
    ReconciliationFailed(#[from] ReconciliationError),
}

pub struct DispatchReconcilerResult {
    events: Vec<Payload>
}

pub struct ModelDeploymentController {
    model_deployment_service: ModelDeploymentService,
    model_metadata_repo: Box<dyn ModelMetadataRepository>,
    client_provider: Box<dyn ModelDeploymentPlatformReconcilerProvider>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ModelDeploymentController {
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

    async fn dispatch_reconciler(&self, payload: ModelDeploymentStateDriftDetectedPayload) -> Result<DispatchReconcilerResult, ModelDeploymentControllerError> {
        let input = FindForReconciliationInput {
            deployment_id: payload.deployment_id.clone(),
            revision: payload.deployment_revision.clone(),
            state: payload.actual_state.clone(),
            desired_state: payload.desired_state.clone(),
        };

        let maybe_deployment = self.model_deployment_service
            .find_for_reconciliation(input)
            .await;

        let mut deployment = match maybe_deployment {
            Ok(d) => Ok(d),
            Err(err) => {
                match err {
                    ModelDeploymentServiceError::DeploymentNotFound(_) => Err(ModelDeploymentControllerError::StaleEvent("Deployment not found".into())),
                    ModelDeploymentServiceError::RevisionMismatch(expected, actual) => Err(ModelDeploymentControllerError::StaleEvent(format!("Revision mismatch: Expected revision {0}. Actual revision: {1}", expected, actual))),
                    ModelDeploymentServiceError::StateMismatch(expected, actual) => Err(ModelDeploymentControllerError::StaleEvent(format!("State mismatch: Expected state {0}. Actual state: {1}", expected, actual))),
                    ModelDeploymentServiceError::DesiredStateMismatch(expected, actual) => Err(ModelDeploymentControllerError::StaleEvent(format!("State mismatch: Expected state {0}. Actual state: {1}", expected, actual))),
                    other => Err(ModelDeploymentControllerError::from(other)) 
                }
            }
        }?;
        
        let maybe_action = Self::resolve_reconciliation_action(&deployment);

        let action = match maybe_action {
            Some(a) => a,
            None => return Ok(DispatchReconcilerResult { events: vec![] })
        };
        
        let client = self.client_provider.provide(&deployment.platform)?;

        let find_model_metadata = || self.model_metadata_repo.get_by_name_and_author(
            &deployment.model.name,
            &deployment.model.author
        );

        let maybe_model_metadata = retry_async(find_model_metadata, &Self::REPO_RETRY_POLICY)
            .await
            .map_err(|err| ModelDeploymentControllerError::ModelMetadataRetrievalFailed(err.to_string()))?;

        let model_metadata = match maybe_model_metadata {
            Some(mm) => mm,
            None => return Err(ModelDeploymentControllerError::ModelMetadataRetrievalFailed(format!("Model {}/{} not found", &deployment.model.author, &deployment.model.name)))
        };
        
        let outcome = client.reconcile(
            ReconcileModelDeploymentInput {
                action,
                deployment: deployment.clone(),
                model_metadata
            }
        ).await?;

        // Determene which event to publish based on the reconciliation outcome
        let mut events: Vec<Payload> = Vec::with_capacity(1);
        match outcome {
            ReconciliationOutcome::Observed(payload) => {
                deployment.change_state(payload.state.clone(), payload.message.clone())?;
                events.push(
                    Payload::ModelDeploymentStateDriftDetectedPayload(
                        ModelDeploymentStateDriftDetectedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            actual_state: payload.state.clone(),
                            desired_state: deployment.desired_state.clone(),
                            message: payload.message,
                        }
                    )
                )
            },
            ReconciliationOutcome::Started(payload) => {
                deployment.change_state(State::Running, payload.message.clone())?;
                events.push(
                    Payload::ModelDeploymentStartedPayload(
                        ModelDeploymentStartedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            message: payload.message,
                        }
                    )
                );
            },
            ReconciliationOutcome::Stopped(payload) => {
                deployment.change_state(State::Stopped, payload.message.clone())?;
                events.push(
                    Payload::ModelDeploymentStoppedPayload(
                        ModelDeploymentStoppedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            message: payload.message,
                        }
                    )
                )
            },
            ReconciliationOutcome::Undeployed(payload) => {
                deployment.change_state(State::NotDeployed, payload.message.clone())?;
                events.push(
                    Payload::ModelDeploymentDeletedPayload(
                        ModelDeploymentDeletedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            message: payload.message,
                        }
                    )
                )
            },
            ReconciliationOutcome::NoOp => {},
        };

        // TODO Save the deployment

        Ok(DispatchReconcilerResult { events })
    }

    /// Dermine what reconciliation action must be take to synchronize the actual state with the desired state
    fn resolve_reconciliation_action(deployment: &ModelDeployment) -> Option<ReconciliationAction> {
        if deployment.is_state_syncronized() {
            return None
        }

        match (&deployment.state, &deployment.desired_state) {
            (State::Unknown, _) => Some(ReconciliationAction::Observe),
            (State::NotDeployed, DesiredState::Running) => Some(ReconciliationAction::Start),
            (_, DesiredState::NotDeployed) => Some(ReconciliationAction::Undeploy),
            (State::Stopped, DesiredState::Running) => Some(ReconciliationAction::Start),
            (State::Failed, DesiredState::Running) => Some(ReconciliationAction::Start),
            (State::Blocked, DesiredState::Running) => Some(ReconciliationAction::Start),
            (State::Running, DesiredState::Stopped) => Some(ReconciliationAction::Stop),
            _ => None,
        }
    }
}