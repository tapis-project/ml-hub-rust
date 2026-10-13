use std::sync::Arc;

use crate::application::ports::deployment::{
    ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError,
};
use crate::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentError, ModelDeploymentId, ReconciliationRequirement,
};
use crate::domain::entities::principal::Kind;
use crate::domain::entities::site::SiteContext;
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::value_objects::TimeStamp;

use thiserror::Error;
pub struct ModelDeploymentObservationService {
    site_context: SiteContext,
    model_deployment_service: ModelDeploymentService,
    client_provider: Arc<dyn ModelDeploymentReconcilerProvider>,
}

impl ModelDeploymentObservationService {
    pub fn new(
        site_context: SiteContext,
        model_deployment_service: ModelDeploymentService,
        client_provider: Arc<dyn ModelDeploymentReconcilerProvider>,
    ) -> Self {
        Self {
            site_context,
            model_deployment_service,
            client_provider,
        }
    }

    pub async fn observe(
        &self,
        ctx: &RequestContext,
        deployment_id: ModelDeploymentId,
    ) -> Result<ObservationResult, ModelDeploymentObserverError> {
        // Only permit System actors to call this method
        if ctx.actor_kind() != &Kind::System {
            return Err(ModelDeploymentObserverError::Forbidden(
                "Model Deployment observation can only be performed by system actors".into(),
            ));
        }

        let mut deployment = self
            .model_deployment_service
            .find_by_id(&deployment_id)
            .await?
            .ok_or_else(|| ModelDeploymentObserverError::ModelDeploymentNotFound(deployment_id))?;

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

        // Observe
        let observed = client.observe(deployment.clone()).await;

        let deployment_changed = observed.state != deployment.state;

        // Update the state of the deployment to the observed state
        if deployment_changed {
            deployment
                .revise()
                .transition_to_state(observed.state.clone(), observed.message.clone())?
                .apply_metadata_delta(observed.metadata.unwrap_or_default())
                .apply_interface_delta(observed.interface.unwrap_or_default())
                .apply_replica_group_delta(observed.replicas.unwrap_or_default())
                .finish();
        }

        let observed_at = TimeStamp::now();

        deployment.record_observation(observed_at.clone());

        Ok(ObservationResult::new(
            deployment,
            observed_at,
            deployment_changed,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct ObservationResult {
    pub deployment: ModelDeployment,
    pub observed_at: TimeStamp,
    pub deployment_changed: bool,
    pub reconciliation_requirement: Option<ReconciliationRequirement>,
}

impl ObservationResult {
    pub fn new(
        deployment: ModelDeployment,
        observed_at: TimeStamp,
        deployment_changed: bool,
    ) -> Self {
        Self {
            reconciliation_requirement: deployment.resolve_reconciliation_requirement(),
            deployment,
            observed_at,
            deployment_changed,
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum ModelDeploymentObserverError {
    #[error(transparent)]
    ModelDeployment(#[from] ModelDeploymentError),

    #[error(transparent)]
    ModelDeploymentServiceError(#[from] ModelDeploymentServiceError),

    #[error(transparent)]
    ReconciliationProvider(#[from] ModelDeploymentReconcilerProviderError),

    #[error("Action forbidden: {0}")]
    Forbidden(String),

    #[error("Model deployment with id '{0}' not found")]
    ModelDeploymentNotFound(ModelDeploymentId),
}
