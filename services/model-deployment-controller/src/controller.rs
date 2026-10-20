use std::collections::HashSet;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use amqprs::{channel::Channel, consumer::AsyncConsumer, BasicProperties, Deliver};
use async_trait::async_trait;
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use rand::random_range;
use shared::application::inputs::deployment::UpdateModelDeploymentInput;
use shared::application::ports::events::Event;
use shared::application::services::endpoint_issuance_service::EndpointIssuanceService;
use shared::application::services::model_deployment_observation_service::{
    ModelDeploymentObservationService, ObservationResult,
};
use shared::application::services::model_deployment_reconciliation_service::{
    DispatchReconcilerInput, ModelDeploymentReconciliationService, ReconciliationDispatchError,
};
use shared::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use shared::application::workflows::reconciliation::FailureDisposition;
use shared::domain::entities::deployment::{
    ModelDeploymentId, ReconciliationRequirement, ReconciliationStatus,
};
use shared::domain::entities::endpoint::NetworkAddressableResource;
use shared::infra::messaging::codec::deserialize_event_message;
use shared::infra::messaging::rabbitmq::settlement::{ack, nack};
use shared::shared_kernel::context::RequestContext;
use shared::shared_kernel::value_objects::TimeStamp;
use thiserror::Error;
use tokio::sync::Notify;
use tokio::time::MissedTickBehavior;

struct ObservationSet(Mutex<HashSet<ModelDeploymentId>>);

impl ObservationSet {
    fn new() -> Self {
        Self(Mutex::new(HashSet::new()))
    }

    async fn add(&self, id: ModelDeploymentId) -> Result<(), ControlLoopStateError> {
        let mut set = self
            .0
            .lock()
            .map_err(|_| ControlLoopStateError::LockPoisoned("observation set"))?;

        set.insert(id);

        Ok(())
    }

    async fn remove(&self, id: &ModelDeploymentId) -> Result<(), ControlLoopStateError> {
        let mut set = self
            .0
            .lock()
            .map_err(|_| ControlLoopStateError::LockPoisoned("observation set"))?;

        set.remove(id);

        Ok(())
    }
}

struct ReconciliationQueue {
    jobs: Mutex<IndexMap<ModelDeploymentId, ReconciliationJob>>,
    notification: Notify,
}

impl ReconciliationQueue {
    fn new() -> Self {
        Self {
            jobs: Mutex::new(IndexMap::new()),
            notification: Notify::new(),
        }
    }

    async fn enqueue(&self, job: ReconciliationJob) -> Result<(), ControlLoopStateError> {
        let deployment_id = job.deployment_id;

        let deployment_revision = job.deployment_revision;

        let reconciliation_requirement = job.reconciliation_requirement.clone();

        {
            let mut queue = self
                .jobs
                .lock()
                .map_err(|_| ControlLoopStateError::LockPoisoned("reconciliation queue"))?;

            let replaced = queue.shift_remove(&deployment_id).is_some();

            queue.insert(deployment_id, job);

            debug!(
                "Queued model deployment '{}' at revision {} for '{:?}' reconciliation; replaced_existing_job={}; queue_depth={}",
                deployment_id,
                deployment_revision,
                reconciliation_requirement,
                replaced,
                queue.len()
            );
        }

        self.notification.notify_one();

        Ok(())
    }

