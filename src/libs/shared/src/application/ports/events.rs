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
    pub actual_state: State,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStartedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStoppedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentDeletedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum EventPayload {
    ModelDeploymentStateDriftDetectedPayload(ModelDeploymentStateDriftDetectedPayload),
    ModelDeploymentStartedPayload(ModelDeploymentStartedPayload),
    ModelDeploymentStoppedPayload(ModelDeploymentStoppedPayload),
    ModelDeploymentDeletedPayload(ModelDeploymentDeletedPayload),
}

#[derive(Debug, Clone)]
pub enum Kind {
    ModelDeploymentStateDriftDetected,
    ModelDeploymentStarted,
    ModelDeploymentStopped,
    ModelDeploymentDeleted,
    Unknown,
}

impl From<&EventPayload> for Kind {
    fn from(value: &EventPayload) -> Self {
        match value {
            EventPayload::ModelDeploymentDeletedPayload(_) => Kind::ModelDeploymentDeleted,
            EventPayload::ModelDeploymentStartedPayload(_) => Kind::ModelDeploymentStarted,
            EventPayload::ModelDeploymentStoppedPayload(_) => Kind::ModelDeploymentStopped,
            EventPayload::ModelDeploymentStateDriftDetectedPayload(_) => Kind::ModelDeploymentStateDriftDetected,
        }
    }
}

impl From<Kind> for String {
    fn from(value: Kind) -> Self {
        match value {
            Kind::ModelDeploymentStateDriftDetected => String::from("model_deployment.state_drift_detected"),
            Kind::ModelDeploymentStarted => String::from("model_deployment.started"),
            Kind::ModelDeploymentStopped => String::from("model_deployment.stopped"),
            Kind::ModelDeploymentDeleted => String::from("model_deployment.deleted"),
            _ => String::from("unknown"),
        }
    }
}

impl From<String> for Kind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "model_deployment.state_drift_detected" => Kind::ModelDeploymentStateDriftDetected,
            "model_deployment.started" => Kind::ModelDeploymentStarted,
            "model_deployment.stopped" => Kind::ModelDeploymentStopped,
            "model_deployment.deleted" => Kind::ModelDeploymentDeleted,
            _ => Kind::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub id: Uuid,
    pub kind: Kind,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub timestamp: TimeStamp,
}

impl EventMetadata {
    pub fn from_payload(payload: &EventPayload, caused_by: Option<Event>) -> Self {
        let id = Uuid::now_v7();
        let mut correlation_id = id.clone();
        let mut causation_id = id.clone();

        if let Some(event) = caused_by {
            correlation_id = event.metadata().correlation_id.clone();
            causation_id = event.metadata().id.clone();
        }

        Self {
            id: id.clone(),
            kind: Kind::from(payload),
            correlation_id: correlation_id,
            causation_id: causation_id,
            timestamp: TimeStamp::now(),
        }
    }
}

pub struct EventEnvelope {
    payload: EventPayload,
    metadata: EventMetadata,
}

impl EventEnvelope {
    pub fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }
}

pub enum Event {
    ModelDeploymentStateDriftDetected(EventEnvelope),
    ModelDeploymentStarted(EventEnvelope),
    ModelDeploymentStopped(EventEnvelope),
    ModelDeploymentDeleted(EventEnvelope),
}

impl Event {
    pub fn metadata(&self) -> &EventMetadata {
        match self {
            Event::ModelDeploymentDeleted(envelope) => envelope.metadata(),
            Event::ModelDeploymentStarted(envelope) => envelope.metadata(),
            Event::ModelDeploymentStopped(envelope) => envelope.metadata(),
            Event::ModelDeploymentStateDriftDetected(envelope) => envelope.metadata(),
        }
    }

    pub fn from_payload(payload: &EventPayload, caused_by: Option<Event>) -> Self {
        match payload {
            EventPayload::ModelDeploymentStateDriftDetectedPayload(p) => {
                Self::ModelDeploymentStateDriftDetected(
                    EventEnvelope {
                        payload: EventPayload::ModelDeploymentStateDriftDetectedPayload(p.clone()),
                        metadata: EventMetadata::from_payload(payload, caused_by)
                    }
                )
            },
            EventPayload::ModelDeploymentDeletedPayload(p) => {
                Self::ModelDeploymentDeleted(
                    EventEnvelope {
                        payload: EventPayload::ModelDeploymentDeletedPayload(p.clone()),
                        metadata: EventMetadata::from_payload(payload, caused_by)
                    }
                )
            },
            EventPayload::ModelDeploymentStartedPayload(p) => {
                Self::ModelDeploymentStarted(
                    EventEnvelope {
                        payload: EventPayload::ModelDeploymentStartedPayload(p.clone()),
                        metadata: EventMetadata::from_payload(payload, caused_by)
                    }
                )
            },
            EventPayload::ModelDeploymentStoppedPayload(p) => {
                Self::ModelDeploymentStopped(
                    EventEnvelope {
                        payload: EventPayload::ModelDeploymentStoppedPayload(p.clone()),
                        metadata: EventMetadata::from_payload(payload, caused_by)
                    }
                )
            },
        }
    }
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError>;
}