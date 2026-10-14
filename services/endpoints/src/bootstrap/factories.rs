use std::sync::Arc;

use mongodb::Client;
use shared::{
    application::services::endpoint_catalog_query_service::EndpointCatalogQueryService,
    infra::persistence::mongo::repositories::{
        AgentRepository, EndpointRepository, ModelDeploymentRepository,
    },
};

pub fn endpoint_catalog_query_service(
    client: &Client,
    db_name: String,
) -> EndpointCatalogQueryService {
    EndpointCatalogQueryService::new(
        Arc::new(EndpointRepository::new(client, db_name.clone())),
        Arc::new(AgentRepository::new(client, db_name.clone())),
        Arc::new(ModelDeploymentRepository::new(client, db_name)),
    )
}
