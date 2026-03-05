use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use actix_web::{
    get, Responder
};
use serde_json::to_value;
use shared::presentation::http::v1::contracts;
use shared::presentation::http::v1::requests::task::Task;

#[utoipa::path(
    get,
    path="/models-api/tasks",
    tag="Tasks",
    description="List all model tasks/capabilities",
    responses(
        (status=200, description="Listed tasks", body=contracts::responses::ListTasksResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("models-api/tasks")]
async fn list_tasks() -> impl Responder {
    match to_value(Task::as_vec()) {
        Ok(v) => build_success_response(Some(v), Some("Success".into()), None),
        Err(err) => build_error_response(500, err.to_string())
    }
}