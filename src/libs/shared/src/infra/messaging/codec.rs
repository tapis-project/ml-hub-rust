use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage,
    EventEnvelope,
};
use crate::infra::messaging::errors::JsonError;
use crate::application::ports::events::{
    Event,
    EventPublisherError,
    EventMetadata,
    Kind,
};
use crate::application::ports::commands::{
    CommandPublisherError,
    Command,
};
use serde_json::{to_string, Value};

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

pub fn deserialize_event_message(content: Vec<u8>) -> Result<Event, EventPublisherError> {
    let message_value: Value = match serde_json::from_slice(&content) {
        Ok(e) => e,
        Err(err) => return Err(EventPublisherError::DeserializationError(err.to_string()))
    };

    let event_envelope = EventEnvelope::try_from(message_value)?;

    Ok(Event::try_from(event_envelope))?
}

pub fn serialize_event(event: &Event) -> Result<String, EventPublisherError> {
    let event_envelope = EventEnvelope::try_from(event)?;

    match to_string(&event_envelope) {
        Ok(s) => Ok(s),
        Err(err) => Err(EventPublisherError::SerializationError(err.to_string()))
    }
}