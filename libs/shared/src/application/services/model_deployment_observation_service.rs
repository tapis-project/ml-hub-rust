use std::sync::Arc;

use crate::application::ports::deployment::{
    ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError,
};
use crate::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use crate::application::workflows::reconciliation::FailureDisposition;
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentError, ModelDeploymentId, ReconciliationRequirement,
};
use crate::domain::entities::principal::Kind;
use crate::domain::entities::site::SiteContext;
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::value_objects::TimeStamp;

use log::{debug, info, warn};
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
        debug!(
            "Starting model deployment observation; deployment_id='{}'; request_id='{}'",
            deployment_id,
            ctx.request_id()
        );

        // Only permit System actors to call this method
        if ctx.actor_kind() != &Kind::System {
            warn!(
                "Rejected model deployment observation from non-system actor; deployment_id='{}'; actor_kind='{:?}'; request_id='{}'",
                deployment_id,
                ctx.actor_kind(),
                ctx.request_id()
            );

            return Err(ModelDeploymentObserverError::Forbidden(
                "Model Deployment observation can only be performed by system actors".into(),
            ));
        }

        let mut deployment = self
            .model_deployment_service
            .find_by_id(&deployment_id)
            .await?
            .ok_or_else(|| ModelDeploymentObserverError::ModelDeploymentNotFound(deployment_id))?;

        debug!(
            "Loaded model deployment '{}' for observation; revision={}; state='{:?}'; desired_state='{:?}'",
            deployment.id(),
            deployment.revision(),
            deployment.state,
            deployment.desired_state
        );

        let evaluated_at = TimeStamp::now();

        if deployment.is_expired(&evaluated_at) {
            info!(
                "Skipping observation for expired model deployment '{}'",
                deployment.id()
            );

            return Ok(ObservationResult::Expired { deployment_id });
        }

        if !deployment.requires_observation(&evaluated_at) {
            debug!(
                "Skipping model deployment observation because the domain no longer requires it; deployment_id='{}'",
                deployment.id()
            );

            return Ok(ObservationResult::NotRequired { deployment_id });
        }

        if deployment.is_waiting_for_retry(&evaluated_at) {
            debug!(
                "Deferring observation for model deployment '{}' until its reconciliation retry is eligible",
                deployment.id()
            );

            return Ok(ObservationResult::Deferred { deployment_id });
        }

        debug!(
            "Resolving observation client for model deployment '{}'; serving_runtime='{:?}'; target='{:?}'",
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
            Ok(c) => Ok(c),
            Err(e) => {
                // TODO Log error
                Err(e)
            }
        }?;

        debug!(
            "Invoking observation client for model deployment '{}'",
            deployment.id()
        );

        // Observe
        let observed = client.observe(deployment.clone()).await;

        debug!(
            "Observation client returned for model deployment '{}'; current_state='{:?}'; observed_state='{:?}'; message={:?}",
            deployment.id(),
            deployment.state,
            observed.state,
            observed.message
        );

        let observed_at = TimeStamp::now();

        if deployment.is_expired(&observed_at) {
            info!(
                "Model deployment '{}' expired while observation was in progress",
                deployment.id()
            );

            return Ok(ObservationResult::Expired { deployment_id });
        }

        if matches!(
            observed.failure_disposition,
            Some(FailureDisposition::Recoverable)
        ) {
            warn!(
                "Deferring model deployment observation after recoverable client failure; deployment_id='{}'",
                deployment.id()
            );

            return Ok(ObservationResult::Deferred { deployment_id });
        }

        let state_changed = observed.state != deployment.state;

        let batch_execution_outcome_changed =
            observed.batch_execution_outcome.as_ref() != deployment.batch_execution_outcome();

        let target_endpoint_delta = observed.target_endpoint.unwrap_or_default();

        let target_endpoint_changed = !target_endpoint_delta.is_no_change();

        let previous_reconciliation_status = deployment.reconciliation_status().cloned();

        if state_changed || batch_execution_outcome_changed || target_endpoint_changed {
            let mut draft = deployment.revise();

            if state_changed {
                draft.transition_to_state_at(
                    observed.state,
                    observed.message,
                    observed_at.clone(),
                )?;
            }

            draft
                .replace_batch_execution_outcome(observed.batch_execution_outcome)?
                .apply_metadata_delta(observed.metadata.unwrap_or_default())
                .apply_target_endpoint_delta(target_endpoint_delta)
                .apply_replica_group_delta(observed.replicas.unwrap_or_default())
                .finish();
        }

        if matches!(
            observed.failure_disposition,
            Some(FailureDisposition::Terminal)
        ) {
            deployment.mark_reconciliation_failed();
        } else if deployment.is_state_syncronized() {
            deployment.mark_reconciliation_synchronized()?;
        }

        let reconciliation_status_changed =
            previous_reconciliation_status.as_ref() != deployment.reconciliation_status();

        let deployment_changed = state_changed
            || batch_execution_outcome_changed
            || target_endpoint_changed
            || reconciliation_status_changed;

        debug!(
            "Calculated observation changes for model deployment '{}'; state_changed={}; batch_execution_outcome_changed={}; target_endpoint_changed={}; reconciliation_status_changed={}; deployment_changed={}",
            deployment.id(),
            state_changed,
            batch_execution_outcome_changed,
            target_endpoint_changed,
            reconciliation_status_changed,
            deployment_changed
        );

        deployment.record_observation(observed_at.clone());

        let reconciliation_requirement =
            deployment.resolve_reconciliation_requirement(&observed_at);

        debug!(
            "Completed model deployment observation; deployment_id='{}'; revision={}; state='{:?}'; reconciliation_requirement='{:?}'",
            deployment.id(),
            deployment.revision(),
            deployment.state,
            reconciliation_requirement
        );

        Ok(ObservationResult::Observed {
            reconciliation_requirement,
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
    Deferred {
        deployment_id: ModelDeploymentId,
    },
    NotRequired {
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
