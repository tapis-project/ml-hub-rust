use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::{
    DeployModelWithStrategyBody,
    DeployModelWithStrategyPathParams,
};
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{
    post,
    web,
    Responder,
};
use serde_json::to_value;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::shared_kernel::context::RequestContext;
use shared::application::inputs::deployment::DeployWithStrategyInput;
use shared::application::inputs::common::Scope as ScopeInput;

#[utoipa::path(
    post,
    path="/deployments-api/platforms/{platform}/strategies/{strategy_name}",
    tag="Deployments",
    description="Deploy a model to a target platform",
    request_body=DeployModelWithStrategyBody,
    params(
        DeployModelWithStrategyPathParams,
    ),
    responses(
        (status=200, description="Model deployment", body=contracts::responses::ModelDeploymentResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
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
        model_author: body.model_author.clone(),
        model_name: body.model_name.clone(),
        model_scope: ScopeInput::from(body.scope.clone()),
        platform: path.platform.clone(),
        strategy_name: path.strategy_name.clone(),
        params: body.params.clone(),
        deployment_modality: shared::shared_kernel::enums::DeploymentModality::from(body.deployment_modality.clone()),
    };

    let output = match model_deployment_service.deploy_model_with_strategy(input, &ctx).await {
        Ok(output) => output,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let resp = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };
    
    build_success_response(Some(resp), None, None)
}