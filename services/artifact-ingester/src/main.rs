use std::sync::Arc;
use std::path::PathBuf;
use amqprs::{
    channel::{
        BasicAckArguments, 
        BasicConsumeArguments, 
        BasicNackArguments, 
        Channel, 
        QueueDeclareArguments,
        QueueBindArguments,
        ExchangeType
    },
    consumer::AsyncConsumer,
    BasicProperties,
    Deliver
};
use tokio;
use uuid::Uuid;
use client_provider::ClientProvider;
use shared::{constants::ARTIFACT_INGEST_DIR_NAME, infra::messaging::rabbitmq::{connection::open_channel, exchanges::declare_exchanges}};
use shared::domain::entities::artifact_ingestion::ArtifactIngestionStatus;
use shared::domain::entities::artifact::ArtifactType;
use shared::infra::messaging::rabbitmq::exchanges::ARTIFACT_INGESTION_EXCHANGE;
use shared::infra::messaging::rabbitmq::queues::ARTIFACT_INGESTION_QUEUE;
use shared::infra::messaging::rabbitmq::routing::ARTIFACT_INGESTION_ROUTING_KEY;
use shared::presentation::http::v1::requests::models::IngestModelRequest;
use shared::infra::system::Env;
// use shared::datasets::presentation::http::v1::requests::IngestDatasetRequest;
use shared::infra::messaging::messages::IngestArtifactMessage;
use async_trait::async_trait;
use shared::application::services::artifact_service::ArtifactService;
use std::env;
use artifact_ingester::bootstrap::artifact_service_factory;
use artifact_ingester::database::{inialize_client, ClientParams};
use shared::infra::fs::archiver::Archiver;
use log::{error, info};

struct ArtifactIngesterConsumer {
    artifact_service: ArtifactService,
    artifacts_work_dir: PathBuf,
    artifacts_cache_dir: PathBuf,
}

