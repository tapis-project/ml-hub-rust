//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use std::{env, sync::Arc};

use amqprs::{
    channel::{Channel, ExchangeType, QueueBindArguments, QueueDeclareArguments},
    connection::Connection,
    FieldTable, FieldValue,
};
use log::{debug, error, info};
use mongodb::Client;
use shared::application::ports::cipher::Cipher;
use shared::application::ports::deployment::ModelDeploymentReconcilerProvider;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::deployment_argument::DeploymentArgumentRepository;
use shared::application::ports::deployment_option::DeploymentOptionRepository;
use shared::application::ports::endpoint::EndpointRepository;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::hpc_cluster::HpcClusterRepository;
use shared::application::ports::model::ExternalModelRepository;
use shared::application::services::deployment_argument_service::DeploymentArgumentService;
use shared::application::services::endpoint_issuance_service::EndpointIssuanceService;
use shared::application::services::model_deployment_observation_service::ModelDeploymentObservationService;
use shared::application::services::model_deployment_reconciliation_service::ModelDeploymentReconciliationService;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::domain::entities::site::SiteContext;
use shared::infra::_common::mongo::{initialize_client, ClientParams};
use shared::infra::argument::mongo::MongoDeploymentArgumentRepository;
use shared::infra::configuration::site_configuration_loader::SiteConfigurationLoader;
use shared::infra::encryption::vault::VaultCipher;
use shared::infra::messaging::rabbitmq::connection::open_channel;
use shared::infra::messaging::rabbitmq::exchanges::{
    declare_exchanges, DEAD_LETTER_EXCHANGE, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE,
};
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;
use shared::infra::messaging::rabbitmq::queues::{
    DEAD_LETTER_QUEUE, MODEL_DEPLOYMENT_RECONCILIATION_QUEUE,
};
use shared::infra::messaging::rabbitmq::routing::{
    DEAD_LETTER_ROUTING_KEY, MODEL_DEPLOYMENT_RECONCILIATION_ROUTING_KEY,
};
use shared::infra::persistence::mongo::repositories::{
    DeploymentOptionRepository as MongoDeploymentOptionRepository,
    EndpointRepository as MongoEndpointRepository,
    ExternalModelRepository as MongoExternalModelRepository,
    HpcClusterRepository as MongoHpcClusterRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
};
use shared::infra::reconciliation::client_provider::ReconciliationClientProvider;

use crate::controller::{
    ControlLoopState, ModelDeploymentController, ModelDeploymentControllerInitializationError,
    ReconciliationRetryConfiguration,
};

pub struct MessagingContext {
    _connection: Connection,
    pub channel: Arc<Channel>,
}

pub struct BootstrapOutput {
    pub messaging_context: MessagingContext,
    pub client: Client,
    pub db_name: String,
    pub site_context: SiteContext,
}

