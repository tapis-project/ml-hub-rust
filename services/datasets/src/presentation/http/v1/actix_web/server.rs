use super::handlers;
use crate::{
    bootstrap::{factories::dataset_service_factory, state::AppState},
    config::{DEFAULT_HOST, DEFAULT_PORT},
};
use actix_web::{
    middleware::{from_fn, Logger},
    web, App, HttpServer,
};
use log::error;
use shared::{
    bootstrap::build_shared_app_context,
    infra::{
        _common::mongo::{initialize_client, ClientParams},
        configuration::site_configuration_loader::SiteConfigurationLoader,
    },
    presentation::http::v1::actix_web::middleware::{
        authentication::authenticate, preflight::preflight_short_circuit, tenancy::resolve_tenancy,
    },
};
use std::{env, sync::Arc};

pub async fn run_server() -> std::io::Result<()> {
    env_logger::init();

    let address = (
        env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.into()),
        env::var("PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_PORT),
    );

    let config_loader = SiteConfigurationLoader::new()
        .map_err(|e| error!("{e}"))
        .expect("Site configuration repository to be initialized");

    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");

    let client = initialize_client(ClientParams {
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
    .map_err(|e| panic!("Database initialization error: {e}"))
    .expect("Database initialization error");

    let shared =
        build_shared_app_context(config_loader.get_config(), client.clone(), db_name.clone())
            .await
            .map_err(|e| {
                error!("Failed to initialize SharedAppContext: {e}");
                e
            })
            .expect("SharedAppContext to be initialized");

    let site_config = web::Data::from(Arc::new(shared.config));
    let idp_registrar = web::Data::from(Arc::new(shared.idp_registrar));
    let federated_identity_service = web::Data::from(Arc::new(shared.federated_identity_service));
    let principal_service = web::Data::new(shared.principal_service);

    let state = AppState { client, db_name };

    let dataset_service = web::Data::new(dataset_service_factory(
        &state.client,
        state.db_name.clone(),
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(site_config.clone())
            .app_data(idp_registrar.clone())
            .app_data(federated_identity_service.clone())
            .app_data(principal_service.clone())
            .app_data(dataset_service.clone())
            .app_data(web::Data::new(state.clone()))
            .wrap(from_fn(preflight_short_circuit))
            .wrap(Logger::default().exclude("/datasets-api/healthcheck"))
            .service(handlers::healthcheck::healthcheck)
            .service(handlers::openapi::openapi)
            .service(
                web::scope("")
                    .wrap(from_fn(authenticate))
                    .wrap(from_fn(resolve_tenancy))
                    .service(handlers::register_dataset::register_dataset)
                    .service(handlers::get_dataset::get_dataset)
                    .service(handlers::list_datasets::list_datasets),
            )
    })
    .bind(address)?
    .run()
    .await
}
