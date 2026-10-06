use std::sync::Arc;

use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};

use crate::application::ports::deployment::{ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError};
use crate::application::ports::events::payloads::ModelDeploymentStateDriftDetectedPayload;
use crate::application::ports::events::{EventPublisher, Payload};
use crate::application::services::model_deployment_service::ModelDeploymentService;
use crate::application::ports::model::ExternalModelRepository;
use crate::domain::entities::deployment::{ModelDeployment, ModelDeploymentError};
use crate::domain::entities::site::SiteContext;

use once_cell::sync::Lazy;
use thiserror::Error;
pub struct ModelDeploymentObservationService {
    site_context: SiteContext,
    model_deployment_service: Arc<ModelDeploymentService>,
    external_model_repo: Arc<dyn ExternalModelRepository>,
    event_publisher: Arc<dyn EventPublisher>,
    client_provider: Arc<dyn ModelDeploymentReconcilerProvider>,
}

impl ModelDeploymentObservationService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        site_context: SiteContext,
        model_deployment_service: Arc<ModelDeploymentService>,
        external_model_repo: Arc<dyn ExternalModelRepository>,
        event_publisher: Arc<dyn EventPublisher>,
        client_provider: Arc<dyn ModelDeploymentReconcilerProvider>,
    ) -> Self {
        Self {
            site_context,
            model_deployment_service,
            external_model_repo,
            event_publisher,
            client_provider,
        }
    }

    pub async fn observe(
        &self,
        deployment: &mut ModelDeployment,
    ) -> Result<Vec<Payload>, ModelDeploymentObserverError> {
        // Initialize reconciliation client
        let client = match self
            .client_provider
            .provide(&deployment.deployment_option_snapshot, &self.site_context)
            .await
        {
            Ok(c) => Ok(c),
            Err(e) => {
                // TODO Log error
                Err(e)
            }
        }?;

        // Reconcile
        let observed = client
            .observe(deployment.clone())
            .await;

        // Update the state of the deployment to the observed state
        let mut events = vec![];
        if observed.state != deployment.state {
            deployment.revise()
                .transition_to_state(observed.state.clone(), observed.message.clone())?
                .apply_metadata_delta(observed.metadata.unwrap_or_default())
                .apply_interface_delta(observed.interface.unwrap_or_default())
                .apply_replica_group_delta(observed.replicas.unwrap_or_default())
                .finish();

            events.push(Payload::ModelDeploymentStateDriftDetectedPayload(
                ModelDeploymentStateDriftDetectedPayload {
                    deployment_id: deployment.id.clone(),
                    deployment_revision: deployment.revision().clone(),
                    actual_state: observed.state.clone(),
                    desired_state: deployment.desired_state.clone(),
                    message: observed.message,
                },
            ));
        }

        Ok(events)
    }
}

#[derive(Debug, Clone, Error)]
pub enum ModelDeploymentObserverError {
    #[error(transparent)]
    ModelDeployment(#[from] ModelDeploymentError),

    #[error(transparent)]
    ReconciliationProvider(#[from] ModelDeploymentReconcilerProviderError),
}