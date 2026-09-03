use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::agent_service::AgentService, shared_kernel::context::RequestContext,
};
use validator::Validate;

use crate::presentation::http::v1::contracts::responses::CreateAgentResponse;
use crate::presentation::http::v1::requests::create_agent::body::CreateAgentBody;
use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    responses::Agent,
};

#[utoipa::path(post, path = "/agents-api/agents", tag = "Agents", summary = "Register an agent", request_body = CreateAgentBody,
    responses((status = 200, description = "Agent registered", body = CreateAgentResponse), (status = 500, description = "Unable to register agent")))]
#[post("agents-api/agents")]
pub async fn create_agent(
    body: web::Json<CreateAgentBody>,
    ctx: RequestContext,
    agent_service: web::Data<AgentService>,
) -> impl Responder {
    let request_body = body.into_inner();
    if let Err(error) = request_body.validate() {
        return build_error_response(500, error.to_string());
    }
    let agent = match agent_service
        .register_agent(&ctx, request_body.into())
        .await
    {
        Ok(agent) => agent,
        Err(error) => return build_error_response(500, error.to_string()),
    };
    let response = match to_value(Agent::from(agent)) {
        Ok(response) => response,
        Err(error) => return build_error_response(500, error.to_string()),
    };
    build_success_response(
        Some(response),
        Some("Successfully registered agent".into()),
        None,
    )
}
