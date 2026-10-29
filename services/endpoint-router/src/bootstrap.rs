use std::{env, sync::Arc, time::Duration};

use moka::future::Cache;
use shared::{
    application::services::{
        endpoint_routing_service::EndpointRoutingService,
        federated_identity_service::FederatedIdentityService,
        federated_idp_registrar::FederatedIdpRegistrar, principal_service::PrincipalService,
    },
    bootstrap::build_shared_app_context,
    infra::{
        _common::mongo::{ClientParams, initialize_client},
        configuration::site_configuration_loader::SiteConfigurationLoader,
        persistence::mongo::repositories::{
            AgentRepository, EndpointRepository, ModelDeploymentRepository,
        },
    },
};

use crate::{processor::AuthenticationDependencies, route::CachedRoute};

pub struct EndpointRouterApplication {
    pub listen_address: String,
    pub authentication: AuthenticationDependencies,
    pub routing_service: Arc<EndpointRoutingService>,
    pub route_cache: Cache<String, CachedRoute>,
}

pub async fn bootstrap() -> Result<EndpointRouterApplication, Box<dyn std::error::Error>> {
    let database_name = env::var("MONGO_DBNAME")?;

    let client = initialize_client(ClientParams {
        username: env::var("MONGO_USERNAME")?,
        password: env::var("MONGO_PASSWORD")?,
        host: env::var("MONGO_HOST")?,
        port: env::var("MONGO_PORT")?,
        db: database_name.clone(),
        replica_set: Some(env::var("MONGO_REPLICA_SET")?),
    })
    .await?;

    let site_configuration = SiteConfigurationLoader::new()?.get_config();

    let shared_context = build_shared_app_context(
        site_configuration.clone(),
        client.clone(),
        database_name.clone(),
    )
    .await?;

    let authentication = AuthenticationDependencies {
        federated_identity_service: Arc::new(shared_context.federated_identity_service),
        idp_registrar: Arc::new(shared_context.idp_registrar),
        principal_service: Arc::new(shared_context.principal_service),
    };

    let routing_service = Arc::new(EndpointRoutingService::new(
        Arc::new(EndpointRepository::new(&client, database_name.clone())),
        Arc::new(AgentRepository::new(&client, database_name.clone())),
        Arc::new(ModelDeploymentRepository::new(&client, database_name)),
    ));

    let route_cache = Cache::builder()
        .max_capacity(site_configuration.endpoint_router.route_cache_capacity)
        .time_to_live(Duration::from_secs(
            site_configuration.endpoint_router.route_cache_ttl_seconds,
        ))
        .build();

    Ok(EndpointRouterApplication {
        listen_address: env::var("ENDPOINT_ROUTER_LISTEN_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0:9000".into()),
        authentication,
        routing_service,
        route_cache,
    })
}

#[allow(dead_code)]
fn assert_dependencies_are_send_and_sync(
    _federated_identity_service: &FederatedIdentityService,
    _idp_registrar: &FederatedIdpRegistrar,
    _principal_service: &PrincipalService,
) {
}
