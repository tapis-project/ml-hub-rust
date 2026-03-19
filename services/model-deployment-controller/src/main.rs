use std::sync::Arc;
use amqprs::{
    channel::{
        BasicConsumeArguments,
        Channel,
        ExchangeType,
        QueueBindArguments,
        QueueDeclareArguments
    },
    connection::Connection,
    consumer::AsyncConsumer,
    BasicProperties,
    Deliver,
    FieldTable,
    FieldValue
};
use tokio;
use uuid::Uuid;
use shared::{
    application::{
        ports::events::Event,
        services::model_deployment_controller::{
            FinishReconciliationError,
            ModelDeploymentController,
            ReconciliationDispatchError
        },
        workflows::reconciliation::ReconciliationError
    },
    infra::messaging::rabbitmq::{connection::open_channel,
        exchanges::{DEAD_LETTER_EXCHANGE, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE},
        queues::DEAD_LETTER_QUEUE
    }
};
use shared::infra::messaging::rabbitmq::queues::MODEL_DEPLOYMENT_RECONCILIATION_QUEUE;
use shared::infra::messaging::rabbitmq::routing::{MODEL_DEPLOYMENT_RECONCILIATION_ROUTING_KEY, DEAD_LETTER_ROUTING_KEY};
use shared::infra::messaging::rabbitmq::settlement::{ack, nack};
use shared::infra::messaging::rabbitmq::exchanges::declare_exchanges;
use shared::infra::messaging::codec::deserialize_event_message;
use async_trait::async_trait;
use std::env;
use model_deployment_controller::bootstrap::{build_deployment_strategy_provider, model_deployment_conroller_builder};
use model_deployment_controller::database::{get_db, ClientParams};
use log::{error, info, warn};


struct MessagingContext {
    _connection: Connection,
    channel: Arc<Channel>,
}

struct ModelDeploymentControllerConsumer {
    controller: Arc<ModelDeploymentController>,
}

impl ModelDeploymentControllerConsumer {
    // Acknowledges the message and kills the process if unable
    async fn ack(&self, channel: &Channel, deliver: &Deliver, message_id: &String) {
        if let Err(err) = ack(&channel, &deliver, None).await {
            error!("Failed to ack message_id={}: Error: {}. Shutting down...", message_id, err.to_string());
            std::process::exit(1);
        }
    }

    // Negatively acknowledges the message and kills the process if unable
    async fn nack(&self, channel: &Channel, deliver: &Deliver, requeue: bool, message_id: &String) {
        if let Err(err) = nack(&channel, &deliver, Some(requeue), None).await {
            error!("Failed to nack message_id={}: Error: {}. Shutting down...", message_id, err.to_string());
            std::process::exit(1);
        }
    }
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
                self.nack(&channel, &deliver, false, message_id).await;

