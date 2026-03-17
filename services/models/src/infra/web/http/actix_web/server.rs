use std::sync::Arc;
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::build_deployment_strategy_provider;
use crate::infra::persistence::mongo::database::{ClientParams, get_db};
use crate::presentation::http::v1::actix_web::openapi::ApiDoc;
use crate::presentation::http::v1::actix_web::handlers;
use actix_web::{App, HttpServer, middleware::from_fn, web};
use amqprs::channel::ExchangeType;
use shared::bootstrap::build_shared_app_context;
use shared::infra::messaging::rabbitmq::connection::open_channel;
use shared::infra::messaging::rabbitmq::exchanges::{declare_exchanges, ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_PUBLICATION_EXCHANGE};
use shared::presentation::http::v1::actix_web::middleware::authentication::authenticate;
use shared::presentation::http::v1::actix_web::middleware::tenancy::resolve_tenancy;
use std::env;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use log::{warn, error};


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

    let broker_host = std::env::var("ARTIFACT_OP_MQ_HOST").expect("ARTIFACT_OP_MQ_URL missing from environment variables");
    let broker_port = std::env::var("ARTIFACT_OP_MQ_PORT").expect("ARTIFACT_OP_MQ_PORT missing from environment variables");
    let broker_username = std::env::var("ARTIFACT_OP_MQ_USER").expect("ARTIFACT_OP_MQ_USER missing from environment variables");
    let broker_password = std::env::var("ARTIFACT_OP_MQ_PASSWORD").expect("ARTIFACT_OP_MQ_PASSWORD missing from environment variables");
    
    let (_connection, channel) = open_channel(
        broker_host,
        broker_port.parse::<u16>().expect("u16 parsed from 'port' String"),
        broker_username,
        broker_password,
    )
        .await
        .map_err(|err| { error!("{}", err.to_string()) })
        .expect("Connection to message broker established and channel created");

    declare_exchanges(
        &channel, 
        vec![
            (ARTIFACT_INGESTION_EXCHANGE, ExchangeType::Topic),
            (ARTIFACT_PUBLICATION_EXCHANGE, ExchangeType::Topic),
        ]
    ).await
        .map_err(|err| { error!("{}", err.to_string())})
        .expect(format!("Exchanges {} and {} to be declared", ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_PUBLICATION_EXCHANGE).as_str());

    let shared_app_context = build_shared_app_context()
        .await
        .map_err(|err| {
            error!("Failed to initialize SharedState: {}", err.to_string());
            err
        })
        .expect("SharedState to be initialzed");

    let idp_registrar = web::Data::from(Arc::new(shared_app_context.idp_registrar));
    let federated_identity_service = web::Data::from(Arc::new(shared_app_context.federated_identity_service));

    // Initialize AppState
    let state = AppState {
        client_strategy_sets,
        channel: Arc::new(channel),
        db: get_db(ClientParams{
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
            .expect("Datbase initialization error"),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(idp_registrar.clone())
            .app_data(federated_identity_service.clone())
            .app_data(web::Data::new(state.clone()))
            .wrap(from_fn(resolve_tenancy))
            .wrap(from_fn(authenticate))
            .service(handlers::index::index)
            .service(handlers::health_check::health_check)
            .service(handlers::get_model_by_platform::get_model_by_platform)
            .service(handlers::list_models_by_platform::list_models_by_platform)
            .service(handlers::ingest_external_model::ingest_external_model)
            .service(handlers::discover_models_by_platform::discover_models_by_platform)
            .service(handlers::discover_models::discover_models)
            .service(handlers::publish_model_artifact::publish_model_artifact)
            .service(handlers::list_platforms::list_platforms)
            .service(handlers::download_artifact::download_artifact)
            .service(handlers::upload_model_artifact::upload_model_artifact)
            .service(handlers::associate_model_metadata_with_artifact::associate_model_metadata_with_artifact)
            .service(handlers::create_model_metadata::create_model_metadata)
            .service(handlers::publish_model_artifact::publish_model_artifact)
            .service(handlers::list_model_artifacts::list_model_artifacts)
            .service(handlers::list_model_publications::list_model_publications)
            .service(handlers::list_model_ingestions::list_model_ingestions)
            .service(handlers::list_publications_for_artifact::list_publications_for_artifact)
            .service(handlers::get_model_ingestion::get_model_ingestion)
            .service(handlers::get_model_publication::get_model_publication)
            .service(handlers::get_model_artifact::get_model_artifact)
            .service(handlers::list_tasks::list_tasks)
            .service(handlers::ingest_canonical_model::ingest_canonical_model)
            .service(handlers::openapi::openapi)
            .service(
                SwaggerUi::new("models-api/swagger-ui/{_:.*}")
                    .url("/models-api/specs/openapi.json", ApiDoc::openapi()),
            )
    })
        .bind(addrs)?
        .run()
        .await
}