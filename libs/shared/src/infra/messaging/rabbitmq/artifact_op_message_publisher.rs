use crate::application::ports::commands::{Command, CommandPublisher, CommandPublisherError};
use crate::infra::messaging::codec::serialize_command_payload;
use crate::infra::messaging::rabbitmq::exchanges::get_exchange_for_command;
use crate::infra::messaging::rabbitmq::routing::get_routing_key_for_command;
use amqprs::channel::Channel;
use amqprs::{channel::BasicPublishArguments, BasicProperties};
use async_trait::async_trait;
use log::error;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactOpMessagePublisherError {
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Message queue error: {0}")]
    AmqpError(#[from] amqprs::error::Error),
}

pub struct RabbitMQArtifactOpMessagePublisher {
    channel: Arc<Channel>,
}

impl RabbitMQArtifactOpMessagePublisher {
    pub fn new(channel: Arc<Channel>) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl CommandPublisher for RabbitMQArtifactOpMessagePublisher {
    async fn publish(&self, command: &Command) -> Result<(), CommandPublisherError> {
        let payload = serialize_command_payload(&command)
            .map_err(|err| CommandPublisherError::Serialization(err.to_string()))?;

        let args = BasicPublishArguments::new(
            get_exchange_for_command(command),
            get_routing_key_for_command(command),
        )
        .mandatory(true)
        .finish();

        self.channel
            .basic_publish(
                BasicProperties::default(),
                payload.as_bytes().to_vec(),
                args,
            )
            .await
            .map_err(|err| CommandPublisherError::Publishing(err.to_string()))?;

        Ok(())
    }
}
