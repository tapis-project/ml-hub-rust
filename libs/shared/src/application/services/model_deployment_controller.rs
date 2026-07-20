use std::sync::Arc;
use crate::application::inputs::deployment::{FindForReconciliationInput, ReconcileModelDeploymentInput, UpdateModelDeploymentInput};
use crate::application::ports::deployment_strategy::DeploymentStrategyProvider;
use crate::application::ports::events::{Event, EventPublisher, EventPublisherError, Payload};
use crate::application::ports::events::payloads::{ModelDeploymentDeletedPayload, ModelDeploymentStartedPayload, ModelDeploymentStateDriftDetectedPayload, ModelDeploymentStoppedPayload};
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::services::model_deployment_service::{ModelDeploymentService, ModelDeploymentServiceError};
use crate::application::workflows::reconciliation::{ReconciliationAction, ReconciliationError, ReconciliationOutcome};
use crate::application::ports::deployment::{ModelDeploymentPlatformReconcilerProvider, ModelDeploymentPlatformReconcilerProviderError};
use crate::domain::entities::deployment::{DesiredState,
    ModelDeployment,
    ModelDeploymentError,
    State,
    ModelDeploymentInterfaceDelta,
    ModelDeploymentMetadataDelta,
    ReplicaGroupDelta,
};
use thiserror::Error;
use retry_utils::{
    retry_async,
    FixedBackoff,
    Retry,
    RetryPolicy,
};
use log::error;
use once_cell::sync::Lazy;


#[derive(Debug, Error)]
pub enum ReconciliationDispatchError {
    #[error("StaleEvent: {0}")]
    StaleEvent(String),

