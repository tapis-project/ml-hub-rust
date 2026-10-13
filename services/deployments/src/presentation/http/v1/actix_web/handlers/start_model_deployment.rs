use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::application::inputs::deployment::StartModelDeploymentInput;
use shared::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use shared::domain::entities::deployment::ModelDeploymentId;
use shared::presentation::http::v1::requests::deployment::StartModelDeploymentPathParams;

#[utoipa::path(
    post,
    path = "/deployments-api/deployments/{deployment_id}/start",
    tag = "Deployments",
    description = "Start a model deployment on its selected target",
    params(
        (
            "deployment_id" = Uuid,
            Path,
            description = "The id of the Model Deployment"
        ),
    ),
    responses(
        (
            status = 200,
            description = "Model deployment",
            body = contracts::responses::ModelDeploymentResponse
        ),
        (
            status = 400,
            description = "Not found",
            body = contracts::responses::BadRequestResponse
        ),
        (
            status = 404,
            description = "Not found",
            body = contracts::responses::NotFoundResponse
        ),
        (
            status = 409,
            description = "Model deployment has expired",
            body = contracts::responses::ConflictResponse
        ),
        (
            status = 500,
            description = "Not found",
            body = contracts::responses::ServerErrorResponse
        ),
    ),
)]
#[post("deployments-api/deployments/{deployment_id}/start")]
async fn start_model_deployment(
    path: web::Path<StartModelDeploymentPathParams>,
    service: web::Data<ModelDeploymentService>,
) -> impl Responder {
    let input = StartModelDeploymentInput {
        owner: "mlhub".into(),
        deployment_id: ModelDeploymentId::reconstitute(path.deployment_id),
    };

    let output = match service.start_model_deployment(input).await {
        Ok(output) => output,
        Err(ModelDeploymentServiceError::DeploymentExpired) => {
            return build_error_response(409, "Model deployment has expired".into())
        }
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let resp = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(Some(resp), None, None)
}
