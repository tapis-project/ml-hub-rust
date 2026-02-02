use uuid::Uuid;
use crate::application::ports::events::Event;
use crate::application::services::model_deployment_service::ModelDeploymentService;
use crate::application::workflows::reconciliation::{ReconciliationAction, ReconciliationOutcome};
use crate::domain::entities::deployment::{ModelDeployment, State, DesiredState};

struct ModelDeploymentController {
    model_deployment_service: ModelDeploymentService
}

impl ModelDeploymentController {
    pub fn handle(&self, event: &Event) {
        match event {
            Event::ModelDeploymentStateDriftDetected(payload) => {
                self.dispatch_reconciler(
                    &payload.deployment_id,
                    &payload.deployment_revision,
                    &payload.acutal_state,
                    &payload.desired_state,
                );
            },
            _ => {
                // NoOp
            }
        }
    }

    fn dispatch_reconciler(&self, deployment_id: &Uuid, revision: &u32, state: &State, desired_state: &DesiredState) {
        let deployment = self.model_deployment_service.find(deployment_id, revision, state);
        
        let action = Self::resolve_reconciliation_action(&deployment);
        
        let client = self.client_provider.provide_deployment_reconciliation_client(&deployment.platform);
        
        let outcome = client.reconcile(&action, &deployment, &model_metadata);
        
        // Determene which event to publish based on the outcome
        let maybe_event: Option<Event> = match outcome {
            ReconciliationOutcome::Observed(payload) => {
                deployment.change_state(payload.state, Some(payload.message));
                Event::ModelDeploymentStateDriftDetected(())
            },
            ReconciliationOutcome::Started => {
                deployment.change_state(State::Running, Some(payload.message));
                Event::ModelDeploymentStarted(())
            },
            ReconciliationOutcome::Stopped => {
                deployment.change_state(State::Stopped, Some(payload.message));
                Event::ModelDeploymentStopped(())
            },
            ReconciliationOutcome::Undeployed => {
                deployment.change_state(State::NotDeployed, Some(payload.message));
                Event::ModelDeploymentDeleted(())
            },
            ReconciliationOutcome::NoOp => None,
        };
        
        if let Some(event) = maybe_event {
            self.event_publisher.publish(&event)
        }
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