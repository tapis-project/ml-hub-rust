use std::sync::Arc;
use crate::presentation;
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::build_deployment_strategy_provider;
use crate::infra::persistence::mongo::database::{ClientParams, get_db};
use crate::presentation::http::v1::actix_web::openapi::ApiDoc;
use actix_web::{App, HttpServer, web, middleware::from_fn};
use amqprs::channel::ExchangeType;
use shared::bootstrap::build_shared_app_context;
use shared::infra::configuration::site_configuration_loader::SiteConfigurationRepository;
use shared::presentation::http::v1::actix_web::middleware::{authentication::authenticate, tenancy::resolve_tenancy};
use shared::infra::messaging::rabbitmq::connection::open_channel;
use shared::infra::messaging::rabbitmq::exchanges::{declare_exchanges, MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE};
use std::env;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use log::{error, warn};

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

    let deployment_strategy_provider = build_deployment_strategy_provider();

    let client_strategy_sets = match deployment_strategy_provider {
        Ok(p) => Arc::new(p.provide().clone()),
        Err(err) => {
            warn!("Error initializing deployment strategy provider: {}", err.to_string());
            Arc::new(vec![])
        }
    };

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
        .map_err(|err| { error!("{}", err.to_string()) })
        .expect("Connection to message broker established and channel created");

    declare_exchanges(&channel, vec![(MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE, ExchangeType::Topic)])
        .await
        .map_err(|err| { error!("{}", err.to_string())})
        .expect(format!("Exchange {}to be declared", MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE).as_str());

    let config_repository = SiteConfigurationRepository::new()
        .map_err(|err| { error!("{}", err.to_string()) })
        .expect("Site configuration repository to be intialized");

    let shared_app_context = build_shared_app_context(config_repository.get_config())
        .await
        .map_err(|err| {
            error!("Failed to initialize SharedState: {}", err.to_string());
            err
        })
        .expect("SharedState to be initialzed");
    
    let site_config = web::Data::from(Arc::new(shared_app_context.config));
    let idp_registrar = web::Data::from(Arc::new(shared_app_context.idp_registrar));
    let federated_identity_service = web::Data::from(Arc::new(shared_app_context.federated_identity_service));

    // Initialize AppState
    let state = AppState {
        client_strategy_sets,
        channel: Arc::new(channel),
        db: get_db(ClientParams{
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
            .expect("Datbase initialization error")
    };

    HttpServer::new(move || {
        App::new()
            .app_data(site_config.clone())
            .app_data(idp_registrar.clone())
            .app_data(federated_identity_service.clone())
            .app_data(web::Data::new(state.clone()))
            .wrap(from_fn(authenticate))
            .wrap(from_fn(resolve_tenancy))
            .service(presentation::http::v1::actix_web::handlers::index::index)
            .service(presentation::http::v1::actix_web::handlers::list_strategies::list_strategies)
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