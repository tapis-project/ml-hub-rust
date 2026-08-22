use utoipa::OpenApi;

use crate::config::VERSION;
use super::handlers::create_agent_record::__path_create_agent_record;
use super::handlers::healthcheck::__path_healthcheck;
use super::handlers::list_agent_records::__path_list_agent_records;

#[derive(OpenApi)]
#[openapi(
    info(title = "MLHub Agents API", version = VERSION),
    paths(list_agent_records, create_agent_record, healthcheck)
)]
pub struct ApiDoc;
