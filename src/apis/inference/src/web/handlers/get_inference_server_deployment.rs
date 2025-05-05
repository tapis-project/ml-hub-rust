use actix_web::{web, get, HttpResponse, Responder};
use crate::application::repositories::InferenceServerDeploymentRepository as _;
use crate::infra::mongo::repositories::InferenceServerDeploymentRepository;
use crate::bootstrap::state::AppState;
use log::debug;

#[get("/inference-api/inference-servers/{inference_server_name}/deployments/{deployment_name}")]
async fn get_inference_server_deployment(
    _path: web::Path<String>,
    data: web::Data<AppState>
) -> impl Responder {
    debug!("Operation get_inference_server_deployment");
    let repo = InferenceServerDeploymentRepository::new(data.db.clone());
    let _test = repo.find_by_labels(String::from("test"), String::from("test"));
    HttpResponse::Ok()
        .content_type("text/html")
        .body("get inference server deployment")
}
