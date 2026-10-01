use actix_web::{post, web, HttpResponse, Responder};
use serde_json::to_value;
use shared::application::inputs::deployment::{Argument, DeployWithOptionInput};
use shared::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use shared::domain::entities::deployment::ParallelismStrategy;
use shared::domain::entities::deployment_option::DeploymentOptionId;
use shared::presentation::http::v1::responses::JsonResponse;
use shared::shared_kernel::context::RequestContext;

use crate::config::VERSION;
use crate::presentation::http::v1::actix_web::helpers::build_error_response;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::DeployModelWithOptionBody;
use crate::presentation::http::v1::responses::ModelDeployment;

#[utoipa::path(
    post,
    path = "/deployments-api/deployments",
    tag = "Deployments",
    description = "Deploy an ExternalModel with a deployment option",
    request_body = DeployModelWithOptionBody,
    responses(
        (
            status = 201,
            description = "Model deployment created",
            body = contracts::responses::ModelDeploymentResponse
        ),
        (
            status = 400,
            description = "Invalid deployment request",
            body = contracts::responses::BadRequestResponse
        ),
        (
            status = 404,
            description = "Deployment option, ExternalModel, or target not found",
            body = contracts::responses::NotFoundResponse
        ),
        (
            status = 409,
            description = "Deployment option is unavailable"
        ),
        (
            status = 500,
            description = "Unable to create deployment",
            body = contracts::responses::ServerErrorResponse
        ),
    ),
)]
#[post("deployments-api/deployments")]
async fn deploy_model_with_option(
    body: web::Json<DeployModelWithOptionBody>,
    ctx: RequestContext,
    model_deployment_service: web::Data<ModelDeploymentService>,
) -> impl Responder {
    let input = DeployWithOptionInput {
        name: body.name.clone(),
        description: body.description.clone(),
        deployment_option_id: DeploymentOptionId::reconstitute(body.deployment_option_id),
        replicas: body.replicas,
        parallelism_strategies: body.parallelism_strategies.as_ref().map(|strategies| {
            strategies
                .iter()
                .cloned()
                .map(ParallelismStrategy::from)
                .collect()
        }),
        arguments: body
            .arguments
            .as_deref()
            .unwrap_or_default()
            .iter()
            .cloned()
            .map(Argument::from)
            .collect(),
        deployment_modality: body.deployment_modality.clone().into(),
    };

    let output = match model_deployment_service
        .deploy_model_with_option(&ctx, input)
        .await
    {
        Ok(output) => output,
        Err(ModelDeploymentServiceError::MissingDeploymentOption(_))
        | Err(ModelDeploymentServiceError::MissingExternalModel(_))
        | Err(ModelDeploymentServiceError::MissingHpcCluster(_))
        | Err(ModelDeploymentServiceError::MissingBatchSchedulerQueue(_)) => {
            return build_error_response(404, "Deployment resource not found".into());
        }
        Err(ModelDeploymentServiceError::DeploymentOptionUnavailable) => {
            return build_error_response(409, "Deployment option is unavailable".into());
        }
        Err(ModelDeploymentServiceError::InvalidParameters(error)) => {
            return build_error_response(400, error.to_string());
        }
        Err(ModelDeploymentServiceError::ModelDeploymentError(
            shared::domain::entities::deployment::ModelDeploymentError::UnsupportedDeploymentModality(
                error,
            ),
        )) => {
            return build_error_response(400, error.to_string());
        }
        Err(ModelDeploymentServiceError::ModelDeploymentDomainError(error)) => {
            return build_error_response(400, error.to_string());
        }
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    HttpResponse::Created().json(JsonResponse {
        status: Some(201),
        message: Some("Model deployment created".into()),
        result: Some(result),
        metadata: Some(serde_json::Value::Object(serde_json::Map::new())),
        version: Some(VERSION.into()),
    })
}
