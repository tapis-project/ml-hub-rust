use thiserror::Error;
use crate::constants::{
    ARTIFACT_INGESTION_EXCHANGE,
    ARTIFACT_INGESTION_ROUTING_KEY
};
use crate::application::ports::events::{
    EventPublisherError,
    EventPublisher,
    Event
};
use crate::infra::messaging::messages::IngestArtifactMessage;
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

        Ok(channel)
    }    
}

#[async_trait]
impl EventPublisher for RabbitMQArtifactOpMessagePublisher {
    async fn publish(&self, event: Event) -> Result<(), EventPublisherError> {
        let msg: IngestArtifactMessage = match event {
            Event::IngestArtifactEvent(payload) => {
                IngestArtifactMessage::from(payload)
            },
            _other => { return Err(EventPublisherError::InternalError("unsupported message type".into())) }
        };
        
        let payload = match serde_json::to_string(&msg) {
            Ok(p) => p,
            Err(err) => {
                return Err(EventPublisherError::SerializationError(err.to_string()));
            }
        };

        // Publish to exchange
        let args = BasicPublishArguments::new(
            ARTIFACT_INGESTION_EXCHANGE,
            ARTIFACT_INGESTION_ROUTING_KEY,
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