    #[error("Failed to retrieve deployment: {0}")]
    ModelDeploymentRetrievalFailed(#[from] ModelDeploymentServiceError),

    #[error("Failed to find model metadata associated with deployment: {0}")]
    ModelMetadataRetrievalFailed(String),

    #[error("Model deployment domain invariant violation: {0}")]
    ModelDeploymentDomainInvariantViolation(#[from] ModelDeploymentError),

    #[error("Failed to initalize reconciliation client")]
    ReconciliationClientInitilizationFailed(#[from] ModelDeploymentPlatformReconcilerProviderError),

    #[error("Reconciliation Failed: {0}")]
    ReconciliationFailed(#[from] ReconciliationError),

    #[error("Missing deployment strategy: {0}")]
    MissingDeploymentStrategy(String),
}

#[derive(Debug, Error)]
pub enum FinishReconciliationError {
    #[error("Failed update deployment: {0}")]
    ModelDeploymentUpdateFailed(String),

    #[error("Failed to publish events: {0}")]
    EventPublicationFailed(#[from] EventPublisherError),
}

pub struct DispatchReconcilerResult {
    pub deployment: Option<ModelDeployment>,
    pub events: Vec<Payload>,
    caused_by_event: Option<Event>,
}

impl DispatchReconcilerResult {
    pub fn new(deployment: Option<ModelDeployment>, events: Vec<Payload>) -> Self {
        Self {
            deployment,
            events,
            caused_by_event: None,
        }
    }
    
    pub fn correlate_event(&mut self, event: Event) {
        self.caused_by_event = Some(event);
    }
}

pub struct ModelDeploymentController {
    deployment_strategy_provider: Arc<dyn  DeploymentStrategyProvider>,
    model_deployment_service: ModelDeploymentService,
    model_metadata_repo: Arc<dyn ModelMetadataRepository>,
    event_publisher: Arc<dyn EventPublisher>,
    client_provider: Arc<dyn ModelDeploymentPlatformReconcilerProvider>,
}

impl ModelDeploymentController {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        deployment_strategy_provider: Arc<dyn  DeploymentStrategyProvider>,
        model_deployment_service: ModelDeploymentService,
        model_metadata_repo: Arc<dyn ModelMetadataRepository>,
        event_publisher: Arc<dyn EventPublisher>,
        client_provider: Arc<dyn ModelDeploymentPlatformReconcilerProvider>,
    ) -> Self {
        Self {
            deployment_strategy_provider,
            model_deployment_service,
            model_metadata_repo,
            event_publisher,
            client_provider,
        }
    }

    pub async fn dispatch_reconciler(&self, payload: &ModelDeploymentStateDriftDetectedPayload) -> Result<DispatchReconcilerResult, ReconciliationDispatchError> {
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
                    ModelDeploymentServiceError::DeploymentNotFound(_) => Err(ReconciliationDispatchError::StaleEvent("Deployment not found".into())),
                    ModelDeploymentServiceError::RevisionMismatch(expected, actual) => Err(ReconciliationDispatchError::StaleEvent(format!("Revision mismatch: Expected revision {0}. Actual revision: {1}", expected, actual))),
                    ModelDeploymentServiceError::StateMismatch(expected, actual) => Err(ReconciliationDispatchError::StaleEvent(format!("State mismatch: Expected state {0}. Actual state: {1}", expected, actual))),
                    ModelDeploymentServiceError::DesiredStateMismatch(expected, actual) => Err(ReconciliationDispatchError::StaleEvent(format!("State mismatch: Expected state {0}. Actual state: {1}", expected, actual))),
                    other => Err(ReconciliationDispatchError::from(other)) 
                }
            }
        }?;
        
        let maybe_action = self.resolve_reconciliation_action(&deployment).await?;

        let action = match maybe_action {
            Some(a) => a,
            None => return Ok(DispatchReconcilerResult::new(None, vec![] ))
        };
        
        let client = self.client_provider.provide(&deployment.platform)?;

        let find_model_metadata = || self.model_metadata_repo.find_by_author_and_name(
            &deployment.model.author,
            &deployment.model.name,
            &deployment.model.tenant_id,
        );

        let maybe_model_metadata = retry_async(find_model_metadata, &Self::REPO_RETRY_POLICY, None)
            .await
            .map_err(|err| ReconciliationDispatchError::ModelMetadataRetrievalFailed(err.to_string()))?;

        let model_metadata = match maybe_model_metadata {
            Some(mm) => mm,
            None => return Err(ReconciliationDispatchError::ModelMetadataRetrievalFailed(format!("Model {}/{} not found", &deployment.model.author, &deployment.model.name)))
        };
        
        let outcome = client.reconcile(
            ReconcileModelDeploymentInput {
                action,
                deployment: deployment.clone(),
                model_metadata
            }
        ).await?;

        // Determine which event to publish based on the reconciliation outcome
        let mut events: Vec<Payload> = Vec::with_capacity(1);
        let maybe_modified_deployment = match outcome {
            ReconciliationOutcome::Observed(payload) => {
                let revised = deployment
                    .revise()
                    .transition_to_state(payload.state.clone(), payload.message.clone())?
                    .apply_metadata_delta(payload.metadata.unwrap_or(ModelDeploymentMetadataDelta::NoChange))
                    .apply_interface_delta(payload.interface.unwrap_or(ModelDeploymentInterfaceDelta::NoChange))
                    .apply_replica_group_delta(payload.replicas.unwrap_or(ReplicaGroupDelta::NoChange))
                    .finish();

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
                );

                Some(revised)
            },
            ReconciliationOutcome::Started(payload) => {
                let revised = deployment
                    .revise()
                    .transition_to_state(State::Running, payload.message.clone())?
                    .apply_metadata_delta(payload.metadata.unwrap_or(ModelDeploymentMetadataDelta::NoChange))
                    .apply_interface_delta(payload.interface.unwrap_or(ModelDeploymentInterfaceDelta::NoChange))
                    .apply_replica_group_delta(payload.replicas.unwrap_or(ReplicaGroupDelta::NoChange))
                    .finish();

                events.push(
                    Payload::ModelDeploymentStartedPayload(
                        ModelDeploymentStartedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            message: payload.message,
                        }
                    )
                );

                Some(revised)
            },
            ReconciliationOutcome::Stopped(payload) => {
                let revised = deployment
                    .revise()
                    .transition_to_state(State::Stopped, payload.message.clone())?
                    .apply_metadata_delta(payload.metadata.unwrap_or(ModelDeploymentMetadataDelta::NoChange))
                    .apply_interface_delta(payload.interface.unwrap_or(ModelDeploymentInterfaceDelta::NoChange))
                    .apply_replica_group_delta(payload.replicas.unwrap_or(ReplicaGroupDelta::NoChange))
                    .finish();

                events.push(
                    Payload::ModelDeploymentStoppedPayload(
                        ModelDeploymentStoppedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            message: payload.message,
                        }
                    )
                );

                Some(revised)
            },
            ReconciliationOutcome::Undeployed(payload) => {
                let mut revised = deployment.revise();

                revised
                    .transition_to_state(State::NotDeployed, payload.message.clone())?;

                if let Some(metadata_delta) = payload.metadata {
                    revised.apply_metadata_delta(metadata_delta);
                }

                let revised = revised.finish();

                events.push(
                    Payload::ModelDeploymentDeletedPayload(
                        ModelDeploymentDeletedPayload {
                            deployment_id: deployment.id.clone(),
                            deployment_revision: deployment.revision().clone(),
                            message: payload.message,
                        }
                    )
                );

                Some(revised)
            },
            ReconciliationOutcome::NoOp => { None },
        };

        Ok(DispatchReconcilerResult::new(maybe_modified_deployment, events))
    }

    pub async fn finish_reconiliation(&self, result: DispatchReconcilerResult) -> Result<(), FinishReconciliationError> {
        if let Some(deployment) = result.deployment {
            let update = UpdateModelDeploymentInput { deployment };
    
            let _ = retry_async(|| self.model_deployment_service.update(update.clone()), &Self::REPO_RETRY_POLICY, None) 
                .await
                .map_err(|err| FinishReconciliationError::ModelDeploymentUpdateFailed(err.to_string()))?;
        }

        // Publish any events returned by the controller
        for payload in result.events {
            let caused_by = match result.caused_by_event {
                Some(ref e) => Some(e),
                None => None
            };
            
            let new_event = &Event::from_payload(&payload, caused_by);
            
            let _ = self.event_publisher.publish(new_event)
                .await
                .map_err(|err| {
                    error!("Failed to publish event produced while finishing reconciliation. Event id: {}, Event kind: {}. Error: {}", new_event.metadata().id().to_string(), String::from(new_event.metadata().kind()), err.to_string());
                    FinishReconciliationError::EventPublicationFailed(err)
                })?;
        }

        Ok(())
    }

    /// Dermine what reconciliation action must be take to synchronize the actual state with the desired state
    async fn resolve_reconciliation_action(&self, deployment: &ModelDeployment) -> Result<Option<ReconciliationAction>, ReconciliationDispatchError> {
        if deployment.is_state_syncronized() {
            return Ok(None)
        }

        let maybe_strategy = self.deployment_strategy_provider
            .list_all()
            .await
            .iter()
            .find(|css| css.platform == deployment.platform.clone())
            .map(|cs| cs.strategies())
            .map(|strats| strats.iter().find(|strat| Some(strat.name.clone()) == deployment.deployment_strategy.clone()))
            .map(|maybe_strat| maybe_strat.cloned())
            .flatten();

        if deployment.deployment_strategy.is_some() && maybe_strategy.is_none() {
            return Err(ReconciliationDispatchError::MissingDeploymentStrategy(format!("During reconciliation action resolution, the referenced deployment strategy '{}' was not found", deployment.deployment_strategy.clone().unwrap_or(String::from("No name found: Impossible")))))
        }

        Ok(match (&deployment.state, &deployment.desired_state) {
            (State::Unknown, _) => Some(ReconciliationAction::Observe),
            (State::NotDeployed, DesiredState::Running) => Some(ReconciliationAction::Start { strategy: maybe_strategy }),
            (_, DesiredState::NotDeployed) => Some(ReconciliationAction::Undeploy),
            (State::Stopped, DesiredState::Running) => Some(ReconciliationAction::Start { strategy: maybe_strategy }),
            (State::Failed, DesiredState::Running) => Some(ReconciliationAction::Start { strategy: maybe_strategy }),
            (State::Blocked, DesiredState::Running) => Some(ReconciliationAction::Start { strategy: maybe_strategy }),
            (State::Running, DesiredState::Stopped) => Some(ReconciliationAction::Stop),
            _ => None,
        })
    }
}