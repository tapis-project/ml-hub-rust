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
};
use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage
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
    pub fn new() -> Self {
        Self {
            host: std::env::var("ARTIFACT_OP_MQ_HOST").expect("ARTIFACT_OP_MQ_URL missing from environment variables"),
            port: std::env::var("ARTIFACT_OP_MQ_PORT").expect("ARTIFACT_OP_MQ_PORT missing from environment variables"),
            username: std::env::var("ARTIFACT_OP_MQ_USER").expect("ARTIFACT_OP_MQ_USER missing from environment variables"),
            password: std::env::var("ARTIFACT_OP_MQ_PASSWORD").expect("ARTIFACT_OP_MQ_PASSWORD missing from environment variables"),
        }
    }
}
fn get_serialized_event_payload(event: &Event) -> Result<String, EventPublisherError> {
    match event {
        Event::IngestArtifactEvent(payload) => {
            match serde_json::to_string(&IngestArtifactMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Event::PublishArtifactEvent(payload) => {
            match serde_json::to_string(&PublishArtifactMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        _ => Err(EventPublisherError::SerializationError("Invalid Event for Artifact Op Message Publisher".into())),
    }
}


#[async_trait]
impl EventPublisher for RabbitMQModelDeploymentMessagePublisher {
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