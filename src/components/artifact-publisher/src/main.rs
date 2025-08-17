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
use shared::domain::entities::artifact_publication::{ArtifactPublicationFailureReason, ArtifactPublicationStatus};
use shared::domain::entities::artifact::ArtifactType;
use shared::constants::{ARTIFACT_PUBLICATION_EXCHANGE, ARTIFACT_PUBLICATION_QUEUE, ARTIFACT_PUBLICATION_ROUTING_KEY};
use shared::presentation::http::v1::dto::artifacts::PublishArtifactRequest;
use shared::infra::system::Env;
use shared::constants::ARTIFACT_PUBLICATION_DIR_NAME;
// use shared::datasets::presentation::http::v1::dto::IngestDatasetRequest;
use shared::infra::messaging::messages::PublishArtifactMessage;
use async_trait::async_trait;
use shared::application::services::artifact_service::ArtifactService;
use std::env;
use artifact_publisher::bootstrap::artifact_service_factory;
use artifact_publisher::database::{get_db, ClientParams};
use shared::infra::fs::archiver::Archiver;
use clients::{ClientError, PublishModelClient, PublishModelMetadataClient};

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
        let client_request: PublishArtifactRequest = serde_json::from_slice(&request.serialized_client_request)
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
            publication.artifact_id.clone().to_string()
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
                let metadata = self.artifact_service.find_metadata_by_artifact_id(
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
                    
                    // Publish the model files to the target platform
                    match client.publish_model(&extracted_artifact_path, &artifact, &metadata, &client_request).await {
                        Ok(_) => {                
                            // Update publication status to PublishedArtifact
                            self.artifact_service.change_publication_status_by_publication_id(
                                publication_id.clone(),
                                ArtifactPublicationStatus::PublishedArtifact,
                                Some("Extracting artifact files".into())
                            )
                                .await
                                .map_err(|err| {
                                    panic!("Error updating publication status: {}", err.to_string())
                                }).unwrap();

                            // Clean up the extracted_artifact_path
                            std::fs::remove_dir_all(&extracted_artifact_path)
                                .expect(format!("Error cleaning up extracted artifact at path {}", &extracted_artifact_path.to_string_lossy().to_string()).as_str());
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
                                ArtifactPublicationStatus::Failed(ArtifactPublicationFailureReason::FailedToPublishArtifact(err.to_string())),
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

                // Publish the model metadata to the target platform
                if let Some(client) = maybe_publish_metadata_client {
                    // Update publication status to PublishingMetata
                    self.artifact_service.change_publication_status_by_publication_id(
                        publication_id.clone(),
                        ArtifactPublicationStatus::PublishingMetadata,
                        Some("Artifact published successfully".into())
                    )
                        .await
                        .map_err(|err| {
                            panic!("Error updating publication status: {}", err.to_string())
                        }).unwrap();
                    
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
                                ArtifactPublicationStatus::Failed(ArtifactPublicationFailureReason::FailedToPublishArtifact(err.to_string())),
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
    let _ = match channel.queue_declare(QueueDeclareArguments::new(ARTIFACT_PUBLICATION_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to create channel: {}", err.to_string())
    };

    match channel.exchange_declare(
        ExchangeDeclareArguments::new(
            ARTIFACT_PUBLICATION_EXCHANGE,
            ExchangeType::Topic.to_string().as_str()
        )
    ).await {
        Ok(_) => {},
        Err(err) => panic!("Failed to delare exchange: {}", err.to_string())
    };
    
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

    let consumer = ArtifactPublisherConsumer {
        artifact_service: artifact_service_factory(&db).expect("failed to initialize artifact service"),
        publications_work_dir: PathBuf::from(&environment.shared_data_dir).join(ARTIFACT_PUBLICATION_DIR_NAME),
        // artifacts_cache_dir: PathBuf::from(&environment.artifacts_cache_dir)
    };
     
    let args = BasicConsumeArguments::default()
        .queue(ARTIFACT_PUBLICATION_QUEUE.into())
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

