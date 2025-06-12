use amqprs::{
    channel::{
        BasicAckArguments, 
        BasicConsumeArguments, 
        BasicNackArguments, 
        Channel, 
        QueueDeclareArguments,
        ExchangeDeclareArguments,
        QueueBindArguments,
        ExchangeType
    },
    connection::{
        Connection, 
        OpenConnectionArguments
    },
    consumer::AsyncConsumer,
    error::Error,
    BasicProperties,
    Deliver
};
use tokio;
use uuid::Uuid;
use client_provider::ClientProvider;
use shared::constants::{ARTIFACT_INGESTION_QUEUE, ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_INGESTION_ROUTING_KEY};
use shared::models::presentation::http::v1::dto::IngestModelRequest;
// use shared::datasets::presentation::http::v1::dto::IngestDatasetRequest;
use shared::common::infra::messaging::messages::{ArtifactType, IngestArtifactMessage};
use async_trait::async_trait;

#[derive(Debug)]
struct ArtifactIngesterConsumer;

#[async_trait]
impl AsyncConsumer for ArtifactIngesterConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, _basic_properties: BasicProperties, content:Vec<u8>) {
        // Deserialize the message into a DownloadArtifactRequest
        let request: IngestArtifactMessage = match serde_json::from_slice(&content) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Deserialization error in consumer '{}': {}", &deliver.consumer_tag(), err.to_string());
                nack(&channel, &deliver, None, None).await;
                return;
            }
        };

        match request.artifact_type {
            ArtifactType::Model => {
                match ClientProvider::provide_ingest_model_client(&request.platform) {
                    Ok(client) => {
                        let client_request: IngestModelRequest = serde_json::from_slice(&request.serialized_client_request).unwrap();
                        // TODO Handle the error
                        match client.ingest_model(&client_request) {
                            Ok(_) => {
                                // TODO update the artifact ingestion with status Finished
                                // TODO update the artifact with the path to the artifact
                            },
                            Err(err) => {
                                eprintln!("{}", err.to_string());
                                nack(&channel, &deliver, None, None).await;
                            }
                        };
                    },
                    Err(err) => {
                        eprintln!("Client provider error in consumer '{}': {}", &deliver.consumer_tag(), err.to_string());
                        nack(&channel, &deliver, None, None).await;
                        return;
                    }
                };
            },
            ArtifactType::Dataset => {
                // let client = ClientProvider::provide_ingest_dataset_client(&request.platform)
                //     .map_err(|err| {
                //         eprintln!("{}", err);
                //     });
                eprintln!("Artifact ingestion not yet available for datasets");
                nack(&channel, &deliver, None, None).await;
                return 
            }
        };

        // Update database
            
        // Acknowledge the message
        ack(&channel, &deliver, None).await;
    }
}

async fn connect_to_broker(args: &OpenConnectionArguments, max_connection_attempts: i8) -> Connection {
    println!("Attempting to connect to broker");
    
    let mut connection_attempts: i8 = 1;
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

#[tokio::main]
async fn main() -> () {
    env_logger::init();

    let host = std::env::var("BROKER_HOST").expect("BROKER_URL missing from environment variables");
    let port = std::env::var("BROKER_PORT").expect("BROKER_PORT missing from environment variables");
    let username = std::env::var("BROKER_USER").expect("BROKER_USER missing from environment variables");
    let password = std::env::var("BROKER_PASSWORD").expect("BROKER_PASSWORD missing from environment variables");

    let connection_args = OpenConnectionArguments::new(
        host.as_str(),
        port.parse::<u16>().unwrap_or(5672),
        username.as_str(),
        password.as_str()
    );

    // Connect to the broker
    let conn = connect_to_broker(&connection_args, 25).await;

    // Create a channel
    let channel = match conn.open_channel(None).await {
        Ok(c) => c,
        Err(err) => panic!("Failed to open channel: {}", err.to_string())
    };

    // Declare queue
    let _ = match channel.queue_declare(QueueDeclareArguments::new(ARTIFACT_INGESTION_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to create channel: {}", err.to_string())
    };

    match channel.exchange_declare(
        ExchangeDeclareArguments::new(
            ARTIFACT_INGESTION_EXCHANGE,
            ExchangeType::Topic.to_string().as_str()
        )
    ).await {
        Ok(_) => {},
        Err(err) => panic!("Failed to delare exchange: {}", err.to_string())
    };
    
     match channel.queue_bind(
        QueueBindArguments::new(
            ARTIFACT_INGESTION_QUEUE,
            ARTIFACT_INGESTION_EXCHANGE, 
            ARTIFACT_INGESTION_ROUTING_KEY
        )
    ).await {
        Ok(_) => {},
        Err(err) => panic!("Failed to bind queue: {}", err.to_string())
    };
    
    // Unique consumer tag. Make this unique per worker. 
    let consumer_tag = Uuid::now_v7();

    let consumer = ArtifactIngesterConsumer;
    let args = BasicConsumeArguments::default()
        .queue(ARTIFACT_INGESTION_QUEUE.into())
        .consumer_tag(consumer_tag.to_string())
        .finish();

    match channel.basic_consume(consumer, args).await {
        Ok(_) => { println!("Ready to recieve messages...") },
        Err(err) => panic!("Failed to consume: {}", err.to_string())
    };

    // Block forever or until terminated
    if let Err(err) = tokio::signal::ctrl_c().await {
        panic!("{}", err.to_string())
    }
}

