use uuid::Uuid;
use crate::domain::entities::deployment::{DesiredState, State};
use crate::domain::entities::timestamp::TimeStamp;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventPublisherError {
    #[error("Event broker error: {0}")]
    AmqpError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Connection: {0}")]
    ConnectionError(String),
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStateDriftDetectedPayload {
    pub deployment_revision: u32,
    pub deployment_id: Uuid,
    pub desired_state: DesiredState,
    pub acutal_state: State,
    pub timestamp: TimeStamp,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStartedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub timestamp: TimeStamp,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStoppedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub timestamp: TimeStamp,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentDeletedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub timestamp: TimeStamp,
}

pub enum Event {
    ModelDeploymentStateDriftDetected(ModelDeploymentStateDriftDetectedPayload),
    ModelDeploymentStarted(ModelDeploymentStartedPayload),
    ModelDeploymentStopped(ModelDeploymentStoppedPayload),
    ModelDeploymentDeleted(ModelDeploymentDeletedPayload),
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError>;
}