use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::StopModelDeploymentPathParams;
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::application::inputs::deployment::StopModelDeploymentInput;
use shared::application::services::endpoint_query_service::EndpointQueryService;
use shared::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use shared::domain::entities::deployment::ModelDeploymentId;
use shared::domain::entities::endpoint::NetworkAddressableResource;
use shared::shared_kernel::context::RequestContext;

#[utoipa::path(
    post,
    path = "/deployments-api/deployments/{deployment_id}/stop",
    tag = "Deployments",
    description = "Stop a Model Deployment",
    params(
        (
            "deployment_id" = Uuid,
            Path,
            description = "The the id of the Model Deployment"
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
#[post("deployments-api/deployments/{deployment_id}/stop")]
async fn stop_model_deployment(
    path: web::Path<StopModelDeploymentPathParams>,
    service: web::Data<ModelDeploymentService>,
    endpoint_query_service: web::Data<EndpointQueryService>,
    ctx: RequestContext,
) -> impl Responder {
    let input = StopModelDeploymentInput {
        owner: "mlhub".into(),
        deployment_id: ModelDeploymentId::reconstitute(path.deployment_id),
    };

    let output = match service.stop_model_deployment(input).await {
        Ok(output) => output,
        Err(ModelDeploymentServiceError::DeploymentExpired) => {
            return build_error_response(409, "Model deployment has expired".into())
        }
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let endpoints = if output.deployment.network_target_names().is_empty() {
        Vec::new()
    } else {
        match endpoint_query_service
            .find_by_network_addressable_resource(&ctx, &output.deployment)
            .await
        {
            Ok(endpoints) => endpoints,
            Err(error) => return build_error_response(500, error.to_string()),
        }
    };

    let response = match ModelDeployment::try_from((output.deployment, endpoints)) {
        Ok(response) => response,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let resp = match to_value(response) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(Some(resp), None, None)
}
