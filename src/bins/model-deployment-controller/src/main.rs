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
use shared::domain::entities::artifact_publication::ArtifactPublicationStatus;
use shared::domain::entities::artifact::ArtifactType;
use shared::infra::messaging::rabbitmq::constants::{ARTIFACT_PUBLICATION_EXCHANGE, ARTIFACT_PUBLICATION_QUEUE, ARTIFACT_PUBLICATION_ROUTING_KEY};
use shared::presentation::http::v1::requests::artifacts::PublishArtifactServiceRequest;
use shared::infra::system::Env;
use shared::constants::ARTIFACT_PUBLICATION_DIR_NAME;
// use shared::datasets::presentation::http::v1::requests::IngestDatasetRequest;
use shared::infra::messaging::messages::PublishArtifactMessage;
use async_trait::async_trait;
use shared::application::services::artifact_service::ArtifactService;
use std::env;
use artifact_publisher::bootstrap::model_deployment_controller_factory;
use artifact_publisher::database::{get_db, ClientParams};
use shared::infra::fs::archiver::Archiver;
use shared::application::services::ModelDeploymentController;


struct ArtifactPublisherConsumer {
    artifact_service: ArtifactService,
    publications_work_dir: PathBuf,
}

#[async_trait]
impl AsyncConsumer for ArtifactPublisherConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, _basic_properties: BasicProperties, content: Vec<u8>) {
        // Deserialize the message
        let message: PublishArtifactMessage = match serde_json::from_slice(&content) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Deserialization error in consumer '{}': {}", &deliver.consumer_tag(), err.to_string());
                nack(&channel, &deliver, None, None).await;
                return;
            }
        };

        let event = ??

        let controller = model_deployment_controller_factory();

        match controller.handle(event) {
            Ok(_) => {
                // TODO Ack?
            },
            Err(_) => {
                // TODO Nack? Reject?
            }
        }
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
