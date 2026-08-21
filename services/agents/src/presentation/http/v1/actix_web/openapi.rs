use utoipa::OpenApi;

use crate::config::VERSION;
use super::handlers::create_agent::__path_create_agent;
use super::handlers::healthcheck::__path_healthcheck;
use super::handlers::list_agents::__path_list_agents;

#[derive(OpenApi)]
#[openapi(
    info(title = "MLHub Agents API", version = VERSION),
    paths(list_agents, create_agent, healthcheck)
)]
pub struct ApiDoc;
