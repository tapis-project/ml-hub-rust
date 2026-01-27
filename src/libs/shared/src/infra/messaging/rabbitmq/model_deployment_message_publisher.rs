use thiserror::Error;
use crate::application::ports::commands::{
    CommandPublisherError,
    CommandPublisher,
    Command
};
use crate::infra::messaging::rabbitmq::helpers::{
    get_exchange,
    get_routing_key,
    amqp_connection_builder,
    get_serialized_command_payload,
};
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

pub struct RabbitMQModelDeploymentMessagePublisher {
    host: String,
    port: String,
    username: String,
    password: String,
}

impl RabbitMQModelDeploymentMessagePublisher {
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
impl CommandPublisher for RabbitMQModelDeploymentMessagePublisher {
    async fn publish(&self, command: &Command) -> Result<(), CommandPublisherError> {    
        let payload = get_serialized_command_payload(&command)?;

        // Publish to exchange
        let args = BasicPublishArguments::new(
            get_exchange(command),
            get_routing_key(command),
        ).mandatory(true)
            .finish();

        let connection = amqp_connection_builder(
            &self.host,
            &self.port,
            &self.username,
            &self.password,
        ).await.unwrap();

        connection.basic_publish(BasicProperties::default(), payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| {
                error!("Failed basic publish: {:#?}", err);
                CommandPublisherError::AmqpError(err.to_string())
            })?;
       
        Ok(())
    }
}