    async fn dequeue(&self) -> Result<ReconciliationJob, ControlLoopStateError> {
        loop {
            let notified = self.notification.notified();

            let (maybe_job, earliest_retry_after) = {
                let mut queue = self
                    .jobs
                    .lock()
                    .map_err(|_| ControlLoopStateError::LockPoisoned("reconciliation queue"))?;

                let scan_length = queue.len();

                let now = TimeStamp::now();

                let mut ready = None;

                let mut earliest_retry_after: Option<TimeStamp> = None;

                for _ in 0..scan_length {
                    let Some((deployment_id, job)) = queue.shift_remove_index(0) else {
                        break;
                    };

                    let delayed = job
                        .retry_after
                        .as_ref()
                        .is_some_and(|retry_after| retry_after.into_inner() > now.into_inner());

                    if delayed {
                        if let Some(job_retry_after) = job.retry_after.as_ref() {
                            let is_earlier = earliest_retry_after
                                .as_ref()
                                .map(|earliest| {
                                    job_retry_after.into_inner() < earliest.into_inner()
                                })
                                .unwrap_or(true);

                            if is_earlier {
                                earliest_retry_after = Some(job_retry_after.clone());
                            }
                        }

                        queue.insert(deployment_id, job);

                        continue;
                    }

                    ready = Some(job);

                    break;
                }

                (ready, earliest_retry_after)
            };

            if let Some(job) = maybe_job {
                debug!(
                    "Dequeued model deployment '{}' at revision {} for '{:?}' reconciliation",
                    job.deployment_id, job.deployment_revision, job.reconciliation_requirement
                );

                return Ok(job);
            }

            if let Some(retry_after) = earliest_retry_after {
                let wait = retry_after
                    .into_inner()
                    .signed_duration_since(TimeStamp::now().into_inner())
                    .to_std()
                    .unwrap_or_default();

                debug!(
                    "No reconciliation job is eligible; waiting up to {:?} for the earliest retry",
                    wait
                );

                tokio::select! {
                    _ = tokio::time::sleep(wait) => {}
                    _ = notified => {}
                }
            } else {
                debug!("Reconciliation queue is empty; waiting for work");

                notified.await;
            }
        }
    }

