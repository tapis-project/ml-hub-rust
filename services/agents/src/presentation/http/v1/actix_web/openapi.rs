use utoipa::OpenApi;

use super::handlers::create_agent::__path_create_agent;
use super::handlers::create_agent_record::__path_create_agent_record;
use super::handlers::healthcheck::__path_healthcheck;
use super::handlers::list_agent_records::__path_list_agent_records;
use super::handlers::list_agents::__path_list_agents;
use crate::config::VERSION;

#[derive(OpenApi)]
#[openapi(
    info(title = "MLHub Agents API", version = VERSION),
    paths(list_agent_records, create_agent_record, list_agents, create_agent, healthcheck)
)]
pub struct ApiDoc;
