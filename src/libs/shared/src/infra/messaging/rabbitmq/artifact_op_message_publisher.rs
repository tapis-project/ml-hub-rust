use std::sync::Arc;
use amqprs::channel::Channel;
use thiserror::Error;
use crate::application::ports::commands::{
    CommandPublisherError,
    CommandPublisher,
    Command
};
use crate::infra::messaging::rabbitmq::exchanges::get_exchange_for_command;
use crate::infra::messaging::rabbitmq::routing::get_routing_key_for_command;
use crate::infra::messaging::codec::serialize_command_payload;
use amqprs::{
    channel::BasicPublishArguments,
    BasicProperties
};
use async_trait::async_trait;
use log::{error, debug};

#[derive(Debug, Error)]
pub enum ArtifactOpMessagePublisherError {
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Message queue error: {0}")]
    AmqpError(#[from] amqprs::error::Error)
}

pub struct RabbitMQArtifactOpMessagePublisher {
    channel: Arc<Channel>
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
        ).mandatory(true)
            .finish();

        debug!("Exchange to publish to: {}", get_exchange_for_command(command));
        debug!("Routing key to use:  {}", get_routing_key_for_command(command));

        self.channel.basic_publish(BasicProperties::default(), payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| CommandPublisherError::Publishing(err.to_string()))?;
       
        Ok(())
    }
}