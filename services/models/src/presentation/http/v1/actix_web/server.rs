use crate::bootstrap::{
    factories::{
        external_model_discovery_service_factory, model_artifact_association_service_factory,
        model_creation_service_factory, model_query_service_factory,
    },
    state::AppState,
};
use crate::presentation::http::v1::actix_web::handlers;
use crate::presentation::http::v1::actix_web::openapi::ApiDoc;
use actix_web::{
    middleware::{from_fn, Logger},
    web, App, HttpServer,
};
use amqprs::channel::ExchangeType;
use log::error;
use shared::bootstrap::build_shared_app_context;
pub use shared::infra::_common::mongo::{initialize_client, ClientParams};
use shared::infra::configuration::site_configuration_loader::SiteConfigurationLoader;
use shared::infra::messaging::rabbitmq::connection::open_channel;
use shared::infra::messaging::rabbitmq::exchanges::{
    declare_exchanges, ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_PUBLICATION_EXCHANGE,
};
use shared::presentation::http::v1::actix_web::middleware::{
    authentication::authenticate, preflight::preflight_short_circuit, tenancy::resolve_tenancy,
};
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
            .unwrap_or(DEFAULT_PORT),
    );

    let broker_host =
        std::env::var("RABBIT_HOST").expect("RABBIT_URL missing from environment variables");
    let broker_port =
        std::env::var("RABBIT_PORT").expect("RABBIT_PORT missing from environment variables");
    let broker_username =
        std::env::var("RABBIT_USER").expect("RABBIT_USER missing from environment variables");
    let broker_password = std::env::var("RABBIT_PASSWORD")
        .expect("RABBIT_PASSWORD missing from environment variables");

    let (_connection, channel) = open_channel(
        broker_host,
        broker_port
            .parse::<u16>()
            .expect("u16 parsed from 'port' String"),
        broker_username,
        broker_password,
    )
    .await
    .map_err(|e| error!("{}", e.to_string()))
    .expect("Connection to message broker established and channel created");

    declare_exchanges(
        &channel,
        vec![
            (ARTIFACT_INGESTION_EXCHANGE, ExchangeType::Topic),
            (ARTIFACT_PUBLICATION_EXCHANGE, ExchangeType::Topic),
        ],
    )
    .await
    .map_err(|e| error!("{}", e.to_string()))
    .expect(
        format!(
            "Exchanges {} and {} to be declared",
            ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_PUBLICATION_EXCHANGE
        )
        .as_str(),
    );

    let config_loader = SiteConfigurationLoader::new()
        .map_err(|e| error!("{}", e.to_string()))
        .expect("Site configuration repository to be intialized");

    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");

    let mongo_client = initialize_client(ClientParams {
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: db_name.clone(),
        replica_set: Some(
            env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set"),
        ),
    })
    .await
    .map_err(|e| {
        panic!("Database initialization error: {}", e.to_string().as_str());
    })
    .expect("Database initialization error");

    let shared_app_context = build_shared_app_context(
        config_loader.get_config(),
        mongo_client.clone(),
        db_name.clone(),
    )
    .await
    .map_err(|e| {
        error!("Failed to initialize SharedState: {}", e.to_string());
        e
    })
    .expect("SharedState to be initialzed");

    let site_config = web::Data::from(Arc::new(shared_app_context.config));
    let idp_registrar = web::Data::from(Arc::new(shared_app_context.idp_registrar));
    let federated_identity_service =
        web::Data::from(Arc::new(shared_app_context.federated_identity_service));
    let principal_service = web::Data::new(shared_app_context.principal_service);

    let model_creation_service = Arc::new(model_creation_service_factory(
        &mongo_client,
        db_name.clone(),
    ));
    let model_query_service = Arc::new(model_query_service_factory(&mongo_client, db_name.clone()));
    let model_artifact_association_service = Arc::new(model_artifact_association_service_factory(
        &mongo_client,
        db_name.clone(),
    ));
    let external_model_discovery_service = Arc::new(external_model_discovery_service_factory(
        &mongo_client,
        db_name.clone(),
    ));

    // Initialize AppState
    let state = AppState {
        channel: Arc::new(channel),
        db_name: env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set"),
        client: mongo_client.clone(),
    };

    HttpServer::new(move || {
        App::new()
            // App-wide data
            .app_data(site_config.clone())
            .app_data(idp_registrar.clone())
            .app_data(federated_identity_service.clone())
            .app_data(principal_service.clone())
            .app_data(web::Data::from(model_creation_service.clone()))
            .app_data(web::Data::from(model_query_service.clone()))
            .app_data(web::Data::from(model_artifact_association_service.clone()))
            .app_data(web::Data::from(external_model_discovery_service.clone()))
            .app_data(web::Data::new(state.clone()))
            // Globally-scoped middlewares
            .wrap(from_fn(preflight_short_circuit))
            .wrap(Logger::default())
            // Public routes
            .service(handlers::index::index)
            .service(handlers::health_check::health_check)
            .service(
                SwaggerUi::new("models-api/swagger-ui/{_:.*}")
                    .url("/models-api/specs/openapi.json", ApiDoc::openapi()),
            )
            .service(handlers::openapi::openapi)
            // Protected routes
            .service(
                web::scope("")
                    .wrap(from_fn(authenticate))
                    .wrap(from_fn(resolve_tenancy))
                    .service(handlers::get_model::get_model)
                    .service(handlers::list_models::list_models)
                    .service(handlers::get_external_model::get_external_model)
                    .service(handlers::discover_external_models::discover_external_models)
                    .service(handlers::publish_model_artifact::publish_model_artifact)
                    .service(handlers::list_platforms::list_platforms)
                    .service(handlers::download_artifact::download_artifact)
                    .service(handlers::upload_model_artifact::upload_model_artifact)
                    .service(handlers::associate_model_with_artifact::associate_model_with_artifact)
                    .service(handlers::create_model::create_model)
                    .service(handlers::publish_model_artifact::publish_model_artifact)
                    .service(handlers::list_model_artifacts::list_model_artifacts)
                    .service(handlers::list_model_publications::list_model_publications)
                    .service(handlers::list_model_ingestions::list_model_ingestions)
                    .service(
                        handlers::list_publications_for_artifact::list_publications_for_artifact,
                    )
                    .service(handlers::get_model_ingestion::get_model_ingestion)
                    .service(handlers::get_model_publication::get_model_publication)
                    .service(handlers::get_model_artifact::get_model_artifact)
                    .service(handlers::list_tasks::list_tasks),
            )
    })
    .bind(addrs)?
    .run()
    .await
}
