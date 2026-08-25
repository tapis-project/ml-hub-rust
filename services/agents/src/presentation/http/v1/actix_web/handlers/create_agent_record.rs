use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::agent_record_service::AgentRecordService,
    shared_kernel::context::RequestContext,
};
use validator::Validate;

use crate::presentation::http::v1::contracts::responses::CreateAgentRecordResponse;
use crate::presentation::http::v1::requests::create_agent_record::body::CreateAgentRecordBody;
use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    responses::AgentRecord,
};

#[utoipa::path(
    post,
    path = "/agents-api/agent-records",
    tag = "Agents",
    summary = "Create an agent record",
    request_body = CreateAgentRecordBody,
    responses(
        (status = 200, description = "Agent record created", body = CreateAgentRecordResponse),
        (status = 500, description = "Unable to create agent record")
    )
)]
#[post("/agents-api/agent-records")]
pub async fn create_agent_record(
    body: web::Json<CreateAgentRecordBody>,
    ctx: RequestContext,
    agent_record_service: web::Data<AgentRecordService>,
) -> impl Responder {
    let request_body = body.into_inner();
    if let Err(error) = request_body.validate() {
        return build_error_response(500, error.to_string());
    }

    let agent_record = match agent_record_service
        .create_agent_record(&ctx, request_body.into())
        .await
    {
        Ok(agent_record) => agent_record,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let response = match to_value(AgentRecord::from(agent_record)) {
        Ok(response) => response,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(response),
        Some("Successfully created agent record".into()),
        None,
    )
}