                return
            }
        };

        let maybe_outcome = match &event {
            Event::ModelDeploymentStateDriftDetected { payload, .. } => {
                self.controller.dispatch_reconciler(payload).await
            }
            _ => {
                error!("Invalid event type for this consumer: {}", String::from(event.metadata().kind()));
                self.nack(&channel, &deliver, false, message_id).await;

                return
            }
        };

        let mut dispatch_result = match maybe_outcome {
            Ok(result) => result,
            Err(err) => {
                match err {
                    ReconciliationDispatchError::ModelDeploymentDomainInvariantViolation(e) => {
                        error!("ModelDeploymentDomainInvariantViolation: {}", e.to_string());
                        self.nack(&channel, &deliver, false, message_id).await;
                        
                        return
                    },
                    ReconciliationDispatchError::MissingDeploymentStrategy(e) => {
                        error!("MissingDeploymentStrategy: {}", e.to_string());
                        self.nack(&channel, &deliver, false, message_id).await;
        
                        return
                    },
                    ReconciliationDispatchError::ModelDeploymentRetrievalFailed(e) => {
                        error!("ModelDeploymentRetrievalFailed: {}", e.to_string());
                        self.nack(&channel, &deliver, true, message_id).await;
        
                        return
                    },
                    ReconciliationDispatchError::ModelMetadataRetrievalFailed(e) => {
                        error!("ModelMetadataRetrievalFailed: {}", e.to_string());
                        self.nack(&channel, &deliver, true, message_id).await;
        
                        return
                    },
                    ReconciliationDispatchError::StaleEvent(e) => {
                        warn!("StaleEvent: {}", e.to_string());
                        self.ack(&channel, &deliver, message_id).await;
        
                        return
                    },
                    ReconciliationDispatchError::ReconciliationClientInitilizationFailed(e) => {
                        // Event was processible but client was incorrectly configured. Once the client
                        // is reconfigured, this event can be processed again
                        error!("ReconciliationClientInitilizationFailed: {}", e.to_string());
                        self.nack(&channel, &deliver, true, message_id).await;
        
                        return
                    },
                    ReconciliationDispatchError::ReconciliationFailed(e) => {
                        error!("ReconciliationFailed: {}", e.to_string());
                        match e {
                            ReconciliationError::Unimplemented(_) => {
                                self.nack(&channel, &deliver, false, message_id).await;
                            }
                        }
        
                        return
                    },
                };
            }
        };

        // Relate the event that caused this reconciliation to the events caused
        // by this reconciliation
        dispatch_result.correlate_event(event.clone());

        let maybe_finish_result = self.controller.finish_reconiliation(dispatch_result).await;

        match maybe_finish_result {
            Ok(_) => {
                // Successfully processed. Acknowledge
                self.ack(&channel, &deliver, message_id).await;
            },
            Err(err) => {
                match err {
                    FinishReconciliationError::ModelDeploymentUpdateFailed(e) => {
                        error!("ModelDeploymentUpdateFailed: {}", e.to_string());
                        self.nack(&channel, &deliver, false, message_id).await;
        
                        return
                    }
                    FinishReconciliationError::EventPublicationFailed(e) => {
                        error!("EventPublicationFailed: {}", e.to_string());
                        self.nack(&channel, &deliver, false, message_id).await;
        
                        return
                    },
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> () {
    env_logger::init();

    let broker_host = std::env::var("RABBIT_HOST").expect("RABBIT_URL to be in environment variables");
    let broker_port = std::env::var("RABBIT_PORT").expect("RABBIT_PORT to be in environment variables");
    let broker_username = std::env::var("RABBIT_USER").expect("RABBIT_USER to be in environment variables");
    let broker_password = std::env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD to be in environment variables");
    
    let (_connection, channel) = open_channel(
        broker_host,
        broker_port.parse::<u16>().expect("u16 parsed from 'port' String"),
        broker_username,
        broker_password,
    )
        .await
        .map_err(|err| { error!("{}", err.to_string()) })
        .expect("Connection to message broker established and channel created");
    
    // We keep the connection in the MessageContext because even though we never
    // use it, if we drop it from memory, the connection will also drop.
    let context = MessagingContext {
        _connection,
        channel: Arc::new(channel)
    };

    // Declare dlx
    declare_exchanges(&context.channel, vec![(DEAD_LETTER_EXCHANGE, ExchangeType::Direct)])
        .await
        .expect("Exchanges to be declared");

    // Declare dlq
    let _ = match context.channel.queue_declare(QueueDeclareArguments::new(DEAD_LETTER_QUEUE.into())).await {
        Ok(q) => q,
        Err(err) => panic!("Failed to declare dead letter queue: {}", err.to_string())
    };

    // Bind dlq to dlx
    context.channel.queue_bind(QueueBindArguments::new(DEAD_LETTER_QUEUE, DEAD_LETTER_EXCHANGE, DEAD_LETTER_ROUTING_KEY))
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

    let _ = context.channel.queue_declare(queue_declare_args.clone())
        .await
        .expect("Model deployment reconciliation queue to be declared");

    declare_exchanges(
        &context.channel,
        vec![
            (MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, ExchangeType::Topic),
            (DEAD_LETTER_EXCHANGE, ExchangeType::Direct),
        ]
    )
        .await
        .expect("Model deployment reconciliation and dead leater exchanges to be declared");
    
    context.channel.queue_bind(
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

    let deployment_strategy_provider = build_deployment_strategy_provider();

    let client_strategy_sets = match deployment_strategy_provider {
        Ok(p) => Arc::new(p.provide().clone()),
        Err(err) => {
            warn!("Error initializing deployment strategy provider: {}", err.to_string());
            Arc::new(vec![])
        }
    };
    
    let consumer = ModelDeploymentControllerConsumer {
        controller: model_deployment_conroller_builder(&db, context.channel.clone(), client_strategy_sets),
    };
     
    let args = BasicConsumeArguments::default()
        .queue(MODEL_DEPLOYMENT_RECONCILIATION_QUEUE.into())
        .consumer_tag(consumer_tag.to_string())
        .finish();

    match context.channel.basic_consume(consumer, args).await {
        Ok(_) => { info!("Ready to recieve messages...") },
        Err(err) => panic!("Failed to consume: {}", err.to_string())
    };

    // Block forever or until terminated
    if let Err(err) = tokio::signal::ctrl_c().await {
        panic!("{}", err.to_string())
    }
}
