use amqprs::connection::OpenConnectionArguments;
use thiserror::Error;
use crate::application::ports::commands::{
    CommandPublisherError,
    CommandPublisher,
    Command
};
use crate::infra::messaging::rabbitmq::exchanges::get_exchange_for_command;
use crate::infra::messaging::rabbitmq::routing::get_routing_key_for_command;
use crate::infra::messaging::rabbitmq::connection::open_channel;
use crate::infra::messaging::rabbitmq::exchanges::{delcare_exchanges, ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_PUBLICATION_EXCHANGE};
use crate::infra::messaging::codec::serialize_command_payload;
use amqprs::{
    channel::BasicPublishArguments,
    BasicProperties
};
use async_trait::async_trait;
use log::error;

#[derive(Debug, Error)]
pub enum ArtifactOpMessagePublisherError {
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Message queue error: {0}")]
    AmqpError(#[from] amqprs::error::Error)
}

pub struct RabbitMQArtifactOpMessagePublisher {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

impl RabbitMQArtifactOpMessagePublisher {
    pub fn new(host: String, port: String, username: String, password: String) -> Self {
        Self {
            host,
            port,
            username,
            password
        }
    }
}

#[async_trait]
impl CommandPublisher for RabbitMQArtifactOpMessagePublisher {
    async fn publish(&self, command: &Command) -> Result<(), CommandPublisherError> {    
        let payload = serialize_command_payload(&command)
            .map_err(|err| CommandPublisherError::Serialization(err.to_string()))?;

        // Publish to exchange
        let args = BasicPublishArguments::new(
            get_exchange_for_command(command),
            get_routing_key_for_command(command),
        ).mandatory(true)
            .finish();

        let connection_args = OpenConnectionArguments::new(
            self.host.as_str(),
            self.port.parse::<u16>().unwrap_or(5672),
            self.username.as_str(),
            self.password.as_str()
        );

        let channel = open_channel(connection_args)
            .await
            .map_err(|err| CommandPublisherError::Connection(err.to_string()))?;

        delcare_exchanges(
            &channel, 
            vec![
                (ARTIFACT_INGESTION_EXCHANGE, "topic"),
                (ARTIFACT_PUBLICATION_EXCHANGE, "topic"),
            ]
        ).await
            .map_err(|err| CommandPublisherError::Routing(err.to_string()))?;

        channel.basic_publish(BasicProperties::default(), payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| CommandPublisherError::Publishing(err.to_string()))?;
       
        Ok(())
    }
}