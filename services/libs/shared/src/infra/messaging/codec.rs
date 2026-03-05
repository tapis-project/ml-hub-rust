use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage,
    EventEnvelope,
};
use crate::infra::messaging::errors::SerializationError;
use crate::application::ports::events::Event;
use crate::application::ports::commands::Command;
use serde_json::{to_string, Value};

pub fn serialize_command_payload(command: &Command) -> Result<String, SerializationError> {
    match command {
        Command::IngestArtifactCommand(payload) => {
            serde_json::to_string(&IngestArtifactMessage::from(payload))
                .map_err(|err| SerializationError::SerializationFailed(err.to_string()))
        },
        Command::PublishArtifactCommand(payload) => {
            serde_json::to_string(&PublishArtifactMessage::from(payload))
                .map_err(|err| SerializationError::SerializationFailed(err.to_string()))
        },
    }
}

pub fn deserialize_event_message(content: Vec<u8>) -> Result<Event, SerializationError> {
    let message_value: Value = match serde_json::from_slice(&content) {
        Ok(e) => e,
        Err(err) => return Err(SerializationError::DeserializationFailed(err.to_string()))
    };

    let event_envelope = EventEnvelope::try_from(message_value)
        .map_err(|err| SerializationError::DeserializationFailed(err.to_string()))?;

    Ok(Event::try_from(event_envelope).map_err(|err| SerializationError::DeserializationFailed(err.to_string())))?
}

pub fn serialize_event(event: &Event) -> Result<String, SerializationError> {
    let event_envelope = EventEnvelope::try_from(event)
        .map_err(|err| SerializationError::SerializationFailed(err.to_string()))?;

    match to_string(&event_envelope) {
        Ok(s) => Ok(s),
        Err(err) => Err(SerializationError::SerializationFailed(err.to_string()))
    }
}