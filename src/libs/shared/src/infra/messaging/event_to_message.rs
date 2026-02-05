use crate::application::ports::events::{
    ModelDeploymentStateDriftDetectedPayload,
    ModelDeploymentDeletedPayload,
    ModelDeploymentStartedPayload,
    ModelDeploymentStoppedPayload,
    EventMetadata,
    EventPayload,
    EventEnvelope,
    EventPublisherError,
    Kind,
};
use crate::infra::messaging::messages;
use serde_json::{to_value, Value};

impl TryFrom<&EventEnvelope> for messages::EventMessageEnvelope {
    type Error = EventPublisherError;

    fn try_from(value: &EventEnvelope) -> Result<Self, Self::Error> {
        Ok(messages::EventMessageEnvelope {
            kind: String::from(Kind::from(value.payload())),
            event_envelope: messages::EventEnvelope {
                payload: Value::try_from(value.payload())?,
                metadata: messages::EventMetadata::from(&value.metadata().clone())
            },
        })
    }
}

impl TryFrom<&EventPayload> for Value {
    type Error = EventPublisherError;

    fn try_from(value: &EventPayload) -> Result<Self, Self::Error> {
        let payload = match value {
            EventPayload::ModelDeploymentDeletedPayload(p) => to_value(messages::ModelDeploymentDeletedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
            EventPayload::ModelDeploymentStartedPayload(p) => to_value(messages::ModelDeploymentStartedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
            EventPayload::ModelDeploymentStateDriftDetectedPayload(p) => to_value(messages::ModelDeploymentStateDriftDetectedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
            EventPayload::ModelDeploymentStoppedPayload(p) => to_value(messages::ModelDeploymentStoppedPayload::from(p))
                .map_err(|err| Self::Error::SerializationError(err.to_string()))?,
        };

        Ok(payload)
    }
}

impl From<&EventMetadata> for messages::EventMetadata {
    fn from(value: &EventMetadata) -> Self {
        Self {
            id: value.id.clone(),
            kind: String::from(value.kind.clone()),
            correlation_id: value.correlation_id.clone(),
            causation_id: value.causation_id.clone(),
            timestamp: String::from(value.timestamp.clone()),
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