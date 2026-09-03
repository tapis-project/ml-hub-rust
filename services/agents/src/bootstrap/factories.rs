//! Factories that compose Agents API infrastructure with application concerns.

use std::sync::Arc;

use mongodb::Client;
use shared::application::ports::agent::AgentRepository;
use shared::application::ports::agent_record::AgentRecordRepository;
use shared::application::ports::endpoint::EndpointRepository;
use shared::application::services::agent_record_service::AgentRecordService;
use shared::application::services::agent_service::AgentService;
use shared::application::services::endpoint_catalog_service::EndpointCatalogService;
use shared::application::services::endpoint_issuance_service::EndpointIssuanceService;
use shared::infra::persistence::mongo::repositories::AgentRecordRepository as MongoAgentRecordRepository;
use shared::infra::persistence::mongo::repositories::AgentRepository as MongoAgentRepository;
use shared::infra::persistence::mongo::repositories::EndpointRepository as MongoEndpointRepository;

pub fn agent_record_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn AgentRecordRepository> {
    Arc::new(MongoAgentRecordRepository::new(client, db_name))
}

pub fn agent_record_service_factory(client: &Client, db_name: String) -> AgentRecordService {
    AgentRecordService::new(agent_record_repo_factory(client, db_name))
}

pub fn agent_repo_factory(client: &Client, db_name: String) -> Arc<dyn AgentRepository> {
    Arc::new(MongoAgentRepository::new(client, db_name))
}

pub fn endpoint_repo_factory(client: &Client, db_name: String) -> Arc<dyn EndpointRepository> {
    Arc::new(MongoEndpointRepository::new(client, db_name))
}

pub fn endpoint_issuance_service_factory(
    client: &Client,
    db_name: String,
) -> EndpointIssuanceService {
    EndpointIssuanceService::new(endpoint_repo_factory(client, db_name))
}

pub fn endpoint_catalog_service_factory(
    client: &Client,
    db_name: String,
) -> EndpointCatalogService {
    EndpointCatalogService::new(endpoint_repo_factory(client, db_name))
}

pub fn agent_service_factory(client: &Client, db_name: String) -> AgentService {
    AgentService::new(
        agent_repo_factory(client, db_name.clone()),
        agent_record_repo_factory(client, db_name.clone()),
        endpoint_issuance_service_factory(client, db_name),
    )
}
