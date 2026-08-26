use std::{env, sync::Arc};

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

use super::handlers;
use crate::{
    bootstrap::{
        factories::{agent_record_service_factory, agent_service_factory},
        state::AppState,
    },
    config::{DEFAULT_HOST, DEFAULT_PORT},
};

pub async fn run_server() -> std::io::Result<()> {
    env_logger::init();

    let address = (
        env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_owned()),
        env::var("PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT),
    );

    let config_loader = SiteConfigurationLoader::new()
        .map_err(|err| error!("{err}"))
        .expect("Site configuration repository to be initialized");

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
    .map_err(|err| panic!("Database initialization error: {err}"))
    .expect("Database initialization error");

    let shared_app_context = build_shared_app_context(
        config_loader.get_config(),
        mongo_client.clone(),
        db_name.clone(),
    )
    .await
    .map_err(|err| {
        error!("Failed to initialize SharedAppContext: {err}");
        err
    })
    .expect("SharedAppContext to be initialized");

    let site_config = web::Data::from(Arc::new(shared_app_context.config));
    let idp_registrar = web::Data::from(Arc::new(shared_app_context.idp_registrar));
    let federated_identity_service =
        web::Data::from(Arc::new(shared_app_context.federated_identity_service));
    let principal_service = web::Data::new(shared_app_context.principal_service);
    let state = AppState {
        client: mongo_client,
        db_name,
    };
    let agent_record_service = web::Data::new(agent_record_service_factory(
        &state.client,
        state.db_name.clone(),
    ));
    let agent_service = web::Data::new(agent_service_factory(&state.client, state.db_name.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(site_config.clone())
            .app_data(idp_registrar.clone())
            .app_data(federated_identity_service.clone())
            .app_data(principal_service.clone())
            .app_data(agent_record_service.clone())
            .app_data(agent_service.clone())
            .app_data(web::Data::new(state.clone()))
            .wrap(from_fn(preflight_short_circuit))
            .wrap(Logger::default())
            .service(handlers::healthcheck::healthcheck)
            .service(handlers::openapi::openapi)
            .service(
                web::scope("")
                    .wrap(from_fn(authenticate))
                    .wrap(from_fn(resolve_tenancy))
                    .service(handlers::list_agent_records::list_agent_records)
                    .service(handlers::create_agent_record::create_agent_record)
                    .service(handlers::list_agents::list_agents)
                    .service(handlers::create_agent::create_agent),
            )
    })
    .bind(address)?
    .run()
    .await
}
