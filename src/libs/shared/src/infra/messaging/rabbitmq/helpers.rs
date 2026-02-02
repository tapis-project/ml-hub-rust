use crate::infra::messaging::rabbitmq::constants;
use crate::application::ports::commands::{
    CommandPublisherError,
    Command,
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

pub enum MessageType {
    Command(Command),
}

pub fn get_serialized_command_payload(command: &Command) -> Result<String, CommandPublisherError> {
    match command {
        Command::IngestArtifactCommand(payload) => {
            match serde_json::to_string(&IngestArtifactMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(CommandPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Command::PublishArtifactCommand(payload) => {
            match serde_json::to_string(&PublishArtifactMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(CommandPublisherError::SerializationError(err.to_string()));
                }
            };
        },
        Command::DeployModelWithStrategyCommand(payload) => {
            match serde_json::to_string(&DeployModelWithStrategyMessage::from(payload)) {
                Ok(p) => return Ok(p),
                Err(err) => {
                    return Err(CommandPublisherError::SerializationError(err.to_string()));
                }
            };
        },
    }
}

pub fn get_exchange(command: &Command) -> &'static str {
    match command {
        Command::IngestArtifactCommand(_) => constants::ARTIFACT_INGESTION_EXCHANGE,
        Command::PublishArtifactCommand(_) => constants::ARTIFACT_PUBLICATION_EXCHANGE,
        Command::DeployModelWithStrategyCommand(_) => constants::MODEL_DEPLOYMENT_WITH_STRATEGY_EXCHANGE,
    }
}

pub fn get_routing_key(command: &Command) -> &'static str {
    match command {
        Command::IngestArtifactCommand(_) => constants::ARTIFACT_INGESTION_ROUTING_KEY,
        Command::PublishArtifactCommand(_) => constants::ARTIFACT_PUBLICATION_ROUTING_KEY,
        Command::DeployModelWithStrategyCommand(_) => constants::MODEL_DEPLOYMENT_WITH_STRATEGY_ROUTING_KEY,
    }
}

pub async fn delcare_exchanges(channel: &Channel, exchanges: Vec<(&'static str, &str)>) -> Result<(), CommandPublisherError> {
    for (exchange, exchange_type) in exchanges {
        let exchange_args = ExchangeDeclareArguments::new(exchange, exchange_type);
        channel.exchange_declare(exchange_args).await
            .map_err(|err| CommandPublisherError::ConnectionError(err.to_string()))?
    }
    Ok(())
}

pub async fn amqp_connection_builder(host: &String, port: &String, username: &String, password: &String) -> Result<Channel, CommandPublisherError> {
    let connection_args = OpenConnectionArguments::new(
        host.as_str(),
        port.parse::<u16>().unwrap_or(5672),
        username.as_str(),
        password.as_str()
    );

    let conn = match Connection::open(&connection_args).await {
        Ok(conn) => conn,
        Err(err) => return Err(CommandPublisherError::AmqpError(err.to_string()))
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

async fn connect_to_channel(args: &OpenConnectionArguments, max_connection_attempts: i8) -> Connection {
    println!("Attempting to connect to broker");
    
    let mut connection_attempts: i8 = 0;
    while connection_attempts <= max_connection_attempts {
        // Attempt to connect. Out of all the possible errors, we only want to retry
        // the connection on the two IO errors below
        
        // Open connection
        let maybe_connection = Connection::open(args)
            .await;

        match maybe_connection {
            Ok(conn) => return conn, // Return the successful connection
            Err(err) => {
                connection_attempts += 1;
                match err {
                    Error::NetworkError(_) => {
                        println!("Failed to connect to message broker: Attempt {} of {}", connection_attempts, max_connection_attempts);
                        connection_attempts += 1;
                        continue;
                    },
                    other => panic!("Failed to connect to message broker: {}", other.to_string())
                };
            }
        }
    }

    panic!("Failed to connect to message broker. Max attempts reached: {}", max_connection_attempts);
}

async fn ack(channel: &Channel, deliver: &Deliver, multiple: Option<bool>) {
    let args = BasicAckArguments {
        delivery_tag: deliver.delivery_tag(),
        multiple: multiple.unwrap_or(false)
    };

    if let Err(err) = channel.basic_ack(args).await {
        eprintln!("CRITICAL: Failed to ack message: {}", err.to_string());
        panic!("Cannot ack. Shutting down to avoid inconsistent state.");
    }
}

async fn nack(channel: &Channel, deliver: &Deliver, requeue: Option<bool>, multiple: Option<bool>) {
    let args = BasicNackArguments {
        delivery_tag: deliver.delivery_tag(),
        requeue: requeue.unwrap_or(false),
        multiple: multiple.unwrap_or(false)
    };

    if let Err(err) = channel.basic_nack(args).await {
        eprintln!("CRITICAL: Failed to nack message: {}", err.to_string());
        panic!("Cannot nack. Shutting down to avoid inconsistent state.");
    }
}