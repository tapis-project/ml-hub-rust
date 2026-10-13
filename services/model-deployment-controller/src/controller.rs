use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use amqprs::{channel::Channel, consumer::AsyncConsumer, BasicProperties, Deliver};
use async_trait::async_trait;
use indexmap::IndexMap;
use log::{error, warn};
use shared::application::inputs::deployment::UpdateModelDeploymentInput;
use shared::application::ports::events::Event;
use shared::application::services::model_deployment_observation_service::ModelDeploymentObservationService;
use shared::application::services::model_deployment_reconciliation_service::{
    DispatchReconcilerInput, ModelDeploymentReconciliationService, ReconciliationDispatchError,
};
use shared::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use shared::domain::entities::deployment::{ModelDeploymentId, ReconciliationRequirement};
use shared::infra::messaging::codec::deserialize_event_message;
use shared::infra::messaging::rabbitmq::settlement::{ack, nack};
use shared::shared_kernel::context::RequestContext;
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
        {
            let mut queue = self
                .jobs
                .lock()
                .map_err(|_| ControlLoopStateError::LockPoisoned("reconciliation queue"))?;

            queue.insert(job.deployment_id, job);
        }

        self.notification.notify_one();

        Ok(())
    }

    async fn dequeue(&self) -> Result<ReconciliationJob, ControlLoopStateError> {
        loop {
            let notified = self.notification.notified();

            let maybe_job = {
                let mut queue = self
                    .jobs
                    .lock()
                    .map_err(|_| ControlLoopStateError::LockPoisoned("reconciliation queue"))?;

                queue.shift_remove_index(0).map(|(_, job)| job)
            };

            if let Some(job) = maybe_job {
                return Ok(job);
            }

            notified.await;
        }
    }
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
    state: Arc<ControlLoopState>,
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
        state: Arc<ControlLoopState>,
    ) -> Result<Self, ModelDeploymentControllerInitializationError> {
        let controller = Self {
            model_deployment_service,
            reconciliation_service,
            observer_service,
            state,
        };

        controller.synchronize().await?;

        Ok(controller)
    }

    async fn synchronize(&self) -> Result<(), ModelDeploymentControllerInitializationError> {
        let deployments = self
            .model_deployment_service
            .find_all_reconcilable()
            .await?;

        for deployment in deployments {
            self.register_for_observation(*deployment.id()).await?;

            if let Some(requirement) = deployment.resolve_reconciliation_requirement() {
                self.queue_for_reconciliation(ReconciliationJob {
                    deployment_id: deployment.id().clone(),
                    deployment_revision: deployment.revision().clone(),
                    reconciliation_requirement: requirement,
                    context: RequestContext::system(None, None, None),
                })
                .await?;
            }
        }

        Ok(())
    }

    pub async fn register_for_observation(
        &self,
        deployment_id: ModelDeploymentId,
    ) -> Result<(), ControlLoopStateError> {
        self.state.observation_set.add(deployment_id).await
    }

    pub async fn queue_for_reconciliation(
        &self,
        job: ReconciliationJob,
    ) -> Result<(), ControlLoopStateError> {
        self.state.reconciliation_queue.enqueue(job).await
    }

    pub async fn run_reconciliation_loop(&self) -> Result<(), ControlLoopStateError> {
        loop {
            let job = self.state.reconciliation_queue.dequeue().await?;

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
                    Self::log_reconciliation_error(&job, err);

                    continue;
                }
            };

            let Some(deployment) = reconciliation_result.deployment else {
                continue;
            };

            let deployment_id = *deployment.id();

            let update = UpdateModelDeploymentInput { deployment };

            if let Err(err) = self.model_deployment_service.update(update).await {
                error!(
                    "Failed to persist reconciled model deployment '{}': {}",
                    deployment_id, err
                );
            }
        }
    }

    pub async fn run_observation_loop(&self) -> Result<(), ControlLoopStateError> {
        let mut interval = tokio::time::interval(Self::OBSERVATION_INTERVAL);

        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            for deployment_id in self.state.get_observation_snapshot()? {
                let context = RequestContext::system(None, None, None);

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

                let observed_deployment_id = *result.deployment.id();
                let observed_deployment_revision = *result.deployment.revision();

                let persistence_result = if result.deployment_changed {
                    self.model_deployment_service
                        .update(UpdateModelDeploymentInput {
                            deployment: result.deployment,
                        })
                        .await
                } else {
                    self.model_deployment_service
                        .record_observation(&context, &observed_deployment_id, &result.observed_at)
                        .await
                };

                if let Err(err) = persistence_result {
                    error!(
                        "Failed to persist observed model deployment '{}': {}",
                        observed_deployment_id, err
                    );

                    continue;
                }

                if let Some(requirement) = result.reconciliation_requirement {
                    self.queue_for_reconciliation(ReconciliationJob {
                        deployment_id: observed_deployment_id,
                        deployment_revision: observed_deployment_revision,
                        reconciliation_requirement: requirement,
                        context,
                    })
                    .await?;
                }
            }
        }
    }

    fn log_reconciliation_error(job: &ReconciliationJob, err: ReconciliationDispatchError) {
        match err {
            ReconciliationDispatchError::StaleEvent(err) => {
                warn!(
                    "Discarding stale reconciliation job for deployment '{}' at revision {} with requirement '{:?}': {}",
                    job.deployment_id,
                    job.deployment_revision,
                    job.reconciliation_requirement,
                    err
                );
            }
            err => {
                error!(
                    "Failed to reconcile deployment '{}' at revision {} with requirement '{:?}': {}",
                    job.deployment_id,
                    job.deployment_revision,
                    job.reconciliation_requirement,
                    err
                );
            }
        }
    }

    pub async fn run_synchronization_loop(&self) {
        let mut interval = tokio::time::interval(Self::SYNCHRONIZATION_INTERVAL);

        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        interval.tick().await;

        loop {
            interval.tick().await;

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
            _ => {
                error!(
                    "Invalid event type for this consumer: {}",
                    String::from(event.metadata().kind())
                );
                self.nack(channel, &deliver, false, message_id).await;

                return;
            }
        };

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

        let job = ReconciliationJob {
            deployment_id,
            deployment_revision,
            reconciliation_requirement,
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

        self.ack(channel, &deliver, message_id).await;
    }
}

#[derive(Debug, Clone)]
pub struct ReconciliationJob {
    deployment_id: ModelDeploymentId,
    deployment_revision: u32,
    reconciliation_requirement: ReconciliationRequirement,
    context: RequestContext,
}
