use std::sync::Arc;
use amqprs::{
    channel::{
        BasicConsumeArguments, Channel, ExchangeType, QueueBindArguments, QueueDeclareArguments
    }, connection::OpenConnectionArguments, consumer::AsyncConsumer, BasicProperties, Deliver, FieldTable, FieldValue,
};
use tokio;
use uuid::Uuid;
use shared::{
    application::{ports::events::{Event, EventPublisher}, services::model_deployment_controller::{ModelDeploymentController, ModelDeploymentControllerError}},
    infra::messaging::rabbitmq::{connection::open_channel, exchanges::{DEAD_LETTER_EXCHANGE, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE}, queues::DEAD_LETTER_QUEUE}};
use shared::infra::messaging::rabbitmq::queues::MODEL_DEPLOYMENT_RECONCILIATION_QUEUE;
use shared::infra::messaging::rabbitmq::routing::{MODEL_DEPLOYMENT_RECONCILIATION_ROUTING_KEY, DEAD_LETTER_ROUTING_KEY};
use shared::infra::messaging::rabbitmq::settlement::{ack, nack};
use shared::infra::messaging::rabbitmq::exchanges::delcare_exchanges;
use shared::infra::messaging::codec::deserialize_event_message;
use async_trait::async_trait;
use std::env;
use model_deployment_controller::bootstrap::{model_deployment_conroller_builder, event_publisher_factory};
use model_deployment_controller::database::{get_db, ClientParams};
use log::{error, info, warn};


struct ModelDeploymentControllerConsumer {
    event_publisher: Arc<dyn EventPublisher>,
    controller: Arc<ModelDeploymentController>,
}

#[async_trait]
impl AsyncConsumer for ModelDeploymentControllerConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, properties: BasicProperties, content: Vec<u8>) {
        let default_message_id = String::from("unknown");
        let message_id = properties.message_id().unwrap_or(&default_message_id);

        let event = match deserialize_event_message(content) {
            Ok(e) => e,
            Err(err) => {
                error!("Failed to desireialize message: Message id={}; Error: {}", message_id, err.to_string());
                let _ = nack(&channel, &deliver, Some(false), None)
                    .await
                    .map_err(|err| {
                        panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                    });

                return
            }
        };

        let maybe_dispatch_reconciliation_outcome = match &event {
            Event::ModelDeploymentStateDriftDetected { payload, .. } => {
                self.controller.dispatch_reconciler(payload).await
            }
            _ => {
                error!("Invalid event type for this consumer: {}", String::from(event.metadata().kind()));
                let _ = nack(&channel, &deliver, Some(false), None)
                    .await
                    .map_err(|err| {
                        panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                    });

                return
            }
        };

        let event_payloads = match maybe_dispatch_reconciliation_outcome {
            Ok(o) => o.events,
            Err(err) => {
                match err {
                    ModelDeploymentControllerError::ModelDeploymentDomainInvariantViolation(e) => {
                        error!("{}", e.to_string());
                        let _ = nack(&channel, &deliver, Some(false), None).await
                            .map_err(|err| {
                                panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                            });
        
                        return
                    },
                    ModelDeploymentControllerError::ModelDeploymentRetrievalFailed(e) => {
                        error!("{}", e.to_string());
                        let _ = nack(&channel, &deliver, Some(true), None).await
                            .map_err(|err| {
                                panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                            });
        
                        return
                    },
                    ModelDeploymentControllerError::ModelMetadataRetrievalFailed(e) => {
                        error!("{}", e.to_string());
                        let _ = nack(&channel, &deliver, Some(true), None)
                            .await
                            .map_err(|err| {
                                panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                            });
        
                        return
                    },
                    ModelDeploymentControllerError::StaleEvent(e) => {
                        warn!("{}", e.to_string());
                        let _ = ack(&channel, &deliver, None)
                            .await
                            .map_err(|err| {
                                panic!("Failed to ack message_id={}: Error: {}", message_id, err.to_string())
                            });
        
                        return
                    },
                    ModelDeploymentControllerError::ReconciliationClientInitilizationFailed(e) => {
                        // Event was processible but client was incorrectly configured. Once the client
                        // is reconfigured, this event can be processed again
                        error!("{}", e.to_string());
                        let _ = nack(&channel, &deliver, Some(true), None)
                            .await
                            .map_err(|err| {
                                panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                            });
        
                        return
                    },
                    ModelDeploymentControllerError::ReconciliationFailed(e) => {
                        error!("{}", e.to_string());
                        let _ = nack(&channel, &deliver, Some(true), None)
                            .await
                            .map_err(|err| {
                                panic!("Failed to nack message_id={}: Error: {}", message_id, err.to_string())
                            });
        
                        return
                    },
                };
            }
        };


