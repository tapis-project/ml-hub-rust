use actix_web::{post, HttpResponse, Responder};
use crate::presentation::http::v1::contracts::responses::CreateAgentResponse;
use crate::presentation::http::v1::requests::create_agent::body::CreateAgentBody;

#[utoipa::path(
    post,
    path = "/agents-api/agents",
    tag = "Agents",
    summary = "Create an agent",
    request_body = CreateAgentBody,
    responses(
        (status = 200, description = "Agent created", body = CreateAgentResponse),
        (status = 501, description = "Agent operations are not implemented yet")
    )
)]
#[post("/agents-api/agents")]
pub async fn create_agent() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
