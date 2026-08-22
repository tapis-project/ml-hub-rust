use actix_web::{get, HttpResponse, Responder};
use crate::presentation::http::v1::contracts::responses::ListAgentRecordsResponse;

#[utoipa::path(
    get,
    path = "/agents-api/agent-records",
    tag = "Agents",
    summary = "List agent records",
    responses(
        (status = 200, description = "A list of agent records", body = ListAgentRecordsResponse),
        (status = 501, description = "Agent record operations are not implemented yet")
    )
)]
#[get("/agents-api/agent-records")]
pub async fn list_agent_records() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