        // Successfully processed. Acknowledge
        let _ = ack(&channel, &deliver, None)
            .await
            .map_err(|err| {
                panic!("Failed to ack message_id={}: Error: {}", message_id, err.to_string())
            });

        // Publish any events returned by the controller
        for payload in event_payloads {
            let new_event = &Event::from_payload(&payload, Some(&event));
            let _ = self.event_publisher.publish(new_event)
                .await
                .map_err(|err| {
                    error!("Failed to publish event produced by reconciliation. Event id: {}, Event kind: {}. Error: {}", new_event.metadata().id().to_string(), String::from(new_event.metadata().kind()), err.to_string())
                });
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
    let maybe_channel = open_channel(connection_args).await;

    // Create a channel
    let channel = match maybe_channel {
        Ok(c) => c,
        Err(err) => panic!("Failed to open channel: {}", err.to_string())
    };

    // Declare dlx
    delcare_exchanges(&channel, vec![(DEAD_LETTER_EXCHANGE, ExchangeType::Direct.to_string().as_str())])
        .await
        .expect("Exchanges to be declared");

    // Declare dlq
    let _ = match channel.queue_declare(QueueDeclareArguments::new(DEAD_LETTER_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to declare dead letter queue: {}", err.to_string())
    };

    // Bind dlq to dlx
    channel.queue_bind(QueueBindArguments::new(DEAD_LETTER_QUEUE, DEAD_LETTER_EXCHANGE, DEAD_LETTER_ROUTING_KEY))
        .await
        .expect(format!("DLQ bound to DLX with routing key {}", DEAD_LETTER_ROUTING_KEY).as_str());

    // Declare main queue
    let mut dl_args = FieldTable::new();

    // When message is rejected, send to DLX
    dl_args.insert(
        String::from("x-dead-letter-exchange").try_into().expect("Should be ShortStr"),
        FieldValue::from(DEAD_LETTER_EXCHANGE)
    );

    // Routing key used when dead-lettering
    dl_args.insert(
        String::from("x-dead-letter-routing-key").try_into().expect("Should be ShortStr"),
        FieldValue::from(DEAD_LETTER_ROUTING_KEY)
    );
    
    let mut queue_declare_args = QueueDeclareArguments::new(MODEL_DEPLOYMENT_RECONCILIATION_QUEUE.into());
    
    queue_declare_args.arguments(dl_args);

    let _ = channel.queue_declare(queue_declare_args.clone())
        .await
        .expect("Model deployment reconciliation queue to be declared");

    delcare_exchanges(
        &channel,
        vec![
            (MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, ExchangeType::Topic.to_string().as_str()),
            (DEAD_LETTER_EXCHANGE, ExchangeType::Direct.to_string().as_str()),
        ]
    )
        .await
        .expect("Model deployment reconciliation and dead leater exchanges to be declared");
    
    channel.queue_bind(
        QueueBindArguments::new(
            MODEL_DEPLOYMENT_RECONCILIATION_QUEUE,
            MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, 
            MODEL_DEPLOYMENT_RECONCILIATION_ROUTING_KEY,
        )
    )
        .await
        .expect("Model deployment reconciliation queue bound to exchange with routing key");

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

    let consumer = ModelDeploymentControllerConsumer {
        event_publisher: event_publisher_factory(&host, &port, &username, &password),
        controller: model_deployment_conroller_builder(&db, &host, &port, &username, &password)
    };
     
    let args = BasicConsumeArguments::default()
        .queue(MODEL_DEPLOYMENT_RECONCILIATION_QUEUE.into())
        .consumer_tag(consumer_tag.to_string())
        .finish();

    match channel.basic_consume(consumer, args).await {
        Ok(_) => { info!("Ready to recieve messages...") },
        Err(err) => panic!("Failed to consume: {}", err.to_string())
    };

    // Block forever or until terminated
    if let Err(err) = tokio::signal::ctrl_c().await {
        panic!("{}", err.to_string())
    }
}