    async fn remove(&self, deployment_id: &ModelDeploymentId) -> Result<(), ControlLoopStateError> {
        let mut queue = self
            .jobs
            .lock()
            .map_err(|_| ControlLoopStateError::LockPoisoned("reconciliation queue"))?;

        queue.shift_remove(deployment_id);

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ReconciliationRetryConfiguration {
    initial_delay: Duration,
    multiplier: u32,
    maximum_delay: Duration,
    maximum_retries: u32,
}

impl ReconciliationRetryConfiguration {
    pub fn from_environment() -> Self {
        let initial_delay_seconds = read_u64("RECONCILIATION_RETRY_INITIAL_SECONDS", 30).max(1);
        let maximum_delay_seconds =
            read_u64("RECONCILIATION_RETRY_MAXIMUM_SECONDS", 900).max(initial_delay_seconds);

        Self {
            initial_delay: Duration::from_secs(initial_delay_seconds),
            multiplier: read_u32("RECONCILIATION_RETRY_MULTIPLIER", 2).max(1),
            maximum_delay: Duration::from_secs(maximum_delay_seconds),
            maximum_retries: read_u32("RECONCILIATION_MAXIMUM_RETRIES", 5),
        }
    }

    fn retry_after(&self, retries: u32) -> TimeStamp {
        let exponent = retries.saturating_sub(1);

        let multiplier = self.multiplier.saturating_pow(exponent);

        let delay = self
            .initial_delay
            .saturating_mul(multiplier)
            .min(self.maximum_delay);

        let jittered_millis = random_range(0..=delay.as_millis().min(u64::MAX as u128) as u64);

        let retry_after =
            TimeStamp::now().into_inner() + chrono::Duration::milliseconds(jittered_millis as i64);

        TimeStamp::from(retry_after)
    }
}

fn read_u64(name: &str, default: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn read_u32(name: &str, default: u32) -> u32 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

pub(crate) struct ControlLoopState {
    observation_set: ObservationSet,
    reconciliation_queue: ReconciliationQueue,
}

impl ControlLoopState {
    pub(crate) fn new() -> Self {
        Self {
            observation_set: ObservationSet::new(),
            reconciliation_queue: ReconciliationQueue::new(),
        }
    }

    pub(crate) fn get_observation_snapshot(
        &self,
    ) -> Result<Vec<ModelDeploymentId>, ControlLoopStateError> {
        let deployment_ids = self
            .observation_set
            .0
            .lock()
            .map_err(|_| ControlLoopStateError::LockPoisoned("observation set"))?
            .iter()
            .cloned()
            .collect();

        Ok(deployment_ids)
    }
}

#[derive(Debug, Clone, Error)]
pub enum ControlLoopStateError {
    #[error("The {0} lock is poisoned")]
    LockPoisoned(&'static str),
}

#[derive(Clone)]
pub struct ModelDeploymentController {
    model_deployment_service: Arc<ModelDeploymentService>,
    reconciliation_service: Arc<ModelDeploymentReconciliationService>,
    observer_service: Arc<ModelDeploymentObservationService>,
    endpoint_issuance_service: Arc<EndpointIssuanceService>,
    state: Arc<ControlLoopState>,
    retry_configuration: ReconciliationRetryConfiguration,
}

#[derive(Debug, Clone, Error)]
pub enum ModelDeploymentControllerInitializationError {
    #[error(transparent)]
    FailedToFetch(#[from] ModelDeploymentServiceError),

    #[error(transparent)]
    ControlLoopState(#[from] ControlLoopStateError),
}

impl ModelDeploymentController {
    const OBSERVATION_INTERVAL: Duration = Duration::from_secs(30);
    const SYNCHRONIZATION_INTERVAL: Duration = Duration::from_secs(12 * 60 * 60);

    pub(crate) async fn new(
        model_deployment_service: Arc<ModelDeploymentService>,
        reconciliation_service: Arc<ModelDeploymentReconciliationService>,
        observer_service: Arc<ModelDeploymentObservationService>,
        endpoint_issuance_service: Arc<EndpointIssuanceService>,
        state: Arc<ControlLoopState>,
        retry_configuration: ReconciliationRetryConfiguration,
    ) -> Result<Self, ModelDeploymentControllerInitializationError> {
        let controller = Self {
            model_deployment_service,
            reconciliation_service,
            observer_service,
            endpoint_issuance_service,
            state,
            retry_configuration,
        };

        info!("Initializing model deployment controller state");

        controller.synchronize().await?;

        info!("Model deployment controller state initialized");

        Ok(controller)
    }

    async fn synchronize(&self) -> Result<(), ModelDeploymentControllerInitializationError> {
        debug!("Loading observable and reconcilable model deployments for control-loop synchronization");

        let context = RequestContext::system(None, None, None);

        let candidates = self
            .model_deployment_service
            .find_control_loop_candidates(&context)
            .await?;

        info!(
            "Synchronizing {} observable and {} reconcilable model deployment(s)",
            candidates.observable.len(),
            candidates.reconcilable.len()
        );

        for deployment in candidates.observable {
            debug!(
                "Registering model deployment '{}' for observation; revision={}; state='{:?}'; desired_state='{:?}'",
                deployment.id(),
                deployment.revision(),
                deployment.state,
                deployment.desired_state
            );

            self.register_for_observation(*deployment.id()).await?;
        }

        for mut deployment in candidates.reconcilable {
            let deployment_id = *deployment.id();

            let deployment_revision = *deployment.revision();

            if deployment.reconciliation_status().is_none() {
                deployment.accept_for_reconciliation();

                self.model_deployment_service
                    .update_at_revision(
                        UpdateModelDeploymentInput {
                            deployment: deployment.clone(),
                        },
                        deployment_revision,
                    )
                    .await?;
            }

            // Invariant: reconciliation is a subset of observation. Registration is
            // idempotent and repairs any concurrent or query-level omission.
            self.register_for_observation(*deployment.id()).await?;

            let evaluated_at = deployment
                .retry_after()
                .cloned()
                .unwrap_or_else(TimeStamp::now);

            if let Some(requirement) = deployment.resolve_reconciliation_requirement(&evaluated_at)
            {
                debug!(
                    "Synchronized model deployment '{}' requires '{:?}' reconciliation",
                    deployment_id, requirement
                );

                self.queue_for_reconciliation(ReconciliationJob {
                    deployment_id,
                    deployment_revision,
                    reconciliation_requirement: requirement,
                    retries: deployment.retries(),
                    retry_after: deployment.retry_after().cloned(),
                    context: RequestContext::system(None, None, None),
                })
                .await?;
            } else {
                debug!(
                    "Synchronized model deployment '{}' does not require reconciliation",
                    deployment_id
                );
            }
        }

        debug!("Model deployment work queue synchronization completed");

        Ok(())
    }

    pub async fn register_for_observation(
        &self,
        deployment_id: ModelDeploymentId,
    ) -> Result<(), ControlLoopStateError> {
        debug!(
            "Registering model deployment '{}' for observation",
            deployment_id
        );

        self.state.observation_set.add(deployment_id).await
    }

    pub async fn unregister_from_observation(
        &self,
        deployment_id: &ModelDeploymentId,
    ) -> Result<(), ControlLoopStateError> {
        debug!(
            "Removing model deployment '{}' from observation",
            deployment_id
        );

        self.state.observation_set.remove(deployment_id).await
    }

    pub async fn queue_for_reconciliation(
        &self,
        job: ReconciliationJob,
    ) -> Result<(), ControlLoopStateError> {
        self.state.reconciliation_queue.enqueue(job).await
    }

    pub async fn remove_from_reconciliation_queue(
        &self,
        deployment_id: &ModelDeploymentId,
    ) -> Result<(), ControlLoopStateError> {
        self.state.reconciliation_queue.remove(deployment_id).await
    }

    pub async fn run_reconciliation_loop(&self) -> Result<(), ControlLoopStateError> {
        info!("Model deployment reconciliation loop started");

        loop {
            let job = self.state.reconciliation_queue.dequeue().await?;

            debug!(
                "Dispatching reconciliation for model deployment '{}' at revision {} with queued requirement '{:?}'; request_id='{}'",
                job.deployment_id,
                job.deployment_revision,
                job.reconciliation_requirement,
                job.context.request_id()
            );

            let find_input = shared::application::inputs::deployment::FindForReconciliationInput {
                deployment_id: job.deployment_id,
                revision: job.deployment_revision,
            };

            let mut current = match self
                .model_deployment_service
                .find_for_reconciliation(find_input)
                .await
            {
                Ok(deployment) => deployment,
                Err(err) => {
                    warn!(
                        "Discarding stale or unavailable reconciliation job for deployment '{}' at revision {}: {}",
                        job.deployment_id, job.deployment_revision, err
                    );

                    continue;
                }
            };

            let evaluated_at = TimeStamp::now();

            if current.retries() != job.retries {
                debug!(
                    "Queued retry count for deployment '{}' is stale; queued={}; persisted={}",
                    job.deployment_id,
                    job.retries,
                    current.retries()
                );
            }

            let Some(current_requirement) =
                current.resolve_reconciliation_requirement(&evaluated_at)
            else {
                debug!(
                    "Discarding reconciliation job for deployment '{}' because reconciliation is no longer required",
                    job.deployment_id
                );

                continue;
            };

            if current_requirement != job.reconciliation_requirement {
                debug!(
                    "Discarding reconciliation job for deployment '{}' because requirement changed from '{:?}' to '{:?}'",
                    job.deployment_id, job.reconciliation_requirement, current_requirement
                );

                continue;
            }

            current.begin_reconciliation(&current_requirement);

            if let Err(err) = self
                .model_deployment_service
                .update_at_revision(
                    UpdateModelDeploymentInput {
                        deployment: current,
                    },
                    job.deployment_revision,
                )
                .await
            {
                warn!(
                    "Could not claim reconciliation job for deployment '{}' at revision {}: {}",
                    job.deployment_id, job.deployment_revision, err
                );

                continue;
            }

            let input = DispatchReconcilerInput {
                deployment_id: job.deployment_id,
                deployment_revision: job.deployment_revision,
            };

            let reconciliation_result = match self
                .reconciliation_service
                .dispatch_reconciler(&job.context, &input)
                .await
            {
                Ok(result) => result,
                Err(err) => {
                    let recoverable = err.is_recoverable();

                    Self::log_reconciliation_error(&job, err);

                    self.finish_failed_reconciliation(&job, recoverable).await;

                    continue;
                }
            };

            let Some(mut deployment) = reconciliation_result.deployment else {
                debug!(
                    "Reconciliation for model deployment '{}' completed without a deployment update; resolved_requirement='{:?}'; action='{:?}'",
                    job.deployment_id,
                    reconciliation_result.requirement,
                    reconciliation_result.action
                );

                continue;
            };

            let recoverable_failure = matches!(
                reconciliation_result.failure_disposition,
                Some(FailureDisposition::Recoverable)
            );

            if recoverable_failure {
                self.schedule_retry(&job, deployment).await;

                continue;
            }

            if matches!(
                reconciliation_result.failure_disposition,
                Some(FailureDisposition::Terminal)
            ) {
                deployment.mark_reconciliation_failed();
            } else if deployment.is_state_syncronized() {
                if let Err(err) = deployment.mark_reconciliation_synchronized() {
                    error!(
                        "Failed to mark model deployment '{}' synchronized: {}",
                        deployment.id(),
                        err
                    );

                    continue;
                }
            }

            let deployment_id = *deployment.id();

            let deployment_revision = *deployment.revision();

            let deployment_state = deployment.state.clone();

            let update = UpdateModelDeploymentInput { deployment };

            debug!(
                "Persisting reconciled model deployment '{}' at revision {} with state '{:?}'; resolved_requirement='{:?}'; action='{:?}'",
                deployment_id,
                deployment_revision,
                deployment_state,
                reconciliation_result.requirement,
                reconciliation_result.action
            );

            match self
                .model_deployment_service
                .update_at_revision(update, job.deployment_revision)
                .await
            {
                Ok(()) => {
                    info!(
                        "Persisted reconciled model deployment '{}' at revision {} with state '{:?}'",
                        deployment_id, deployment_revision, deployment_state
                    );
                }
                Err(err) => {
                    error!(
                        "Failed to persist reconciled model deployment '{}' at revision {}: {}",
                        deployment_id, deployment_revision, err
                    );
                }
            }
        }
    }

    pub async fn run_observation_loop(&self) -> Result<(), ControlLoopStateError> {
        let mut interval = tokio::time::interval(Self::OBSERVATION_INTERVAL);

        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!(
            "Model deployment observation loop started with interval {:?}",
            Self::OBSERVATION_INTERVAL
        );

        loop {
            interval.tick().await;

            let deployment_ids = self.state.get_observation_snapshot()?;

            debug!(
                "Starting observation pass for {} model deployment(s)",
                deployment_ids.len()
            );

            for deployment_id in deployment_ids {
                let context = RequestContext::system(None, None, None);

                debug!(
                    "Observing model deployment '{}'; request_id='{}'",
                    deployment_id,
                    context.request_id()
                );

                let result = match self.observer_service.observe(&context, deployment_id).await {
                    Ok(result) => result,
                    Err(err) => {
                        error!(
                            "Failed to observe model deployment '{}': {}",
                            deployment_id, err
                        );

                        continue;
                    }
                };

                let (deployment, observed_at, deployment_changed, reconciliation_requirement) =
                    match result {
                        ObservationResult::Expired { deployment_id } => {
                            info!(
                                "Model deployment '{}' has expired; removing it from observation",
                                deployment_id
                            );

                            self.unregister_from_observation(&deployment_id).await?;

                            self.remove_from_reconciliation_queue(&deployment_id)
                                .await?;

                            continue;
                        }
                        ObservationResult::Deferred { deployment_id } => {
                            debug!(
                                "Observation deferred for model deployment '{}' while its reconciliation retry deadline is pending",
                                deployment_id
                            );

                            continue;
                        }
                        ObservationResult::NotRequired { deployment_id } => {
                            debug!(
                                "Model deployment '{}' no longer requires observation; removing it from control-loop work",
                                deployment_id
                            );

                            self.unregister_from_observation(&deployment_id).await?;

                            self.remove_from_reconciliation_queue(&deployment_id)
                                .await?;

                            continue;
                        }
                        ObservationResult::Observed {
                            deployment,
                            observed_at,
                            deployment_changed,
                            reconciliation_requirement,
                        } => (
                            deployment,
                            observed_at,
                            deployment_changed,
                            reconciliation_requirement,
                        ),
                    };

                let observed_deployment_id = *deployment.id();

                let observed_deployment_revision = *deployment.revision();

                let observed_retries = deployment.retries();

                let observed_retry_after = deployment.retry_after().cloned();

                let endpoint_resource = deployment.clone();

                debug!(
                    "Observed model deployment '{}' at revision {}; state='{:?}'; changed={}; reconciliation_requirement='{:?}'",
                    observed_deployment_id,
                    observed_deployment_revision,
                    deployment.state,
                    deployment_changed,
                    reconciliation_requirement
                );

                let persistence_result = if deployment_changed {
                    debug!(
                        "Persisting observed changes for model deployment '{}'",
                        observed_deployment_id
                    );

                    self.model_deployment_service
                        .update(UpdateModelDeploymentInput { deployment })
                        .await
                } else {
                    debug!(
                        "Recording observation timestamp for unchanged model deployment '{}'",
                        observed_deployment_id
                    );

                    self.model_deployment_service
                        .record_observation(&context, &observed_deployment_id, &observed_at)
                        .await
                };

                if let Err(err) = persistence_result {
                    error!(
                        "Failed to persist observed model deployment '{}': {}",
                        observed_deployment_id, err
                    );

                    continue;
                }

                if !endpoint_resource.requires_observation(&observed_at) {
                    debug!(
                        "Observed model deployment '{}' no longer requires observation; removing it from control-loop work",
                        observed_deployment_id
                    );

                    self.unregister_from_observation(&observed_deployment_id)
                        .await?;

                    self.remove_from_reconciliation_queue(&observed_deployment_id)
                        .await?;

                    continue;
                }

                if !endpoint_resource.network_target_names().is_empty() {
                    trace!(
                        "Ensuring endpoint exists for observed model deployment '{}'",
                        observed_deployment_id
                    );

                    if let Err(err) = self
                        .endpoint_issuance_service
                        .issue_for_resource(&context, &endpoint_resource)
                        .await
                    {
                        error!(
                            "Failed to issue endpoint for observed model deployment '{}': {}",
                            observed_deployment_id, err
                        );
                    }
                }

                if let Some(requirement) = reconciliation_requirement {
                    debug!(
                        "Observation queued model deployment '{}' at revision {} for '{:?}' reconciliation",
                        observed_deployment_id, observed_deployment_revision, requirement
                    );

                    self.queue_for_reconciliation(ReconciliationJob {
                        deployment_id: observed_deployment_id,
                        deployment_revision: observed_deployment_revision,
                        reconciliation_requirement: requirement,
                        retries: observed_retries,
                        retry_after: observed_retry_after,
                        context,
                    })
                    .await?;
                } else if endpoint_resource.is_state_syncronized()
                    || matches!(
                        endpoint_resource.reconciliation_status(),
                        Some(ReconciliationStatus::Failed)
                    )
                {
                    self.remove_from_reconciliation_queue(&observed_deployment_id)
                        .await?;
                }
            }

            debug!("Model deployment observation pass completed");
        }
    }

    async fn schedule_retry(
        &self,
        job: &ReconciliationJob,
        mut deployment: shared::domain::entities::deployment::ModelDeployment,
    ) {
        if deployment.retries() >= self.retry_configuration.maximum_retries {
            deployment.mark_reconciliation_failed();

            if let Err(err) = self
                .model_deployment_service
                .update_at_revision(
                    UpdateModelDeploymentInput { deployment },
                    job.deployment_revision,
                )
                .await
            {
                error!(
                    "Failed to persist terminal reconciliation failure for deployment '{}': {}",
                    job.deployment_id, err
                );
            }

            return;
        }

        let retry_number = deployment.retries().saturating_add(1);

        let retry_after = self.retry_configuration.retry_after(retry_number);

        if let Err(err) = deployment.schedule_reconciliation_retry(retry_after.clone()) {
            error!(
                "Failed to schedule reconciliation retry for deployment '{}': {}",
                job.deployment_id, err
            );

            return;
        }

        if let Err(err) = self
            .model_deployment_service
            .update_at_revision(
                UpdateModelDeploymentInput {
                    deployment: deployment.clone(),
                },
                job.deployment_revision,
            )
            .await
        {
            error!(
                "Failed to persist reconciliation retry for deployment '{}': {}",
                job.deployment_id, err
            );

            return;
        }

        info!(
            "Scheduled reconciliation retry {} of {} for deployment '{}' at {}",
            deployment.retries(),
            self.retry_configuration.maximum_retries,
            job.deployment_id,
            String::from(retry_after.clone())
        );

        if let Err(err) = self
            .queue_for_reconciliation(ReconciliationJob {
                deployment_id: job.deployment_id,
                deployment_revision: job.deployment_revision,
                reconciliation_requirement: job.reconciliation_requirement.clone(),
                retries: deployment.retries(),
                retry_after: Some(retry_after),
                context: job.context.clone(),
            })
            .await
        {
            error!(
                "Failed to requeue deployment '{}' for reconciliation: {}",
                job.deployment_id, err
            );
        }
    }

    async fn finish_failed_reconciliation(&self, job: &ReconciliationJob, recoverable: bool) {
        let input = shared::application::inputs::deployment::FindForReconciliationInput {
            deployment_id: job.deployment_id,
            revision: job.deployment_revision,
        };

        let mut deployment = match self
            .model_deployment_service
            .find_for_reconciliation(input)
            .await
        {
            Ok(deployment) => deployment,
            Err(err) => {
                warn!(
                    "Could not reload deployment '{}' after reconciliation failure: {}",
                    job.deployment_id, err
                );

                return;
            }
        };

        if recoverable {
            self.schedule_retry(job, deployment).await;

            return;
        }

        deployment.mark_reconciliation_failed();

        if let Err(err) = self
            .model_deployment_service
            .update_at_revision(
                UpdateModelDeploymentInput { deployment },
                job.deployment_revision,
            )
            .await
        {
            error!(
                "Failed to persist terminal reconciliation failure for deployment '{}': {}",
                job.deployment_id, err
            );
        }
    }

    fn log_reconciliation_error(job: &ReconciliationJob, err: ReconciliationDispatchError) {
        match err {
            ReconciliationDispatchError::StaleEvent(err) => {
                warn!(
                    "Discarding stale reconciliation job for deployment '{}' at revision {} with requirement '{:?}': {}",
                    job.deployment_id, job.deployment_revision, job.reconciliation_requirement, err
                );
            }
            err => {
                error!(
                    "Failed to reconcile deployment '{}' at revision {} with requirement '{:?}': {}",
                    job.deployment_id, job.deployment_revision, job.reconciliation_requirement, err
                );
            }
        }
    }

    pub async fn run_synchronization_loop(&self) {
        let mut interval = tokio::time::interval(Self::SYNCHRONIZATION_INTERVAL);

        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        interval.tick().await;

        info!(
            "Model deployment synchronization loop started with interval {:?}",
            Self::SYNCHRONIZATION_INTERVAL
        );

        loop {
            interval.tick().await;

            debug!("Starting scheduled model deployment queue synchronization");

            if let Err(err) = self.synchronize().await {
                error!(
                    "Failed to synchronize model deployment work queues: {}",
                    err
                );
            }
        }
    }

    async fn ack(&self, channel: &Channel, deliver: &Deliver, message_id: &String) {
        if let Err(err) = ack(channel, deliver, None).await {
            error!(
                "Failed to ack message_id={}: Error: {}. Shutting down...",
                message_id, err
            );
            std::process::exit(1);
        }
    }

    async fn nack(&self, channel: &Channel, deliver: &Deliver, requeue: bool, message_id: &String) {
        if let Err(err) = nack(channel, deliver, Some(requeue), None).await {
            error!(
                "Failed to nack message_id={}: Error: {}. Shutting down...",
                message_id, err
            );
            std::process::exit(1);
        }
    }
}

#[async_trait]
impl AsyncConsumer for ModelDeploymentController {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let default_message_id = String::from("unknown");

        let message_id = properties.message_id().unwrap_or(&default_message_id);

        debug!(
            "Received model deployment event message; message_id='{}'; delivery_tag={}",
            message_id,
            deliver.delivery_tag()
        );

        let event = match deserialize_event_message(content) {
            Ok(event) => event,
            Err(err) => {
                error!(
                    "Failed to deserialize message: Message id={}; Error: {}",
                    message_id, err
                );
                self.nack(channel, &deliver, false, message_id).await;

                return;
            }
        };

        let (deployment_id, deployment_revision, reconciliation_requirement) = match &event {
            Event::ModelDeploymentStarted { payload, .. } => (
                payload.deployment_id,
                payload.deployment_revision,
                ReconciliationRequirement::Start,
            ),
            Event::ModelDeploymentStopped { payload, .. } => (
                payload.deployment_id,
                payload.deployment_revision,
                ReconciliationRequirement::Stop,
            ),
            Event::ModelDeploymentDeleted { payload, .. } => (
                payload.deployment_id,
                payload.deployment_revision,
                ReconciliationRequirement::Undeploy,
            ),
        };

        debug!(
            "Decoded model deployment event; message_id='{}'; event_id='{}'; event_kind='{:?}'; deployment_id='{}'; deployment_revision={}; reconciliation_requirement='{:?}'",
            message_id,
            event.metadata().id(),
            event.metadata().kind(),
            deployment_id,
            deployment_revision,
            reconciliation_requirement
        );

        if let Err(err) = self.register_for_observation(deployment_id).await {
            error!(
                "Failed to register model deployment for observation: {}",
                err
            );

            self.nack(channel, &deliver, true, message_id).await;

            return;
        }

        let context = RequestContext::system(
            Some(*event.metadata().id()),
            Some(*event.metadata().correlation_id()),
            Some(*event.metadata().id()),
        );

        let find_input = shared::application::inputs::deployment::FindForReconciliationInput {
            deployment_id,
            revision: deployment_revision,
        };

        let mut deployment = match self
            .model_deployment_service
            .find_for_reconciliation(find_input)
            .await
        {
            Ok(deployment) => deployment,
            Err(
                ModelDeploymentServiceError::DeploymentNotFound(_)
                | ModelDeploymentServiceError::RevisionMismatch(_, _),
            ) => {
                warn!(
                    "Discarding stale model deployment event; deployment_id='{}'; revision={}",
                    deployment_id, deployment_revision
                );

                self.ack(channel, &deliver, message_id).await;

                return;
            }
            Err(err) => {
                error!(
                    "Failed to accept model deployment event; deployment_id='{}'; revision={}; error={}",
                    deployment_id, deployment_revision, err
                );

                self.nack(channel, &deliver, true, message_id).await;

                return;
            }
        };

        match deployment.reconciliation_status() {
            None => {
                deployment.accept_for_reconciliation();

                if let Err(err) = self
                    .model_deployment_service
                    .update_at_revision(
                        UpdateModelDeploymentInput {
                            deployment: deployment.clone(),
                        },
                        deployment_revision,
                    )
                    .await
                {
                    error!(
                        "Failed to persist pending reconciliation state; deployment_id='{}'; revision={}; error={}",
                        deployment_id, deployment_revision, err
                    );

                    self.nack(channel, &deliver, true, message_id).await;

                    return;
                }
            }
            Some(ReconciliationStatus::Pending) => {}
            Some(status) => {
                debug!(
                    "Acknowledging duplicate deployment event already accepted by the control plane; deployment_id='{}'; revision={}; reconciliation_status='{:?}'",
                    deployment_id, deployment_revision, status
                );

                self.ack(channel, &deliver, message_id).await;

                return;
            }
        }

        let job = ReconciliationJob {
            deployment_id,
            deployment_revision,
            reconciliation_requirement,
            retries: deployment.retries(),
            retry_after: deployment.retry_after().cloned(),
            context,
        };

        if let Err(err) = self.queue_for_reconciliation(job).await {
            error!(
                "Failed to queue model deployment for reconciliation: {}",
                err
            );

            self.nack(channel, &deliver, true, message_id).await;

            return;
        }

        debug!(
            "Acknowledging model deployment event; message_id='{}'; deployment_id='{}'",
            message_id, deployment_id
        );

        self.ack(channel, &deliver, message_id).await;
    }
}

#[derive(Debug, Clone)]
pub struct ReconciliationJob {
    deployment_id: ModelDeploymentId,
    deployment_revision: u32,
    reconciliation_requirement: ReconciliationRequirement,
    retries: u32,
    retry_after: Option<TimeStamp>,
    context: RequestContext,
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    fn job(retry_after: Option<TimeStamp>) -> ReconciliationJob {
        ReconciliationJob {
            deployment_id: ModelDeploymentId::new(),
            deployment_revision: 1,
            reconciliation_requirement: ReconciliationRequirement::Start,
            retries: u32::from(retry_after.is_some()),
            retry_after,
            context: RequestContext::system(None, None, None),
        }
    }

    #[tokio::test]
    async fn delayed_job_rotates_behind_ready_job() -> Result<(), Box<dyn Error>> {
        let queue = ReconciliationQueue::new();
        let delayed_until =
            TimeStamp::from(TimeStamp::now().into_inner() + chrono::Duration::minutes(5));
        let delayed = job(Some(delayed_until));

        let ready = job(None);

        let ready_id = ready.deployment_id;

        queue.enqueue(delayed).await?;
        queue.enqueue(ready).await?;

        let dequeued = queue.dequeue().await?;

        assert_eq!(dequeued.deployment_id, ready_id);

        Ok(())
    }

    #[tokio::test]
    async fn delayed_queue_waits_until_retry_deadline() -> Result<(), Box<dyn Error>> {
        let queue = Arc::new(ReconciliationQueue::new());
        let delayed_until =
            TimeStamp::from(TimeStamp::now().into_inner() + chrono::Duration::milliseconds(20));
        let delayed = job(Some(delayed_until));

        let delayed_id = delayed.deployment_id;

        queue.enqueue(delayed).await?;

        let dequeue_queue = queue.clone();
        let task = tokio::spawn(async move { dequeue_queue.dequeue().await });

        let dequeued = task.await??;

        assert_eq!(dequeued.deployment_id, delayed_id);

        Ok(())
    }
}
