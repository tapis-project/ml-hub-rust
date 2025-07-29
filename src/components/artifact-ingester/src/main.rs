use std::path::PathBuf;
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
use shared::common::domain::entities::{ArtifactType, ArtifactIngestionFailureReason, ArtifactIngestionStatus};
use shared::logging::GlobalLogger;
use shared::constants::{ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_INGESTION_QUEUE, ARTIFACT_INGESTION_ROUTING_KEY, DATASET_INGEST_DIR_NAME, MODEL_INGEST_DIR_NAME};
use shared::models::presentation::http::v1::dto::IngestModelRequest;
use shared::common::infra::system::Env;
// use shared::datasets::presentation::http::v1::dto::IngestDatasetRequest;
use shared::common::infra::messaging::messages::IngestArtifactMessage;
use async_trait::async_trait;
use shared::common::application::services::artifact_service::ArtifactService;
use std::env;
use artifact_ingester::bootstrap::artifact_service_factory;
use artifact_ingester::database::{get_db, ClientParams};
use shared::common::infra::fs::compression::FileCompressor;

struct ArtifactIngesterConsumer {
    artifact_service: ArtifactService,
    models_target_base_path: PathBuf,
    datasets_target_base_path: PathBuf,
    artifacts_cache_dir: PathBuf,
}

