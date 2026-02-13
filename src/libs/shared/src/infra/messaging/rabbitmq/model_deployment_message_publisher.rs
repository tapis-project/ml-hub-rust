use amqprs::channel::ExchangeType;
use thiserror::Error;
use crate::application::ports::events::{
    EventPublisherError,
    EventPublisher,
    Event
};
use crate::infra::messaging::codec::serialize_event;
use crate::infra::messaging::rabbitmq::exchanges::get_exchange_for_event;
use crate::infra::messaging::rabbitmq::routing::get_routing_key_for_event;
use crate::infra::messaging::rabbitmq::exchanges::{declare_exchanges, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE};
use crate::infra::messaging::rabbitmq::connection::open_channel;

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
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl RabbitMQModelDeploymentMessagePublisher {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Self {
            host,
            port,
            username,
            password
        }
    }
}

#[async_trait]
impl EventPublisher for RabbitMQModelDeploymentMessagePublisher {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError> {    
        let payload = serialize_event(&event)
            .map_err(|err| EventPublisherError::Serialization(err.to_string()))?;

        let (_, channel) = open_channel(
            self.host.clone(),
            self.port,
            self.username.clone(),
            self.password.clone(),
        )
            .await
            .map_err(|err| EventPublisherError::Connection(err.to_string()))?;

        declare_exchanges(&channel, vec![(MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, ExchangeType::Topic)])
            .await
            .map_err(|err| EventPublisherError::Routing(err.to_string()))?;

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

        channel.basic_publish(props, payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| EventPublisherError::Publishing(err.to_string()))?;
       
        Ok(())
    }
}