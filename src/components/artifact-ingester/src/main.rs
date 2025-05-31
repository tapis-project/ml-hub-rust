use lapin::{
    message::Delivery, options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, QueueDeclareOptions}, types::FieldTable, Connection, ConnectionProperties, Error
};
use tokio;
use uuid::Uuid;
use artifact_ingester::messages::{ArtifactType, StageArtifactJobRequest};
use std::io::ErrorKind;
use futures_util::stream::StreamExt;
use client_provider::ClientProvider;
use shared::constants::ARTIFACT_STAGING_QUEUE;

async fn connect_to_broker(uri: String, max_connection_attempts: i8) -> Connection {
    println!("Attempting to connect to broker");
    
    let mut connection_attempts: i8 = 1;
    while connection_attempts <= max_connection_attempts {
        // Attempt to connect. Out of all the possible errors, we only want to retry
        // the connection on the two IO errors below
        match Connection::connect(&uri, ConnectionProperties::default()).await {
            Ok(conn) => return conn, // Return the successful connection
            Err(err) => {
                connection_attempts += 1;
                match err {
                    Error::IOError(io_err) => {
                        match io_err.kind() {
                            ErrorKind::ConnectionRefused | ErrorKind::NotFound => {
                                println!("Failed to connect to message broker: Attempt {} of {}", connection_attempts, max_connection_attempts);
                                connection_attempts += 1;
                                continue;
                            },
                            other => panic!("Failed to connect to broker {}", other.to_string()),
                        };
                    },
                    other => panic!("Failed to connect to message broker: {}", other.to_string())
                };
            }
        }
    }

    panic!("Failed to connect to message broker. Max attempts reached: {}", max_connection_attempts);
}

async fn ack(delivery: &Delivery, opts: BasicAckOptions) {
    if let Err(err) = delivery.ack(opts).await {
        eprintln!("CRITICAL: Failed to ack message: {}", err.to_string());
        panic!("Cannot ack. Shutting down to avoid inconsistent state.");
    }
}

async fn nack(delivery: &Delivery, opts: BasicNackOptions) {
    if let Err(err) = delivery.nack(opts).await {
        eprintln!("CRITICAL: Failed to nack message: {}", err.to_string());
        panic!("Cannot nack. Shutting down to avoid inconsistent state.");
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    const PROTOCOL: &str = "ampq";

    let host = std::env::var("BROKER_HOST").expect("BROKER_URL missing from environment variables");
    let port = std::env::var("BROKER_PORT").expect("BROKER_PORT missing from environment variables");
    let username = std::env::var("BROKER_USER").expect("BROKER_USER missing from environment variables");
    let password = std::env::var("BROKER_PASSWORD").expect("BROKER_PASSWORD missing from environment variables");

    let uri = format!(
        "{}://{}:{}@{}:{}/%2f",
        PROTOCOL,
        username,
        password,
        host,
        port
    );

    // Connect to the broker
    let conn = connect_to_broker(uri, 25).await;

    // Create a channel
    let channel = match conn.create_channel().await {
        Ok(c) => c,
        Err(err) => panic!("Failed to create channel: {}", err.to_string())
    };

    // Declare queue. Note, this is an idempotent delacration. Redeclaring an existing
    // queue with the same parameters will not reuslt in an error.
    let _ = match channel.queue_declare(ARTIFACT_STAGING_QUEUE, QueueDeclareOptions::default(), FieldTable::default()).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to create channel: {}", err.to_string())
    };
    
    // Unique consumer tag. Make this unique per worker. 
    let consumer_tag = Uuid::now_v7();

    // Start consuming
    let maybe_consumer = channel
        .basic_consume(
            ARTIFACT_STAGING_QUEUE,
            consumer_tag.to_string().as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await;

    let mut consumer = match maybe_consumer {
        Ok(c) => c,
        Err(err) => {
            panic!("Error consuming queue: {}", err.to_string());
        }
    };

    println!("Ready to recieve messages");
    
    while let Some(result) = consumer.next().await {
        let delivery = match result {
            Ok(delivery) => delivery,
            Err(err) => {
                eprintln!("Error in consumer '{}' attmpting to consume the message: {}", &consumer_tag, err.to_string());
                continue;
            }
        };

        // Deserialize the message into a DownloadArtifactRequest
        let request: StageArtifactJobRequest = match serde_json::from_slice(&delivery.data) {
            Ok(message) => message,
            Err(err) => {
                eprintln!("Deserialization error in consumer '{}': {}", &consumer_tag, err.to_string());
                nack(&delivery, BasicNackOptions::default()).await;
                continue;
            }
        };

        match request.r#type {
            ArtifactType::Model => {
                match ClientProvider::provide_download_model_client(&request.platform) {
                    Ok(client) => {
                        client.download_model(request)
                    },
                    Err(err) => {
                        eprintln!("Client provider error in consumer '{}': {}", &consumer_tag, err.to_string());
                        nack(&delivery, BasicNackOptions::default()).await;
                        continue;
                    }
                }
            },
            ArtifactType::Dataset => {
                // let client = ClientProvider::provide_download_dataseet_client(&request.platform)
                //     .map_err(|err| {
                //         eprintln!("{}", err);
                //     });

                nack(&delivery, BasicNackOptions::default()).await;
            }
        };

        // Dispatch client to download
        // Update database
            
        // Acknowledge the message
        ack(&delivery, BasicAckOptions::default()).await;
    }
}

