use thiserror::Error;
use crate::constants::{
    ARTIFACT_INGESTION_EXCHANGE,
    ARTIFACT_INGESTION_ROUTING_KEY,
    ARTIFACT_PUBLICATION_EXCHANGE,
    ARTIFACT_PUBLICATION_ROUTING_KEY
};
use crate::application::ports::events::{
    EventPublisherError,
    EventPublisher,
    Event
};
use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage
};
use amqprs::{
    channel::{
        Channel,
        BasicPublishArguments,
        ExchangeDeclareArguments
    },
    connection::{
        Connection, 
        OpenConnectionArguments
    },
    BasicProperties
};
use async_trait::async_trait;
use crate::logging::GlobalLogger;

#[derive(Debug, Error)]
pub enum ArtifactOpMessagePublisherError {
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Message queue error: {0}")]
    AmqpError(#[from] amqprs::error::Error)
}

pub struct RabbitMQArtifactOpMessagePublisher;

impl RabbitMQArtifactOpMessagePublisher {
    async fn connect(&self) -> Result<Channel, EventPublisherError> {
        let host = std::env::var("ARTIFACT_OP_MQ_HOST").expect("ARTIFACT_OP_MQ_URL missing from environment variables");
        let port = std::env::var("ARTIFACT_OP_MQ_PORT").expect("ARTIFACT_OP_MQ_PORT missing from environment variables");
        let username = std::env::var("ARTIFACT_OP_MQ_USER").expect("ARTIFACT_OP_MQ_USER missing from environment variables");
        let password = std::env::var("ARTIFACT_OP_MQ_PASSWORD").expect("ARTIFACT_OP_MQ_PASSWORD missing from environment variables");

        let connection_args = OpenConnectionArguments::new(
            host.as_str(),
            port.parse::<u16>().unwrap_or(5672),
            username.as_str(),
            password.as_str()
        );

        let conn = match Connection::open(&connection_args).await {
            Ok(conn) => conn,
            Err(err) => return Err(EventPublisherError::AmqpError(err.to_string()))
        };

        let channel = conn.open_channel(None).await.expect("Open channel failed");

        let exchange_args = ExchangeDeclareArguments::new(ARTIFACT_INGESTION_EXCHANGE, "topic");
        channel.exchange_declare(exchange_args).await
            .map_err(|err| EventPublisherError::ConnectionError(err.to_string()))?;

        let exchange_args = ExchangeDeclareArguments::new(ARTIFACT_INGESTION_EXCHANGE, "topic");
        channel.exchange_declare(exchange_args).await
            .map_err(|err| EventPublisherError::ConnectionError(err.to_string()))?;

        Ok(channel)
    }    
}

fn get_message_from_event(event: &Event) -> Result<String, EventPublisherError> {
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
        }
    }
}

fn get_exchange(event: &Event) -> &'static str {
    match event {
        Event::IngestArtifactEvent(_) => ARTIFACT_INGESTION_EXCHANGE,
        Event::PublishArtifactEvent(_) => ARTIFACT_PUBLICATION_EXCHANGE
    }
}

fn get_routing_key(event: &Event) -> &'static str {
    match event {
        Event::IngestArtifactEvent(_) => ARTIFACT_INGESTION_ROUTING_KEY,
        Event::PublishArtifactEvent(_) => ARTIFACT_PUBLICATION_ROUTING_KEY
    }
}

#[async_trait]
impl EventPublisher for RabbitMQArtifactOpMessagePublisher {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError> {    
        let serialized_message = get_message_from_event(&event)?;
        let payload = match serde_json::to_string(&serialized_message) {
            Ok(p) => p,
            Err(err) => {
                return Err(EventPublisherError::SerializationError(err.to_string()));
            }
        };

        // Publish to exchange
        let args = BasicPublishArguments::new(
            get_exchange(event),
            get_routing_key(event),
        ).mandatory(true)
            .finish();

        let connection = self.connect().await.unwrap();

        connection.basic_publish(BasicProperties::default(), payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| {
                GlobalLogger::error(format!("Failed basic publish: {:#?}", err).as_str());
                EventPublisherError::AmqpError(err.to_string())
            })?;
       
        Ok(())
    }
}