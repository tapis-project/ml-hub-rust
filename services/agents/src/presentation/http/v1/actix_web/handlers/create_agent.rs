use actix_web::{post, HttpResponse, Responder};

#[utoipa::path(
    post,
    path = "/agents-api/agents",
    tag = "Agents",
    summary = "Create an agent",
    responses((status = 501, description = "Agent operations are not implemented yet"))
)]
#[post("/agents-api/agents")]
pub async fn create_agent() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