pub async fn bootstrap() -> BootstrapOutput {
    info!("Bootstrapping model deployment controller infrastructure");

    let broker_host = env::var("RABBIT_HOST").expect("RABBIT_HOST to be in environment variables");

    let broker_port = env::var("RABBIT_PORT").expect("RABBIT_PORT to be in environment variables");

    let broker_username =
        env::var("RABBIT_USER").expect("RABBIT_USER to be in environment variables");

    let broker_password =
        env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD to be in environment variables");

    debug!(
        "Opening model deployment controller RabbitMQ channel; host='{}'; port='{}'",
        broker_host, broker_port
    );

    let (_connection, channel) = open_channel(
        broker_host,
        broker_port
            .parse::<u16>()
            .expect("u16 parsed from RABBIT_PORT"),
        broker_username,
        broker_password,
    )
    .await
    .map_err(|err| error!("{}", err))
    .expect("Connection to message broker established and channel created");

    let messaging_context = MessagingContext {
        _connection,
        channel: Arc::new(channel),
    };

    info!("Model deployment controller RabbitMQ channel opened");

    debug!("Declaring model deployment controller RabbitMQ topology");

    declare_exchanges(
        &messaging_context.channel,
        vec![(DEAD_LETTER_EXCHANGE, ExchangeType::Direct)],
    )
    .await
    .expect("Exchanges to be declared");

    messaging_context
        .channel
        .queue_declare(QueueDeclareArguments::new(DEAD_LETTER_QUEUE.into()))
        .await
        .unwrap_or_else(|err| panic!("Failed to declare dead letter queue: {}", err));

    messaging_context
        .channel
        .queue_bind(QueueBindArguments::new(
            DEAD_LETTER_QUEUE,
            DEAD_LETTER_EXCHANGE,
            DEAD_LETTER_ROUTING_KEY,
        ))
        .await
        .unwrap_or_else(|err| {
            panic!(
                "Failed to bind dead letter queue with routing key {}: {}",
                DEAD_LETTER_ROUTING_KEY, err
            )
        });

    let mut dead_letter_arguments = FieldTable::new();

    dead_letter_arguments.insert(
        String::from("x-dead-letter-exchange")
            .try_into()
            .expect("x-dead-letter-exchange to be a valid ShortStr"),
        FieldValue::from(DEAD_LETTER_EXCHANGE),
    );
    dead_letter_arguments.insert(
        String::from("x-dead-letter-routing-key")
            .try_into()
            .expect("x-dead-letter-routing-key to be a valid ShortStr"),
        FieldValue::from(DEAD_LETTER_ROUTING_KEY),
    );

    let mut queue_declare_arguments =
        QueueDeclareArguments::new(MODEL_DEPLOYMENT_RECONCILIATION_QUEUE.into());

    queue_declare_arguments.arguments(dead_letter_arguments);

    messaging_context
        .channel
        .queue_declare(queue_declare_arguments)
        .await
        .expect("Model deployment reconciliation queue to be declared");

    declare_exchanges(
        &messaging_context.channel,
        vec![(
            MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE,
            ExchangeType::Topic,
        )],
    )
    .await
    .expect("Model deployment reconciliation exchange to be declared");

    messaging_context
        .channel
        .queue_bind(QueueBindArguments::new(
            MODEL_DEPLOYMENT_RECONCILIATION_QUEUE,
            MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE,
            MODEL_DEPLOYMENT_RECONCILIATION_ROUTING_KEY,
        ))
        .await
        .expect("Model deployment reconciliation queue to be bound");

    info!("Model deployment controller RabbitMQ topology is ready");

    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");

    let mongo_host = env::var("MONGO_HOST").expect("MONGO_HOST env var not set");

    let mongo_port = env::var("MONGO_PORT").expect("MONGO_PORT env var not set");

    let mongo_replica_set =
        env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set");

    debug!(
        "Initializing model deployment controller Mongo client; host='{}'; port='{}'; database='{}'; replica_set='{}'",
        mongo_host, mongo_port, db_name, mongo_replica_set
    );

    let client = initialize_client(ClientParams {
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: mongo_host,
        port: mongo_port,
        db: db_name.clone(),
        replica_set: Some(mongo_replica_set),
    })
    .await
    .unwrap_or_else(|err| panic!("Database initialization error: {}", err));

    info!("Model deployment controller Mongo client initialized");

    debug!("Loading model deployment controller site configuration");

    let config_loader = SiteConfigurationLoader::new()
        .unwrap_or_else(|err| panic!("Site configuration initialization error: {}", err));

    let config = config_loader.get_config();

    let site_context = SiteContext {
        base_url: config.base_url.clone(),
        site_id: config.site_id.clone(),
    };

    info!(
        "Model deployment controller site configuration loaded; site_id='{}'; base_url='{}'",
        site_context.site_id, site_context.base_url
    );

    BootstrapOutput {
        messaging_context,
        client,
        db_name,
        site_context,
    }
}

