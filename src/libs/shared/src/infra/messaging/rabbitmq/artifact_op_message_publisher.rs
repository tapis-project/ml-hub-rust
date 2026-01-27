use thiserror::Error;
use crate::application::ports::events::{
    EventPublisherError,
    EventPublisher,
    Event
};
use crate::infra::messaging::rabbitmq::helpers::{
    get_exchange,
    get_routing_key,
    amqp_connection_builder,
    get_serialized_event_payload,
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
impl EventPublisher for RabbitMQArtifactOpMessagePublisher {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError> {    
        let payload = get_serialized_event_payload(&event)?;

        // Publish to exchange
        let args = BasicPublishArguments::new(
            get_exchange(event),
            get_routing_key(event),
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
                EventPublisherError::AmqpError(err.to_string())
            })?;
       
        Ok(())
    }
}