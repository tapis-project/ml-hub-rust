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

        let evaluated_at = TimeStamp::now();

        if deployment.is_expired(&evaluated_at) {
            return Ok(ObservationResult::Expired { deployment_id });
        }

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

        let observed_at = TimeStamp::now();

        if deployment.is_expired(&observed_at) {
            return Ok(ObservationResult::Expired { deployment_id });
        }

        let state_changed = observed.state != deployment.state;

        let target_endpoint_delta = observed.target_endpoint.unwrap_or_default();

        let target_endpoint_changed = !target_endpoint_delta.is_no_change();

        let deployment_changed = state_changed || target_endpoint_changed;

        if deployment_changed {
            let mut draft = deployment.revise();

            if state_changed {
                draft.transition_to_state_at(
                    observed.state,
                    observed.message,
                    observed_at.clone(),
                )?;
            }

            draft
                .apply_metadata_delta(observed.metadata.unwrap_or_default())
                .apply_target_endpoint_delta(target_endpoint_delta)
                .apply_replica_group_delta(observed.replicas.unwrap_or_default())
                .finish();
        }

        deployment.record_observation(observed_at.clone());

        Ok(ObservationResult::Observed {
            reconciliation_requirement: deployment.resolve_reconciliation_requirement(&observed_at),
            deployment,
            observed_at,
            deployment_changed,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ObservationResult {
    Expired {
        deployment_id: ModelDeploymentId,
    },
    Observed {
        deployment: ModelDeployment,
        observed_at: TimeStamp,
        deployment_changed: bool,
        reconciliation_requirement: Option<ReconciliationRequirement>,
    },
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
