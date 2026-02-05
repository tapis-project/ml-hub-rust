use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage,
    EventMessageEnvelope,
};

use crate::application::ports::events::{
    Event,
    EventPublisherError,
};
use crate::application::ports::commands::{
    CommandPublisherError,
    Command,
};
use serde_json::to_string;

pub fn serialize_command_payload(command: &Command) -> Result<String, CommandPublisherError> {
    match command {
        Command::IngestArtifactCommand(payload) => {
            match serde_json::to_string(&IngestArtifactMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(CommandPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Command::PublishArtifactCommand(payload) => {
            match serde_json::to_string(&PublishArtifactMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(CommandPublisherError::SerializationError(err.to_string()));
                }
            };
        },
    }
}

pub fn serialize_event(event: &Event) -> Result<String, EventPublisherError> {
    let event_message_envelope = match event {
        Event::ModelDeploymentStateDriftDetected(e) => EventMessageEnvelope::try_from(e)?,
        Event::ModelDeploymentStarted(e) => EventMessageEnvelope::try_from(e)?,
        Event::ModelDeploymentStopped(e) => EventMessageEnvelope::try_from(e)?,
        Event::ModelDeploymentDeleted(e) => EventMessageEnvelope::try_from(e)?,
    };

    match to_string(&event_message_envelope) {
        Ok(s) => Ok(s),
        Err(err) => Err(EventPublisherError::SerializationError(err.to_string()))
    }
}