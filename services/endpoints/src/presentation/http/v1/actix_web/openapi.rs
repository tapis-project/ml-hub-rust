use utoipa::OpenApi;

use super::handlers::get_endpoint::__path_get_endpoint;
use super::handlers::healthcheck::__path_healthcheck;
use super::handlers::list_endpoints::__path_list_endpoints;
use super::handlers::list_resource_endpoints::__path_list_resource_endpoints;
use crate::config::VERSION;

#[derive(OpenApi)]
#[openapi(
    info(title = "MLHub Endpoints API", version = VERSION),
    paths(list_endpoints, get_endpoint, list_resource_endpoints, healthcheck)
)]
pub struct ApiDoc;
