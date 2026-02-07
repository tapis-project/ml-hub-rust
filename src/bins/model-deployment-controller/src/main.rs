use std::sync::Arc;
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
use serde_json::Value;
use tokio;
use uuid::Uuid;
use shared::{application::ports::events::{Event, EventPublisher, Kind, EventMetadata}, infra::messaging::{rabbitmq::exchanges::MODEL_DEPLOYMENT_EXCHANGE}};
use shared::infra::messaging::rabbitmq::queues::MODEL_DEPLOYMENT_QUEUE;
use shared::infra::messaging::rabbitmq::routing::MODEL_DEPLOYMENT_ROUTING_KEY;
use shared::infra::messaging::codec::deserialize_event_message;
use shared::infra::messaging::messages::{EventMessageEnvelope, ModelDeploymentStateDriftDetectedPayload};
use shared::infra::system::Env;
use async_trait::async_trait;
use std::env;
use model_deployment_controller::bootstrap::model_deployment_controller_factory;
use model_deployment_controller::database::{get_db, ClientParams};
use shared::application::services::model_deployment_controller::ModelDeploymentController;
use log::error;


struct ModelDeploymentControllerConsumer {
    event_publisher: Arc<dyn EventPublisher>
}

#[async_trait]
impl AsyncConsumer for ModelDeploymentControllerConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, _basic_properties: BasicProperties, content: Vec<u8>) {
        let event = match deserialize_event_message(content) {
            Ok(e) => e,
            Err(err) => {
                // TODO nack
                return
            }
        };

        match event {
            Event::ModelDeploymentStateDriftDetected { metadata, payload } => {
                controller.dispatch_reconciler(
                    &payload.deployment_id,
                    &payload.deployment_revision,
                    &payload.actual_state,
                    &payload.desired_state,
                ).await;
            }
            _ => {
                // TODO publish to dead letter queue
                // TODO nack
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
    let _ = match channel.queue_declare(QueueDeclareArguments::new(MODEL_DEPLOYMENT_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to create channel: {}", err.to_string())
    };

    match channel.exchange_declare(
        ExchangeDeclareArguments::new(
            MODEL_DEPLOYMENT_EXCHANGE,
            ExchangeType::Topic.to_string().as_str()
        )
    ).await {
        Ok(_) => {},
        Err(err) => panic!("Failed to delare exchange: {}", err.to_string())
    };
    
     match channel.queue_bind(
        QueueBindArguments::new(
            MODEL_DEPLOYMENT_QUEUE,
            MODEL_DEPLOYMENT_EXCHANGE, 
            MODEL_DEPLOYMENT_ROUTING_KEY
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

    let consumer = ModelDeploymentControllerConsumer {
        event_publisher:
    };
     
    let args = BasicConsumeArguments::default()
        .queue(MODEL_DEPLOYMENT_QUEUE.into())
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
