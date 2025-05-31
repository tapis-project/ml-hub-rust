use serde::Serialize;
use lapin::{
    Connection,
    ConnectionProperties,
    Channel,
    BasicProperties,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable
};
use thiserror::Error;
use shared::constants::ARTIFACT_STAGING_QUEUE;

pub struct ArtifactOpMessagePublisher {
    channel: Channel
}

impl ArtifactOpMessagePublisher {
    pub async fn new() -> Result<Self, ()> {
        // Connect to RabbitMQ
        let conn = Connection::connect(
            "amqp://guest:guest@localhost:5672/%2f",
            ConnectionProperties::default(),
        )
        .await
        .expect("connection failed");

        println!("Connected to RabbitMQ");

        // Create a channel
        let channel = conn.create_channel().await.expect("create_channel");

        // Declare a queue (idempotent — safe to call every time)
        let _ = channel
            .queue_declare(
                ARTIFACT_STAGING_QUEUE,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");

        Ok(Self {
            channel
        })
    }

    pub async fn publish<T: Serialize>(&self, message: T) -> Result<(), ArtifactOpMessagePublisherError> {
        let payload = match serde_json::to_string(&message) {
            Ok(p) => p,
            Err(err) => {
                return Err(ArtifactOpMessagePublisherError::SerializationError(err.to_string()));
            }
        };

        self.channel
            .basic_publish(
                "",
                "download_jobs",
                BasicPublishOptions::default(),
                &payload.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("basic_publish")
            .await
            .expect("publisher confirm");

        Ok(())
    }
}