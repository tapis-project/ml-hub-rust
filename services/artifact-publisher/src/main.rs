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
use shared::{domain::entities::artifact_publication::ArtifactPublicationStatus, infra::messaging::rabbitmq::{connection::open_channel, exchanges::declare_exchanges}};
use shared::domain::entities::artifact::ArtifactType;
use shared::infra::messaging::rabbitmq::exchanges::ARTIFACT_PUBLICATION_EXCHANGE;
use shared::infra::messaging::rabbitmq::queues::ARTIFACT_PUBLICATION_QUEUE;
use shared::infra::messaging::rabbitmq::routing::ARTIFACT_PUBLICATION_ROUTING_KEY;
use shared::presentation::http::v1::requests::artifacts::PublishArtifactServiceRequest;
use shared::infra::system::Env;
use shared::constants::ARTIFACT_PUBLICATION_DIR_NAME;
// use shared::datasets::presentation::http::v1::requests::IngestDatasetRequest;
use shared::infra::messaging::messages::PublishArtifactMessage;
use async_trait::async_trait;
use shared::application::services::artifact_service::ArtifactService;
use std::env;
use artifact_publisher::bootstrap::artifact_service_factory;
use artifact_publisher::database::{initialize_client, ClientParams};
use shared::infra::fs::archiver::Archiver;
use clients::{ClientError, PublishModelClient, PublishModelMetadataClient};
use log::{error, info};

struct ArtifactPublisherConsumer {
    artifact_service: ArtifactService,
    publications_work_dir: PathBuf,
}

