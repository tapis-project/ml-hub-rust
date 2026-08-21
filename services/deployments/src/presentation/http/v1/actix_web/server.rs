use std::sync::Arc;
use crate::presentation;
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::{build_deployment_strategy_provider, model_deployment_service_builder};
use shared::application::services::deployment_strategy_service::DeploymentStrategyService;
pub use shared::infra::_common::mongo::{ClientParams, initialize_client};
use shared::presentation::http::v1::actix_web::middleware::preflight::preflight_short_circuit;
use crate::presentation::http::v1::actix_web::openapi::ApiDoc;
use actix_web::{App, HttpServer, web, middleware::{from_fn, Logger}};
use amqprs::channel::ExchangeType;
use shared::bootstrap::build_shared_app_context;
use shared::infra::configuration::site_configuration_loader::SiteConfigurationLoader;
use shared::presentation::http::v1::actix_web::middleware::{authentication::authenticate, tenancy::resolve_tenancy};
use shared::infra::messaging::rabbitmq::connection::open_channel;
use shared::infra::messaging::rabbitmq::exchanges::{declare_exchanges, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE};
use std::env;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use log::error;

pub async fn run_server() -> std::io::Result<()> {
    pub const DEFAULT_PORT: u16 = 8000;
    pub const DEFAULT_HOST: &str = "0.0.0.0";
    
    // Initialize the logger
    env_logger::init();

    // Set the address from env vars HOST and PORT, fallback to default values
    // if values for these env vars are not defined
    let addrs = (
        env::var("HOST").unwrap_or(DEFAULT_HOST.into()),
        env::var("PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT)
    );

    let broker_host = std::env::var("RABBIT_HOST").expect("RABBIT_URL missing from environment variables");
    let broker_port = std::env::var("RABBIT_PORT").expect("RABBIT_PORT missing from environment variables");
    let broker_username = std::env::var("RABBIT_USER").expect("RABBIT_USER missing from environment variables");
    let broker_password = std::env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD missing from environment variables");
    
    let (_connection, channel) = open_channel(
        broker_host,
        broker_port.parse::<u16>().expect("u16 parsed from 'port' String"),
        broker_username,
        broker_password,
    )
        .await
        .map_err(|e| { error!("{}", e.to_string()) })
        .expect("Connection to message broker established and channel created");

    declare_exchanges(&channel, vec![(MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, ExchangeType::Topic)])
        .await
        .map_err(|e| { error!("{}", e.to_string())})
        .expect(format!("Exchange {}to be declared", MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE).as_str());

    let config_loader = SiteConfigurationLoader::new()
        .map_err(|e| { error!("{}", e.to_string()) })
        .expect("Site configuration repository to be intialized");

    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");

    let mongo_client = initialize_client(ClientParams{
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: db_name.clone(),
        replica_set: Some(env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set")),
    })
        .await
        .map_err(|e| {
            panic!("Database initialization error: {}", e.to_string().as_str()); 
        })
        .expect("Datbase initialization error");

    let shared_app_context = build_shared_app_context(
        config_loader.get_config(),
        mongo_client.clone(),
        db_name.clone()
    )
        .await
        .map_err(|e| {
            error!("Failed to initialize SharedState: {}", e.to_string());
            e
        })
        .expect("SharedState to be initialzed");
    
    let site_config = web::Data::from(Arc::new(shared_app_context.config));
    let idp_registrar = web::Data::from(Arc::new(shared_app_context.idp_registrar));
    let federated_identity_service = web::Data::from(Arc::new(shared_app_context.federated_identity_service));
    let principal_service = web::Data::new(shared_app_context.principal_service);
    
    // Initialize AppState
    let state = AppState {
        db_name: db_name.clone(),
        channel: Arc::new(channel),
        client: mongo_client.clone()
    };

    // Model Deployment Service
    let model_deployment_service = Arc::new(
        model_deployment_service_builder(
            &mongo_client,
            db_name.clone(),
            state.channel.clone(),
        ).map_err(|e| {
            error!("Failed to initialize model deployment service: {}", e.to_string());
            e
        })
        .expect("ModelDeploymentService to be initialzed")
    );

    // Deployment Strategy Provider
    let deployment_strategy_provider = build_deployment_strategy_provider()
    .map_err(|e| {
        error!("Failed to initialize DeploymentStrategyProvider: {}", e.to_string());
        e
    })
    .expect("DeploymentStrategyProvider to be initialized");

    // Deployment Strategy Service
    let deployment_strategy_service = Arc::new(DeploymentStrategyService::new(
        deployment_strategy_provider
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(site_config.clone())
            .app_data(idp_registrar.clone())
            .app_data(federated_identity_service.clone())
            .app_data(principal_service.clone())
            .app_data(web::Data::from(model_deployment_service.clone()))
            .app_data(web::Data::from(deployment_strategy_service.clone()))
            .app_data(web::Data::new(state.clone()))

            // Globally-scoped middlewares.
            // NOTE: Middleware runs in reverse order of registration
            .wrap(from_fn(authenticate))
            .wrap(from_fn(resolve_tenancy))
            .wrap(Logger::default())
            .wrap(from_fn(preflight_short_circuit))


            .service(presentation::http::v1::actix_web::handlers::index::index)
            .service(presentation::http::v1::actix_web::handlers::list_strategies::list_strategies)
            .service(presentation::http::v1::actix_web::handlers::list_model_deployments::list_model_deployments)
            .service(presentation::http::v1::actix_web::handlers::deploy_model_with_strategy::deploy_model_with_strategy)
            .service(presentation::http::v1::actix_web::handlers::start_model_deployment::start_model_deployment)
            .service(presentation::http::v1::actix_web::handlers::stop_model_deployment::stop_model_deployment)
            .service(presentation::http::v1::actix_web::handlers::undeploy_model_deployment::undeploy_model_deployment)
            .service(presentation::http::v1::actix_web::handlers::openapi::openapi)
            .service(
                SwaggerUi::new("deployments-api/swagger-ui/{_:.*}")
                    .url("/deployments-api/specs/openapi.json", ApiDoc::openapi()),
            )
    })
        .bind(addrs)?
        .run()
        .await
}