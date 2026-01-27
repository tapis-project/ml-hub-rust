use crate::infra::messaging::rabbitmq::constants;
use crate::application::ports::events::{
    EventPublisherError,
    Event,
};
use amqprs::{
    channel::{
        Channel,
        ExchangeDeclareArguments,
    },
    connection::{
        Connection, 
        OpenConnectionArguments,
    }
};
use crate::infra::messaging::messages::{
    IngestArtifactMessage,
    PublishArtifactMessage,
    DeployModelWithStrategyMessage,
};

pub fn get_serialized_event_payload(event: &Event) -> Result<String, EventPublisherError> {
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
        Event::DeployModelWithStrategyEvent(payload) => {
            match serde_json::to_string(&DeployModelWithStrategyMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(EventPublisherError::SerializationError(err.to_string()));
                }
            };
        },
    }
}

pub fn get_exchange(event: &Event) -> &'static str {
    match event {
        Event::IngestArtifactEvent(_) => constants::ARTIFACT_INGESTION_EXCHANGE,
        Event::PublishArtifactEvent(_) => constants::ARTIFACT_PUBLICATION_EXCHANGE,
        Event::DeployModelWithStrategyEvent(_) => constants::MODEL_DEPLOYMENT_WITH_STRATEGY_EXCHANGE,
    }
}

pub fn get_routing_key(event: &Event) -> &'static str {
    match event {
        Event::IngestArtifactEvent(_) => constants::ARTIFACT_INGESTION_ROUTING_KEY,
        Event::PublishArtifactEvent(_) => constants::ARTIFACT_PUBLICATION_ROUTING_KEY,
        Event::DeployModelWithStrategyEvent(_) => constants::MODEL_DEPLOYMENT_WITH_STRATEGY_ROUTING_KEY,
    }
}

pub async fn delcare_exchanges(channel: &Channel, exchanges: Vec<(&'static str, &str)>) -> Result<(), EventPublisherError> {
    for (exchange, exchange_type) in exchanges {
        let exchange_args = ExchangeDeclareArguments::new(exchange, exchange_type);
        channel.exchange_declare(exchange_args).await
            .map_err(|err| EventPublisherError::ConnectionError(err.to_string()))?
    }
    Ok(())
}

pub async fn amqp_connection_builder(host: &String, port: &String, username: &String, password: &String) -> Result<Channel, EventPublisherError> {
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

    delcare_exchanges(
        &channel, 
        vec![
            (constants::ARTIFACT_INGESTION_EXCHANGE, "topic"),
            (constants::ARTIFACT_PUBLICATION_EXCHANGE, "topic")
        ]
    ).await?;

    Ok(channel)
}    