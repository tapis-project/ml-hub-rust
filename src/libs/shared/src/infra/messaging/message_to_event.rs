use uuid::Uuid;

use crate::application::ports::events::{self, EventMetadata};
use crate::infra::messaging::messages;
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::deployment::{State, DesiredState};
use crate::application::ports::events::EventPublisherError;

impl TryFrom<messages::EventEnvelope> for events::Event {
    type Error = EventPublisherError;

    fn try_from(value: messages::EventEnvelope) -> Result<events::Event, Self::Error> {
        let kind = events::Kind::try_from(value.kind.as_str())
            .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?;

        let metadata = events::EventMetadata::try_from((&kind, value.event.metadata))?;
        let payload = value.event.payload;

        Ok(match kind {
            events::Kind::ModelDeploymentDeleted => {
                events::Event::ModelDeploymentDeleted {
                    metadata,
                    payload: events::payloads::ModelDeploymentDeletedPayload::try_from(
                        &messages::ModelDeploymentDeletedPayload::try_from(payload)?
                    )?
                }
            },
            events::Kind::ModelDeploymentStarted => {
                events::Event::ModelDeploymentStarted {
                    metadata,
                    payload: events::payloads::ModelDeploymentStartedPayload::try_from(
                        &messages::ModelDeploymentStartedPayload::try_from(payload)?
                    )?
                }
            },
            events::Kind::ModelDeploymentStateDriftDetected => {
                events::Event::ModelDeploymentStateDriftDetected {
                    metadata,
                    payload: events::payloads::ModelDeploymentStateDriftDetectedPayload::try_from(
                        &messages::ModelDeploymentStateDriftDetectedPayload::try_from(payload)?
                    )?
                }
            },
            events::Kind::ModelDeploymentStopped => {
                events::Event::ModelDeploymentStopped {
                    metadata,
                    payload: events::payloads::ModelDeploymentStoppedPayload::try_from(
                        &messages::ModelDeploymentStoppedPayload::try_from(payload)?
                    )?
                }
            },
        })
    }
}

impl TryFrom<(&events::Kind, messages::EventMetadata)> for events::EventMetadata {
    type Error = EventPublisherError;

    fn try_from(value: (&events::Kind, messages::EventMetadata)) -> Result<Self, Self::Error> {
        Ok(EventMetadata::rehydrate(
            value.1.id,
            value.0.clone(),
            value.1.correlation_id,
            value.1.causation_id,
            TimeStamp::parse_string(value.1.timestamp.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
        ))
    }
}

impl TryFrom<&messages::ModelDeploymentStateDriftDetectedPayload> for events::payloads::ModelDeploymentStateDriftDetectedPayload {
    type Error = EventPublisherError;

    fn try_from(value: &messages::ModelDeploymentStateDriftDetectedPayload) -> Result<Self, Self::Error> {
        Ok(Self {
            deployment_id: Uuid::parse_str(value.deployment_id.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
            actual_state: State::try_from(value.actual_state.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
            desired_state: DesiredState::try_from(value.desired_state.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        })
    }
}

impl TryFrom<&messages::ModelDeploymentDeletedPayload> for events::payloads::ModelDeploymentDeletedPayload {
    type Error = EventPublisherError;

    fn try_from(value: &messages::ModelDeploymentDeletedPayload) -> Result<Self, Self::Error> {
        Ok(Self {
            deployment_id: Uuid::parse_str(value.deployment_id.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        })
    }
}

impl TryFrom<&messages::ModelDeploymentStartedPayload> for events::payloads::ModelDeploymentStartedPayload {
    type Error = EventPublisherError;

    fn try_from(value: &messages::ModelDeploymentStartedPayload) -> Result<Self, Self::Error> {
        Ok(Self {
            deployment_id: Uuid::parse_str(value.deployment_id.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        })
    }
}

impl TryFrom<&messages::ModelDeploymentStoppedPayload> for events::payloads::ModelDeploymentStoppedPayload {
    type Error = EventPublisherError;

    fn try_from(value: &messages::ModelDeploymentStoppedPayload) -> Result<Self, Self::Error> {
        Ok(Self {
            deployment_id: Uuid::parse_str(value.deployment_id.as_str())
                .map_err(|err| EventPublisherError::DeserializationError(err.to_string()))?,
            deployment_revision: value.deployment_revision,
            message: value.message.clone(),
        })
    }
}