use crate::application::ports::events::payloads::{
    ModelDeploymentStateDriftDetectedPayload,
    ModelDeploymentDeletedPayload,
    ModelDeploymentStartedPayload,
    ModelDeploymentStoppedPayload,
};
use crate::application::ports::events::{
    EventMetadata,
    Payload,
    Event,
    EventPublisherError,
};
use crate::infra::messaging::messages;
use serde_json::{to_value, Value};

impl TryFrom<&Event> for messages::EventEnvelope {
    type Error = EventPublisherError;

    fn try_from(value: &Event) -> Result<Self, Self::Error> {
        let kind = match value {
            Event::ModelDeploymentDeleted { payload, .. } => &Payload::ModelDeploymentDeletedPayload(payload.clone()).kind(),
            Event::ModelDeploymentStarted { payload, .. } => &Payload::ModelDeploymentStartedPayload(payload.clone()).kind(),
            Event::ModelDeploymentStopped { payload, .. } => &Payload::ModelDeploymentStoppedPayload(payload.clone()).kind(),
            Event::ModelDeploymentStateDriftDetected { payload, .. } => &Payload::ModelDeploymentStateDriftDetectedPayload(payload.clone()).kind(),
        };

        Ok(messages::EventEnvelope {
            kind: String::from(kind),
            event: messages::Event {
                payload: Value::try_from(&value.payload())?,
                metadata: messages::EventMetadata::from(value.metadata()),
            }
        })
    }
}

impl TryFrom<&Payload> for Value {
    type Error = EventPublisherError;

    fn try_from(value: &Payload) -> Result<Self, Self::Error> {
        let payload = match value {
            Payload::ModelDeploymentDeletedPayload(p) => to_value(messages::ModelDeploymentDeletedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
            Payload::ModelDeploymentStartedPayload(p) => to_value(messages::ModelDeploymentStartedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
            Payload::ModelDeploymentStateDriftDetectedPayload(p) => to_value(messages::ModelDeploymentStateDriftDetectedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
            Payload::ModelDeploymentStoppedPayload(p) => to_value(messages::ModelDeploymentStoppedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
        };

        Ok(payload)
    }
}

impl From<&EventMetadata> for messages::EventMetadata {
    fn from(value: &EventMetadata) -> Self {
        Self {
            id: value.id().clone(),
            correlation_id: value.correlation_id().clone(),
            causation_id: value.causation_id().clone(),
            timestamp: String::from(value.timestamp().clone()),
        }
    }
}

impl From<&ModelDeploymentStateDriftDetectedPayload> for messages::ModelDeploymentStateDriftDetectedPayload {
    fn from(value: &ModelDeploymentStateDriftDetectedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            actual_state: String::from(value.actual_state.clone()),
            desired_state: String::from(value.desired_state.clone()),
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        }
    }
}

impl From<&ModelDeploymentDeletedPayload> for messages::ModelDeploymentDeletedPayload {
    fn from(value: &ModelDeploymentDeletedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        }
    }
}

impl From<&ModelDeploymentStartedPayload> for messages::ModelDeploymentStartedPayload {
    fn from(value: &ModelDeploymentStartedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        }
    }
}

impl From<&ModelDeploymentStoppedPayload> for messages::ModelDeploymentStoppedPayload {
    fn from(value: &ModelDeploymentStoppedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        }
    }
}