use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::agent_service::AgentService,
    application::services::endpoint_catalog_service::EndpointCatalogService,
    presentation::http::v1::requests::list_agents::query::Scope,
    presentation::http::v1::responses::endpoints::Endpoint, shared_kernel::context::RequestContext,
};

use crate::presentation::http::v1::contracts::responses::ListAgentsResponse;
use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    requests::list_agents::query::ListAgentsQueryParams,
    responses::Agent,
};

#[utoipa::path(get, path = "/agents-api/agents", tag = "Agents", summary = "List agents", params(ListAgentsQueryParams),
    responses((status = 200, description = "A list of agents", body = ListAgentsResponse), (status = 500, description = "Unable to list agents")))]
#[get("agents-api/agents")]
pub async fn list_agents(
    query: web::Query<ListAgentsQueryParams>,
    ctx: RequestContext,
    agent_service: web::Data<AgentService>,
    endpoint_catalog_service: web::Data<EndpointCatalogService>,
) -> impl Responder {
    let agents = match query.scope {
        Scope::Owned => agent_service.list_for_user(&ctx).await,
        Scope::Shared => agent_service.list_shared_with_user(&ctx).await,
    };
    let agents = match agents {
        Ok(agents) => agents,
        Err(error) => return build_error_response(500, error.to_string()),
    };
    let response_agents = if query.include_endpoints {
        let mut response_agents = Vec::with_capacity(agents.len());

        for agent in agents {
            let endpoints = match endpoint_catalog_service
                .find_by_network_addressable_resource(&ctx, &agent)
                .await
            {
                Ok(endpoints) => endpoints
                    .into_iter()
                    .filter_map(|endpoint| Endpoint::try_from((endpoint, &agent)).ok())
                    .collect(),
                Err(error) => return build_error_response(500, error.to_string()),
            };

            response_agents.push(Agent::from((agent, endpoints)));
        }

        response_agents
    } else {
        agents.into_iter().map(Agent::from).collect()
    };

    let response = match to_value(response_agents) {
        Ok(response) => response,
        Err(error) => return build_error_response(500, error.to_string()),
    };
    build_success_response(
        Some(response),
        Some("Successfully listed agents".into()),
        None,
    )
}
