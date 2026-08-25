use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::agent_record_service::AgentRecordService,
    presentation::http::v1::requests::list_agent_records::query::Scope,
    shared_kernel::context::RequestContext,
};

use crate::presentation::http::v1::contracts::responses::ListAgentRecordsResponse;
use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    requests::list_agent_records::query::ListAgentRecordsQueryParams,
    responses::AgentRecord,
};

#[utoipa::path(
    get,
    path = "/agents-api/agent-records",
    tag = "Agents",
    summary = "List agent records",
    params(ListAgentRecordsQueryParams),
    responses(
        (status = 200, description = "A list of agent records", body = ListAgentRecordsResponse),
        (status = 500, description = "Unable to list agent records")
    )
)]
#[get("/agents-api/agent-records")]
pub async fn list_agent_records(
    query: web::Query<ListAgentRecordsQueryParams>,
    ctx: RequestContext,
    agent_record_service: web::Data<AgentRecordService>,
) -> impl Responder {
    let agent_records = match query.scope {
        Scope::Owned => agent_record_service.list_for_user(&ctx).await,
        Scope::SharedPublic => agent_record_service.list_for_tenant(&ctx).await,
    };
    let agent_records = match agent_records {
        Ok(agent_records) => agent_records,
        Err(error) => return build_error_response(500, error.to_string()),
    };
    let response = match to_value(
        agent_records
            .into_iter()
            .map(AgentRecord::from)
            .collect::<Vec<_>>(),
    ) {
        Ok(response) => response,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(response),
        Some("Successfully listed agent records".into()),
        None,
    )
}
