use std::sync::Arc;
use amqprs::channel::Channel;
use thiserror::Error;
use crate::application::ports::events::{
    EventPublisherError,
    EventPublisher,
    Event
};
use crate::infra::messaging::codec::serialize_event;
use crate::infra::messaging::rabbitmq::exchanges::get_exchange_for_event;
use crate::infra::messaging::rabbitmq::routing::get_routing_key_for_event;

use amqprs::{
    channel::BasicPublishArguments,
    BasicProperties
};
use async_trait::async_trait;
use log::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ArtifactOpMessagePublisherError {
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Message queue error: {0}")]
    AmqpError(#[from] amqprs::error::Error)
}

pub struct RabbitMQModelDeploymentMessagePublisher {
    channel: Arc<Channel>
}

impl RabbitMQModelDeploymentMessagePublisher {
    pub fn new(channel: Arc<Channel>) -> Self {
        Self {
            channel
        }
    }
}

#[async_trait]
impl EventPublisher for RabbitMQModelDeploymentMessagePublisher {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError> {    
        let payload = serialize_event(&event)
            .map_err(|err| EventPublisherError::Serialization(err.to_string()))?;

        // Publish to exchange
        let args = BasicPublishArguments::new(
            get_exchange_for_event(event),
            get_routing_key_for_event(event),
        )
            .mandatory(true)
            .finish();

        let props = BasicProperties::default()
            .with_message_id(Uuid::now_v7().to_string().as_str())
            .finish();

        self.channel.basic_publish(props, payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| EventPublisherError::Publishing(err.to_string()))?;
       
        Ok(())
    }
}