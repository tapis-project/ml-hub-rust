use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_deployment_service_builder;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::GetModelDeploymentPathParams;
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{
    get,
    web,
    Responder,
};
use serde_json::to_value;
use shared::application::identity_context::IdentityContext;
use shared::application::inputs::deployment::GetModelDeploymentInput;

#[utoipa::path(
    get,
    path="/deployments-api/deployments/{deployment_id}",
    tag="Deployments",
    description="Get a model deployment by id",
    params(
        ("deployment_id" = Uuid, Path, description = "The id of the Model Deployment")
    ),
    responses(
        (status=200, description="Model deployment", body=contracts::responses::ModelDeploymentResponse),
        (status=400, description="Bad request", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Internal server error", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("deployments-api/deployments/{deployment_id}")]
async fn get_model_deployment(
    data: web::Data<AppState>,
    path: web::Path<GetModelDeploymentPathParams>,
    identity_context: IdentityContext,
) -> impl Responder {
    let service = model_deployment_service_builder(
        &data.client,
        data.db_name.clone(),
        data.channel.clone(),
    );

    let input = GetModelDeploymentInput {
        deployment_id: path.deployment_id,
    };

    let output = match service.get_model_deployment(input, &identity_context).await {
        Ok(output) => output,
        Err(err) => {
            use shared::application::services::model_deployment_service::ModelDeploymentServiceError;
            let (status, msg) = match err {
                ModelDeploymentServiceError::DeploymentNotFound(msg) => (404, msg),
                other => (500, other.to_string()),
            };
            return build_error_response(status, msg);
        }
    };

    let resp = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(Some(resp), None, None)
}
