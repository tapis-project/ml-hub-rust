use actix_web::{get, HttpResponse, Responder};
use crate::presentation::http::v1::contracts::responses::ListAgentsResponse;

#[utoipa::path(
    get,
    path = "/agents-api/agents",
    tag = "Agents",
    summary = "List agents",
    responses(
        (status = 200, description = "A list of agents", body = ListAgentsResponse),
        (status = 501, description = "Agent operations are not implemented yet")
    )
)]
#[get("/agents-api/agents")]
pub async fn list_agents() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
