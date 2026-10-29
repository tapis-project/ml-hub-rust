pub mod payloads;

use crate::shared_kernel::value_objects::TimeStamp;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Kind {
    ModelDeploymentStarted,
    ModelDeploymentStopped,
    ModelDeploymentDeleted,
}

#[derive(Debug, Clone)]
pub enum Payload {
    ModelDeploymentStartedPayload(payloads::ModelDeploymentStartedPayload),
    ModelDeploymentStoppedPayload(payloads::ModelDeploymentStoppedPayload),
    ModelDeploymentDeletedPayload(payloads::ModelDeploymentDeletedPayload),
}

impl Payload {
    pub fn kind(&self) -> Kind {
        match self {
            Payload::ModelDeploymentDeletedPayload(_) => Kind::ModelDeploymentDeleted,
            Payload::ModelDeploymentStartedPayload(_) => Kind::ModelDeploymentStarted,
            Payload::ModelDeploymentStoppedPayload(_) => Kind::ModelDeploymentStopped,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    ModelDeploymentStarted {
        metadata: EventMetadata,
        payload: payloads::ModelDeploymentStartedPayload,
    },
    ModelDeploymentStopped {
        metadata: EventMetadata,
        payload: payloads::ModelDeploymentStoppedPayload,
    },
    ModelDeploymentDeleted {
        metadata: EventMetadata,
        payload: payloads::ModelDeploymentDeletedPayload,
    }
}

impl Event {
    pub fn from_payload(payload: &Payload, correlation_id: Option<&Uuid>, causation_id: Option<&Uuid>) -> Self {
        match payload {
            Payload::ModelDeploymentDeletedPayload(p) => Self::ModelDeploymentDeleted {
                metadata: EventMetadata::from_payload(payload, correlation_id, causation_id),
                payload: p.clone(),
            },
            Payload::ModelDeploymentStartedPayload(p) => Self::ModelDeploymentStarted {
                metadata: EventMetadata::from_payload(payload, correlation_id, causation_id),
                payload: p.clone(),
            },
            Payload::ModelDeploymentStoppedPayload(p) => Self::ModelDeploymentStopped {
                metadata: EventMetadata::from_payload(payload, correlation_id, causation_id),
                payload: p.clone(),
            },
        }
    }

    pub fn payload(&self) -> Payload {
        match self {
            Event::ModelDeploymentDeleted { payload, .. } => {
                Payload::ModelDeploymentDeletedPayload(payload.clone())
            },
            Event::ModelDeploymentStarted { payload, .. } => {
                Payload::ModelDeploymentStartedPayload(payload.clone())
            },
            Event::ModelDeploymentStopped { payload, .. } => {
                Payload::ModelDeploymentStoppedPayload(payload.clone())
            },
        }
    }

    pub fn metadata(&self) -> &EventMetadata {
        match self {
            Event::ModelDeploymentDeleted { metadata, .. } => metadata,
            Event::ModelDeploymentStarted { metadata, .. } => metadata,
            Event::ModelDeploymentStopped { metadata, .. } => metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventMetadata {
    id: Uuid,
    kind: Kind,
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
    timestamp: TimeStamp,
}

impl EventMetadata {
    pub(crate) fn rehydrate(
        id: Uuid,
        kind: Kind,
        correlation_id: Option<Uuid>,
        causation_id: Option<Uuid>,
        timestamp: TimeStamp,
    ) -> Self {
        Self {
            id,
            kind,
            correlation_id,
            causation_id,
            timestamp,
        }
    }

    pub fn from_payload(payload: &Payload, correlation_id: Option<&Uuid>, causation_id: Option<&Uuid>) -> Self {
        let id = Uuid::now_v7();

        Self {
            id: id,
            kind: payload.kind(),
            correlation_id: correlation_id.map(|id| *id),
            causation_id: causation_id.map(|id| *id),
            timestamp: TimeStamp::now(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn correlation_id(&self) -> &Uuid {
        if let Some(ref correlation_id) = self.correlation_id {
            return correlation_id
        }

        self.id()
    }

    pub fn causation_id(&self) -> &Uuid {
        if let Some(ref causation_id) = self.causation_id {
            return causation_id
        }

        self.correlation_id()
    }

    pub fn timestamp(&self) -> &TimeStamp {
        &self.timestamp
    }
}

#[derive(Clone, Debug, Error)]
pub enum EventPublisherError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Routing error: {0}")]
    Routing(String),

    #[error("Publishing failed: {0}")]
    Publishing(String),

    #[error("Connection: {0}")]
    Connection(String),
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError>;
}
