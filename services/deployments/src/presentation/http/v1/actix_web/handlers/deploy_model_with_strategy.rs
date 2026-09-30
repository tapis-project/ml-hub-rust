use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::{
    DeployModelWithStrategyBody, DeployModelWithStrategyPathParams,
};
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::application::inputs::deployment::{Argument, DeployWithStrategyInput};
use shared::application::services::model_deployment_service::{
    ModelDeploymentService, ModelDeploymentServiceError,
};
use shared::domain::entities::deployment::ParallelismStrategy;
use shared::shared_kernel::context::RequestContext;

#[utoipa::path(
    post,
    path = "/deployments-api/platforms/{platform}/strategies/{strategy_name}",
    tag = "Deployments",
    description = "Deploy a model to a target platform",
    request_body = DeployModelWithStrategyBody,
    params(DeployModelWithStrategyPathParams),
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
            status = 500,
            description = "Not found",
            body = contracts::responses::ServerErrorResponse
        ),
    ),
)]
#[post("deployments-api/platforms/{platform}/strategies/{strategy_name}")]
async fn deploy_model_with_strategy(
    body: web::Json<DeployModelWithStrategyBody>,
    path: web::Path<DeployModelWithStrategyPathParams>,
    ctx: RequestContext,
    model_deployment_service: web::Data<ModelDeploymentService>,
) -> impl Responder {
    let input = DeployWithStrategyInput {
        name: body.name.clone(),
        description: body.description.clone(),
        model_id: body.model_id,
        replicas: body.replicas,
        parallelism_strategies: body.parallelism_strategies.clone().and_then(|ref pss| {
            Some(
                pss.iter()
                    .map(|ps| ParallelismStrategy::from(ps.clone()))
                    .collect(),
            )
        }),
        platform: path.platform.clone(),
        strategy_name: path.strategy_name.clone(),
        arguments: body
            .arguments
            .clone()
            .unwrap_or_else(|| vec![])
            .iter()
            .map(|a| Argument::from(a.clone()))
            .collect(),
        deployment_modality: shared::shared_kernel::enums::DeploymentModality::from(
            body.deployment_modality.clone(),
        ),
    };

    let output = match model_deployment_service
        .deploy_model_with_strategy(&ctx, input)
        .await
    {
        Ok(output) => output,
        Err(ModelDeploymentServiceError::MissingModel(_))
        | Err(ModelDeploymentServiceError::MissingExternalModel(_)) => {
            return build_error_response(404, "Model not found".into())
        }
        Err(ModelDeploymentServiceError::InvalidStrategy(message)) => {
            return build_error_response(400, message)
        }
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let resp = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(Some(resp), None, None)
}
