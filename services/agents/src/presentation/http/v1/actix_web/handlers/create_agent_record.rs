use actix_web::{post, HttpResponse, Responder};
use crate::presentation::http::v1::contracts::responses::CreateAgentRecordResponse;
use crate::presentation::http::v1::requests::create_agent_record::body::CreateAgentRecordBody;

#[utoipa::path(
    post,
    path = "/agents-api/agent-records",
    tag = "Agents",
    summary = "Create an agent record",
    request_body = CreateAgentRecordBody,
    responses(
        (status = 200, description = "Agent record created", body = CreateAgentRecordResponse),
        (status = 501, description = "Agent record operations are not implemented yet")
    )
)]
#[post("/agents-api/agent-records")]
pub async fn create_agent_record() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
