use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage,
    ModelDeploymentStateDriftDetectedMessage,
    ModelDeploymentDeletedMessage,
    ModelDeploymentStartedMessage,
    ModelDeploymentStoppedMessage,
};

use crate::application::ports::events::{
    EventPublisherError,
    Event,
};
use crate::application::ports::commands::{
    CommandPublisherError,
    Command,
};

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

pub fn serialize_event_payload(event: &Event) -> Result<String, EventPublisherError> {
    match event {
        Event::ModelDeploymentStateDriftDetected(payload) => {
            match serde_json::to_string(&ModelDeploymentStateDriftDetectedMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Event::ModelDeploymentDeleted(payload) => {
            match serde_json::to_string(&ModelDeploymentDeletedMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Event::ModelDeploymentStarted(payload) => {
            match serde_json::to_string(&ModelDeploymentStartedMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Event::ModelDeploymentStopped(payload) => {
            match serde_json::to_string(&ModelDeploymentStoppedMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        }
    }
}