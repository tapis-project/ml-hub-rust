//! Factories that compose Agents API infrastructure with application concerns.

use std::sync::Arc;

use mongodb::Client;
use shared::application::ports::agent_record::AgentRecordRepository;
use shared::application::services::agent_record_service::AgentRecordService;
use shared::infra::persistence::mongo::repositories::AgentRecordRepository as MongoAgentRecordRepository;

pub fn agent_record_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn AgentRecordRepository> {
    Arc::new(MongoAgentRecordRepository::new(client, db_name))
}

pub fn agent_record_service_factory(client: &Client, db_name: String) -> AgentRecordService {
    AgentRecordService::new(agent_record_repo_factory(client, db_name))
}