pub async fn model_deployment_controller_builder(
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
    site_context: SiteContext,
) -> Result<ModelDeploymentController, ModelDeploymentControllerInitializationError> {
    debug!("Building model deployment controller application services");

    let model_deployment_service = Arc::new(model_deployment_service_builder(
        client,
        db_name.clone(),
        channel.clone(),
    ));

    let reconciliation_service = model_deployment_reconciliation_service_builder(
        site_context.clone(),
        client,
        db_name.clone(),
        channel.clone(),
    );

    let observer_service =
        model_deployment_observer_service_builder(site_context, client, db_name.clone(), channel);

    let endpoint_issuance_service = Arc::new(EndpointIssuanceService::new(endpoint_repo_factory(
        client, db_name,
    )));

    let state = Arc::new(ControlLoopState::new());

    let retry_configuration = ReconciliationRetryConfiguration::from_environment();

    debug!("Model deployment controller application services built");

    ModelDeploymentController::new(
        model_deployment_service,
        reconciliation_service,
        observer_service,
        endpoint_issuance_service,
        state,
        retry_configuration,
    )
    .await
}

pub fn external_model_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ExternalModelRepository> {
    Arc::new(MongoExternalModelRepository::new(client, db_name))
}

pub fn model_deployment_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ModelDeploymentRepository> {
    Arc::new(MongoModelDeploymentRepository::new(client, db_name))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn model_deployment_reconciler_provider_factory() -> Arc<dyn ModelDeploymentReconcilerProvider>
{
    Arc::new(ReconciliationClientProvider::new())
}

pub fn deployment_option_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn DeploymentOptionRepository> {
    Arc::new(MongoDeploymentOptionRepository::new(client, db_name))
}

pub fn hpc_cluster_repo_factory(client: &Client, db_name: String) -> Arc<dyn HpcClusterRepository> {
    Arc::new(MongoHpcClusterRepository::new(client, db_name))
}

pub fn endpoint_repo_factory(client: &Client, db_name: String) -> Arc<dyn EndpointRepository> {
    Arc::new(MongoEndpointRepository::new(client, db_name))
}

pub fn deployment_argument_repo_factory(
    client: &Client,
    db_name: &str,
) -> Arc<dyn DeploymentArgumentRepository> {
    Arc::new(MongoDeploymentArgumentRepository::new(client, db_name))
}

pub fn cipher_factory() -> Arc<dyn Cipher> {
    Arc::new(VaultCipher {})
}

pub fn deployment_argument_service_builder(
    client: &Client,
    db_name: &str,
) -> DeploymentArgumentService {
    DeploymentArgumentService::new(
        deployment_argument_repo_factory(client, db_name),
        cipher_factory(),
    )
}

pub fn model_deployment_service_builder(
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
) -> ModelDeploymentService {
    ModelDeploymentService::new(
        deployment_argument_service_builder(client, &db_name),
        model_deployment_repo_factory(client, db_name.clone()),
        deployment_option_repo_factory(client, db_name.clone()),
        external_model_repo_factory(client, db_name.clone()),
        hpc_cluster_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel),
    )
}

pub fn model_deployment_reconciliation_service_builder(
    site_context: SiteContext,
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
) -> Arc<ModelDeploymentReconciliationService> {
    Arc::new(ModelDeploymentReconciliationService::new(
        site_context,
        deployment_argument_service_builder(client, &db_name),
        model_deployment_service_builder(client, db_name.clone(), channel.clone()),
        external_model_repo_factory(client, db_name.clone()),
        model_deployment_reconciler_provider_factory(),
    ))
}

pub fn model_deployment_observer_service_builder(
    site_context: SiteContext,
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
) -> Arc<ModelDeploymentObservationService> {
    Arc::new(ModelDeploymentObservationService::new(
        site_context,
        model_deployment_service_builder(client, db_name.clone(), channel.clone()),
        model_deployment_reconciler_provider_factory(),
    ))
}