#[async_trait]
impl AsyncConsumer for ArtifactIngesterConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, _basic_properties: BasicProperties, content: Vec<u8>) {
        // Deserialize the message into a DownloadArtifactRequest
        let request: IngestArtifactMessage = match serde_json::from_slice(&content) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Deserialization error in consumer '{}': {}", &deliver.consumer_tag(), err.to_string());
                nack(&channel, &deliver, None, None).await;
                return;
            }
        };

        let ingestion_id = Uuid::parse_str(request.ingestion_id.as_str()).expect("Invalid Uuid. Cannot convert ingestion_id into Uuid");

        self.artifact_service.change_ingestion_status_by_ingestion_id(
            ingestion_id.clone(),
            ArtifactIngestionStatus::Pending,
            Some("Ingestion pending".into())
        )
            .await
            .map_err(|err| {
                panic!("Error updating ingestion status: {}", err.to_string())
            }).unwrap();

        let ref mut artifact = self.artifact_service.find_artifact_by_ingestion_id(
            ingestion_id.clone()
        ).await
            .expect("Failed to fetch artifact")
            .expect(format!("Could not find artifact associated with ingestion '{}'", &ingestion_id).as_str());

        let download_path = match artifact.artifact_type {
            ArtifactType::Model => self.models_target_base_path.join(artifact.id.to_string()),
            ArtifactType::Dataset => self.datasets_target_base_path.join(artifact.id.to_string())
        };

        GlobalLogger::debug(format!("download path: {}", &download_path.to_string_lossy().to_string()).as_str());
     
        match artifact.artifact_type {
            ArtifactType::Model => {
                match ClientProvider::provide_ingest_model_client(&request.platform) {
                    Ok(client) => {
                        self.artifact_service.change_ingestion_status_by_ingestion_id(
                            ingestion_id.clone(),
                            ArtifactIngestionStatus::Downloading,
                            Some("Download in progress".into())
                        )
                            .await
                            .map_err(|err| {
                                panic!("Error updating ingestion status: {}", err.to_string())
                            }).unwrap();

                        let client_request: IngestModelRequest = serde_json::from_slice(&request.serialized_client_request)
                            .expect("Failed deserializing the client request");

                        match client.ingest_model(&client_request, download_path.clone()).await {
                            Ok(_) => {
                                // Update ingestion to Downloaded
                                self.artifact_service.change_ingestion_status_by_ingestion_id(
                                    ingestion_id.clone(),
                                    ArtifactIngestionStatus::Downloaded,
                                    Some("Download complete".into())
                                )
                                    .await
                                    .map_err(|err| {
                                        panic!("Error updating ingestion status: {}", err.to_string())
                                    }).unwrap();
                                
                                // Update ingestion to Archiving
                                self.artifact_service.change_ingestion_status_by_ingestion_id(
                                    ingestion_id.clone(),
                                    ArtifactIngestionStatus::Archiving,
                                    Some("Archiving started".into())
                                )
                                    .await
                                    .map_err(|err| {
                                        panic!("Error updating ingestion status: {}", err.to_string())
                                    }).unwrap();

                                let maybe_artifact_path = FileCompressor::zip(
                                    &download_path,
                                    &PathBuf::from(&self.artifacts_cache_dir).join(artifact.id.clone().to_string()),
                                    None,
                                );

                                let artifact_path = match maybe_artifact_path {
                                    Ok(p) => p,
                                    Err(err) => {
                                        self.artifact_service.change_ingestion_status_by_ingestion_id(
                                            ingestion_id.clone(),
                                            ArtifactIngestionStatus::Failed(ArtifactIngestionFailureReason::FailedToArchive),
                                            Some(err.to_string())
                                        )
                                            .await
                                            .map_err(|err| {
                                                panic!("Error updating ingestion status: {}", err.to_string())
                                            }).unwrap();
                                        return 
                                    }
                                };
                                
                                // Update ingestion to Archived
                                self.artifact_service.change_ingestion_status_by_ingestion_id(
                                    ingestion_id.clone(),
                                    ArtifactIngestionStatus::Archived,
                                    Some("Successfully Archived".into())
                                )
                                    .await
                                    .map_err(|err| {
                                        panic!("Error updating ingestion status: {}", err.to_string())
                                    }).unwrap();

                                // Clean up the ingestion workdir
                                // std::fs::remove_dir_all(&download_path)
                                //     .expect(format!("Error removing files at path {}", &download_path.to_string_lossy().to_string()).as_str());

                                let ref mut ingestion = self.artifact_service.find_ingestion_by_ingestion_id(ingestion_id)
                                    .await
                                    .expect("Error fetching ingestion")
                                    .expect("Ingestion should exist but does not");
                                
                                // Set the path to the artifact on the Artifact itself
                                self.artifact_service.finish_artifact_ingestion(artifact_path, artifact, ingestion)
                                    .await
                                    .map_err(|err| panic!("Error finishing artifact ingestion: {}", err.to_string()))
                                    .unwrap();
                            },
                            Err(err) => {
                                self.artifact_service.change_ingestion_status_by_ingestion_id(
                                    ingestion_id.clone(),
                                    ArtifactIngestionStatus::Failed(ArtifactIngestionFailureReason::FailedToDownload),
                                    Some(err.to_string())
                                )
                                    .await
                                    .map_err(|err| {
                                        panic!("Error updating ingestion status: {}", err.to_string())
                                    }).unwrap();

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
            
        // Acknowledge the message
        ack(&channel, &deliver, None).await;
    }
}

async fn connect_to_broker(args: &OpenConnectionArguments, max_connection_attempts: i8) -> Connection {
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

#[tokio::main]
async fn main() -> () {
    env_logger::init();

    let host = std::env::var("ARTIFACT_OP_MQ_HOST").expect("ARTIFACT_OP_MQ_HOST missing from environment variables");
    let port = std::env::var("ARTIFACT_OP_MQ_PORT").expect("ARTIFACT_OP_MQ_PORT missing from environment variables");
    let username = std::env::var("ARTIFACT_OP_MQ_USER").expect("ARTIFACT_OP_MQ_USER missing from environment variables");
    let password = std::env::var("ARTIFACT_OP_MQ_PASSWORD").expect("ARTIFACT_OP_MQ_PASSWORD missing from environment variables");

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

    // Database connection
    let db = get_db(ClientParams{
        username: env::var("ARTIFACTS_DB_USERNAME").expect("ARTIFACTS_DB_USERNAME env var not set"),
        password: env::var("ARTIFACTS_DB_PASSWORD").expect("ARTIFACTS_DB_PASSWORD env var not set"),
        host: env::var("ARTIFACTS_DB_HOST").expect("ARTIFACTS_DB_HOST env var not set"),
        port: env::var("ARTIFACTS_DB_PORT").expect("ARTIFACTS_DB_PORT env var not set"),
        db: env::var("ARTIFACTS_DB_NAME").expect("ARTIFACTS_DB_NAME env var not set"),
    })
        .await
        .map_err(|err| {
            panic!("Database initialization error: {}", err.to_string().as_str()); 
        })
        .expect("Datbase initialization error");
    
    let environment = Env::new().expect("Env could not be initialized");

    let consumer = ArtifactIngesterConsumer {
        artifact_service: artifact_service_factory(&db).await.expect("failed to initialize artifact service"),
        models_target_base_path: PathBuf::from(&environment.shared_data_dir).join(MODEL_INGEST_DIR_NAME),
        datasets_target_base_path: PathBuf::from(&environment.shared_data_dir).join(DATASET_INGEST_DIR_NAME),
        artifacts_cache_dir: PathBuf::from(&environment.artifacts_cache_dir)
    };
     
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