#[async_trait]
impl AsyncConsumer for ArtifactIngesterConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, _basic_properties: BasicProperties, content: Vec<u8>) {
        // Deserialize the message
        let request: IngestArtifactMessage = match serde_json::from_slice(&content) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Deserialization error in consumer '{}': {}", &deliver.consumer_tag(), err.to_string());
                nack(&channel, &deliver, None, None).await;
                return;
            }
        };

        let ingestion_id = Uuid::parse_str(request.ingestion_id.as_str()).expect("Invalid Uuid. Cannot convert ingestion_id into Uuid");

        // Update artifact ingestion to Pending
        self.artifact_service.change_ingestion_status_by_ingestion_id(
            ingestion_id.clone(),
            ArtifactIngestionStatus::Pending,
            Some("Ingestion pending".into())
        )
            .await
            .map_err(|err| {
                panic!("Error updating ingestion status: {}", err.to_string())
            }).unwrap();

        // Fetch the artifact related to the ingestion
        let ref mut artifact = self.artifact_service.find_artifact_by_ingestion_id(
            ingestion_id.clone()
        ).await
            .expect("Failed to fetch artifact")
            .expect(format!("Could not find artifact associated with ingestion '{}'", &ingestion_id).as_str());

        // Set the download path based on whether this is a model or a dataset
        let download_path = self.artifacts_work_dir.join(artifact.id.to_string());

        // Ingest the artifact
        match artifact.artifact_type {
            ArtifactType::Model => {
                // Get the correct client to do the model ingestion
                match ClientProvider::provide_ingest_model_client(&request.platform) {
                    Ok(client) => {
                        // Update the ingestion to Downloading
                        self.artifact_service.change_ingestion_status_by_ingestion_id(
                            ingestion_id.clone(),
                            ArtifactIngestionStatus::Downloading,
                            Some("Download in progress".into())
                        )
                            .await
                            .map_err(|err| {
                                panic!("Error updating ingestion status: {}", err.to_string())
                            }).unwrap();
                        
                        // Deserialize the client request
                        let client_request: IngestModelRequest = serde_json::from_slice(&request.serialized_client_request)
                            .expect("Failed deserializing the client request");
                        
                        // Ingest the model
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
                                
                                // Archive the artifact files with compression
                                let maybe_artifact_path = Archiver::zip(
                                    &download_path,
                                    &PathBuf::from(&self.artifacts_cache_dir).join(artifact.id.clone().to_string()),
                                    None,
                                    // This is the base path, this path will be stripped from every file
                                    // and directory that is written
                                    Some(
                                        self.artifacts_work_dir.join(artifact.id.clone().to_string())
                                            .to_string_lossy()
                                            .into_owned()
                                            .as_str()
                                    ),
                                );

                                // Get the artifact path
                                let artifact_path = match maybe_artifact_path {
                                    Ok(p) => p,
                                    Err(err) => {
                                        self.artifact_service.change_ingestion_status_by_ingestion_id(
                                            ingestion_id.clone(),
                                            ArtifactIngestionStatus::Failed,
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
                                    Some("Successfully ingested".into())
                                )
                                    .await
                                    .map_err(|err| {
                                        panic!("Error updating ingestion status: {}", err.to_string())
                                    }).unwrap();

                                // Clean up the ingestion workdir
                                std::fs::remove_dir_all(&download_path)
                                    .expect(format!("Error removing files at path {}", &download_path.to_string_lossy().to_string()).as_str());
                                
                                // Get the updated ingestion
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
                                    ArtifactIngestionStatus::Failed,
                                    Some(err.to_string())
                                )
                                    .await
                                    .map_err(|err| {
                                        panic!("Error updating ingestion status: {}", err.to_string())
                                    }).unwrap();

                                eprintln!("{}", err.to_string());
                                nack(&channel, &deliver, None, None).await;
                                return;
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
            // Ingest the dataset
            ArtifactType::Dataset => {
                eprintln!("Artifact ingestion not yet available for datasets");
                nack(&channel, &deliver, None, None).await;
                return 
            }
        };
            
        // Acknowledge the message
        ack(&channel, &deliver, None).await;
    }
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

    // Database connection
    let db_name = env::var("MONGO_NAME").expect("MONGO_NAME env var not set");

    let client = inialize_client(ClientParams {
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: env::var("MONGO_NAME").expect("MONGO_NAME env var not set"),
    })
        .await
        .map_err(|err| {
            panic!("Database initialization error: {}", err.to_string().as_str()); 
        })
        .expect("Datbase initialization error");

    let environment = Env::new().expect("Env could not be initialized");

    let broker_host = std::env::var("RABBIT_HOST").expect("RABBIT_URL missing from environment variables");
    let broker_port = std::env::var("RABBIT_PORT").expect("RABBIT_PORT missing from environment variables");
    let broker_username = std::env::var("RABBIT_USER").expect("RABBIT_USER missing from environment variables");
    let broker_password = std::env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD missing from environment variables");
    
    // Create channel
    let (_connection, channel) = open_channel(
        broker_host,
        broker_port.parse::<u16>().expect("u16 parsed from 'port' String"),
        broker_username,
        broker_password,
    )
        .await
        .map_err(|err| { error!("{}", err.to_string()) })
        .expect("Connection to message broker established and channel created");

    // Declare exchanges
    declare_exchanges(
        &channel, 
        vec![
            (ARTIFACT_INGESTION_EXCHANGE, ExchangeType::Topic),
        ]
    ).await
        .map_err(|err| { error!("{}", err.to_string())})
        .expect(format!("Exchanges {} to be declared", ARTIFACT_INGESTION_EXCHANGE).as_str());
    
    // Declare queue
    let _ = match channel.queue_declare(QueueDeclareArguments::new(ARTIFACT_INGESTION_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to delcare queue: {}", err.to_string())
    };
    
    // Bind queue to exchange
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

    // Unique consumer tag
    let consumer_tag = Uuid::now_v7();

    let consumer = ArtifactIngesterConsumer {
        artifact_service: artifact_service_factory(&client, db_name, Arc::new(channel.clone())),
        artifacts_work_dir: PathBuf::from(&environment.shared_data_dir).join(ARTIFACT_INGEST_DIR_NAME),
        artifacts_cache_dir: PathBuf::from(&environment.artifacts_cache_dir)
    };
     
    let args = BasicConsumeArguments::default()
        .queue(ARTIFACT_INGESTION_QUEUE.into())
        .consumer_tag(consumer_tag.to_string())
        .finish();

    match channel.basic_consume(consumer, args).await {
        Ok(_) => { info!("Consumer {} ready to recieve messages...", &consumer_tag) },
        Err(err) => panic!("Failed to consume: {}", err.to_string())
    };

    // Block forever or until terminated
    if let Err(err) = tokio::signal::ctrl_c().await {
        panic!("{}", err.to_string())
    }
}