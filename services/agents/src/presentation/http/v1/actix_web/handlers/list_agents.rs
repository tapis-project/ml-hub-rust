use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/agents-api/agents",
    tag = "Agents",
    summary = "List agents",
    responses((status = 501, description = "Agent operations are not implemented yet"))
)]
#[get("/agents-api/agents")]
pub async fn list_agents() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