#[async_trait]
impl AsyncConsumer for ArtifactPublisherConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, _basic_properties: BasicProperties, content: Vec<u8>) {
        // Deserialize the message
        let request: PublishArtifactMessage = match serde_json::from_slice(&content) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Deserialization error in consumer '{}': {}", &deliver.consumer_tag(), err.to_string());
                nack(&channel, &deliver, None, None).await;
                return;
            }
        };

        // Deserialize the client request
        let client_request: PublishArtifactServiceRequest = serde_json::from_slice(&request.serialized_client_request)
            .expect("Failed deserializing the client request");

        let publication_id = Uuid::parse_str(request.publication_id.as_str()).expect("Invalid Uuid. Cannot convert publication_id into Uuid");

        // Fetch the publication
        let ref mut publication = self.artifact_service.find_publication_by_publication_id(
            publication_id.clone()
        ).await
            .expect("Failed to fetch publication")
            .expect(format!("Could not find publication '{}'", &publication_id).as_str());

        // Fetch artifact associated with the publication
        let artifact = self.artifact_service.find_artifact_by_artifact_id(
            publication.artifact_id.clone()
        ).await
            .expect("Failed to fetch artifact")
            .expect(format!("Could not find artifact '{}'", &publication.artifact_id).as_str());

        // Check that the artifact is fully ingested
        if !artifact.is_fully_ingested() {
            panic!("Artifact '{}' not fully ingested", artifact.id.to_string())
        }

        // Get the artifact path
        let artifact_path = match self.artifact_service.get_ingested_artifact_path(&artifact) {
            Ok(p) => p,
            Err(err) => { panic!("{}", err.to_string()) }
        };

        // Publish the artifact
        match artifact.artifact_type {
            ArtifactType::Model => {
                // Fetch metadata associated with the model
                let maybe_metadata = self.artifact_service.find_metadata_by_artifact_id(
                    &publication.artifact_id
                ).await
                    .expect(format!("Failed to fetch metadata for artifact '{}'", &artifact.id.to_string()).as_str());

                // Update artifact publication to Pending
                self.artifact_service.change_publication_status_by_publication_id(
                    publication.id.clone(),
                    ArtifactPublicationStatus::Pending,
                    Some("Publication pending".into())
                )
                .await
                .map_err(|err| {
                    panic!("Error updating publication status: {}", err.to_string())
                }).unwrap();
                
                // Check whether at least one of the publish_model_client or the 
                // publish_metadata_client exists
                let (maybe_publish_model_client, maybe_publish_metadata_client) = {
                    let maybe_model = ClientProvider::provide_publish_model_client(&publication.target_platform);
                    let maybe_meta  = ClientProvider::provide_publish_metadata_client(&publication.target_platform);
                
                    match (maybe_model, maybe_meta) {
                        (Err(_), Err(_)) => panic!(
                            "Failed to find a client for both model and metadata publishing for platform {}",
                            publication.target_platform
                        ),
                        (Ok(model), meta) => (Some(model), meta.ok()),
                        (model, Ok(meta)) => (model.ok(), Some(meta)),
                    }
                };

                // Extract the artifact files and publish those files to the target
                // platform
                if let Some(client) = maybe_publish_model_client {
                    // Update publication status to Extracting
                    self.artifact_service.change_publication_status_by_publication_id(
                        publication_id.clone(),
                        ArtifactPublicationStatus::Extracting,
                        Some("Extracting artifact files".into())
                    )
                        .await
                        .map_err(|err| {
                            panic!("Error updating publication status: {}", err.to_string())
                        }).unwrap();
                    
                    // Path to which the files should be extracted
                    let extracted_artifact_path = self.publications_work_dir.clone()
                        .join(PathBuf::from(publication.id.to_string().clone()));
                    
                    // Extract the archived artifact files
                    let _ = Archiver::unzip(
                        &artifact_path,
                        &extracted_artifact_path,
                        None,
                    ).map_err(|err| panic!("Error extracting artifact {}: {}", artifact.id.to_string(), err.to_string()));
                    
                    // Update publication status to Extracted
                    self.artifact_service.change_publication_status_by_publication_id(
                        publication_id.clone(),
                        ArtifactPublicationStatus::Extracted,
                        Some("Successfully extracted artifact file(s)".into())
                    )
                        .await
                        .map_err(|err| {
                            panic!("Error updating publication status: {}", err.to_string())
                        }).unwrap();

                    // Update publication status to PublishingArtifact
                    self.artifact_service.change_publication_status_by_publication_id(
                        publication_id.clone(),
                        ArtifactPublicationStatus::PublishingArtifact,
                        Some("Started publishing artifact".into())
                    )
                        .await
                        .map_err(|err| {
                            panic!("Error updating publication status: {}", err.to_string())
                        }).unwrap();
                    
                    // Publish the model files to the target platform
                    match client.publish_model(&extracted_artifact_path, &artifact, maybe_metadata.as_ref(), &client_request).await {
                        Ok(_) => {            
                            // Update publication status to PublishedArtifact
                            self.artifact_service.change_publication_status_by_publication_id(
                                publication_id.clone(),
                                ArtifactPublicationStatus::PublishedArtifact,
                                Some("Successfully published artifact".into())
                            )
                                .await
                                .map_err(|err| {
                                    panic!("Error updating publication status: {}", err.to_string())
                                }).unwrap();
                        },
                        // Do nothing if getting an unimplemented error. This is because
                        // we have already guaranteed that either there is a publish model
                        // client, or a publish model metadata client and a platform client
                        // only needs to implement one of those.
                        Err(ClientError::Unimplemented)  => {
                            println!("Should be unreachable");
                        },
                        // All other errors are considered failure conditions. Handle them
                        // accordingly
                        Err(err) => {
                            println!("Failed: {}", err.to_string());
                            self.artifact_service.change_publication_status_by_publication_id(
                                publication_id.clone(),
                                ArtifactPublicationStatus::Failed,
                                Some(err.to_string())
                            )
                                .await
                                .map_err(|err| {
                                    panic!("Error updating publication status: {}", err.to_string())
                                }).unwrap();

                            eprintln!("{}", err.to_string());
                            nack(&channel, &deliver, None, None).await;
                            return;
                        }
                    };

                    // Clean up the extracted_artifact_path
                    std::fs::remove_dir_all(&extracted_artifact_path)
                        .expect(format!("Error cleaning up extracted artifact at path {}", &extracted_artifact_path.to_string_lossy().to_string()).as_str());
                }

                // Publish the model metadata to the target platform
                if let Some(client) = maybe_publish_metadata_client {
                    // Update publication status to PublishingMetadata
                    self.artifact_service.change_publication_status_by_publication_id(
                        publication_id.clone(),
                        ArtifactPublicationStatus::PublishingMetadata,
                        Some("Artifact published successfully".into())
                    )
                        .await
                        .map_err(|err| {
                            panic!("Error updating publication status: {}", err.to_string())
                        }).unwrap();
                    
                    let metadata = match maybe_metadata {
                        Some(m) => m,
                        None => {
                            eprintln!("Cannot publish metadata without metadata (:");
                            nack(&channel, &deliver, None, None).await;
                            return;
                        }
                    };

                    // Publish the model files to the target platform
                    match client.publish_model_metadata(&metadata, &client_request).await {
                        Ok(_) => {
                            // Update publication status to PublishedMetadata
                            self.artifact_service.change_publication_status_by_publication_id(
                                publication_id.clone(),
                                ArtifactPublicationStatus::PublishedMetadata,
                                Some("Metadata published successfully".into())
                            )
                                .await
                                .map_err(|err| {
                                    panic!("Error updating publication status: {}", err.to_string())
                                }).unwrap();
                        },
                        // Do nothing if getting an unimplemented error. This is because
                        // we have already guaranteed that either there is a publish model
                        // client, or a publish model metadata client and a platform client
                        // only needs to implement one of those.
                        Err(ClientError::Unimplemented)  => {},
                        // All other errors are considered failure conditions. Handle them
                        // accordingly
                        Err(err) => {
                            self.artifact_service.change_publication_status_by_publication_id(
                                publication_id.clone(),
                                ArtifactPublicationStatus::Failed,
                                Some(err.to_string())
                            )
                                .await
                                .map_err(|err| {
                                    panic!("Error updating publication status: {}", err.to_string())
                                }).unwrap();

                            eprintln!("{}", err.to_string());
                            nack(&channel, &deliver, None, None).await;
                            return;
                        }
                    };
                }
            },
            // Publish the dataset
            ArtifactType::Dataset => {
                // let client = ClientProvider::provide_publish_dataset_client(&request.platform)
                //     .map_err(|err| {
                //         eprintln!("{}", err);
                //     });
                eprintln!("Artifact publication not yet available for datasets");
                nack(&channel, &deliver, None, None).await;
                return 
            }
        };

        // Update publication status to Finsihed
        self.artifact_service.change_publication_status_by_publication_id(
            publication_id.clone(),
            ArtifactPublicationStatus::Finished,
            Some("Successfully published".into())
        )
            .await
            .map_err(|err| {
                panic!("Error updating publication status: {}", err.to_string())
            }).unwrap();

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
    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");
    let client = initialize_client(ClientParams{
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set"),
        replica_set: Some(env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set")),
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
    
    // Create connection and open channel
    let (_channel,  channel) = open_channel(
        broker_host,
        broker_port.parse::<u16>().expect("u16 parsed from 'port' String"),
        broker_username,
        broker_password,
    )
        .await
        .map_err(|err| { error!("{}", err.to_string()) })
        .expect("Connection to message broker created and channel opened");

    // Declare exchanges
    declare_exchanges(
        &channel, 
        vec![
            (ARTIFACT_PUBLICATION_EXCHANGE, ExchangeType::Topic),
        ]
    ).await
        .map_err(|err| { error!("{}", err.to_string())})
        .expect(format!("Exchanges {} to be declared", ARTIFACT_PUBLICATION_EXCHANGE).as_str());
    
    // Declare queue
    let _ = match channel.queue_declare(QueueDeclareArguments::new(ARTIFACT_PUBLICATION_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to delcare queue: {}", err.to_string())
    };
    
    // Bind queue to exchange
    match channel.queue_bind(
    QueueBindArguments::new(
            ARTIFACT_PUBLICATION_QUEUE,
            ARTIFACT_PUBLICATION_EXCHANGE, 
            ARTIFACT_PUBLICATION_ROUTING_KEY
        )
    ).await {
        Ok(_) => {},
        Err(err) => panic!("Failed to bind queue: {}", err.to_string())
    };

    // Unique consumer tag. Make this unique per worker. 
    let consumer_tag = Uuid::now_v7();

    let consumer = ArtifactPublisherConsumer {
        artifact_service: artifact_service_factory(&client, db_name.clone(), Arc::new(channel.clone())),
        publications_work_dir: PathBuf::from(&environment.shared_data_dir).join(ARTIFACT_PUBLICATION_DIR_NAME),
        // artifacts_cache_dir: PathBuf::from(&environment.artifacts_cache_dir)
    };
     
    let args = BasicConsumeArguments::default()
        .queue(ARTIFACT_PUBLICATION_QUEUE.into())
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
