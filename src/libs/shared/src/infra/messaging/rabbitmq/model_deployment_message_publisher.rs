use amqprs::connection::OpenConnectionArguments;
use thiserror::Error;
use crate::application::ports::events::{
    EventPublisherError,
    EventPublisher,
    Event
};
use crate::infra::messaging::codec::serialize_event;
use crate::infra::messaging::rabbitmq::exchanges::get_exchange_for_event;
use crate::infra::messaging::rabbitmq::routing::get_routing_key_for_event;
use crate::infra::messaging::rabbitmq::exchanges::{delcare_exchanges, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE};
use crate::infra::messaging::rabbitmq::connection::open_channel;

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
impl EventPublisher for RabbitMQModelDeploymentMessagePublisher {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError> {    
        let payload = serialize_event(&event)
            .map_err(|err| EventPublisherError::Serialization(err.to_string()))?;

        // Publish to exchange
        let args = BasicPublishArguments::new(
            get_exchange_for_event(event),
            get_routing_key_for_event(event),
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
            .map_err(|err| EventPublisherError::Connection(err.to_string()))?;

        delcare_exchanges(&channel, vec![(MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, "topic")])
            .await
            .map_err(|err| EventPublisherError::Routing(err.to_string()))?;

        channel.basic_publish(BasicProperties::default(), payload.as_bytes().to_vec(), args)
            .await
            .map_err(|err| EventPublisherError::Publishing(err.to_string()))?;
       
        Ok(())
    }
}