use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_deployment_service_builder;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::{
    DeployModelWithStrategyBody,
    DeployModelWithStrategyPathParams
};
use crate::presentation::http::v1::responses::ModelDeployment;
use platforms::Platform;
use actix_web::{
    post, web, HttpMessage, HttpRequest, Responder
};
use serde_json::to_value;
use shared::application::inputs::deployment::DeployWithStrategyInput;
use shared::domain::entities::principal::Principal;

#[utoipa::path(
    post,
    path="/deployments-api/platforms/{platform}/strategies/{strategy_name}",
    tag="Deployments",
    description="Deploy a model to a target platform",
    request_body=DeployModelWithStrategyBody,
    params(
        ("platform" = Platform, Path, description = "The target platform for the Model Deployment")
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
    data: web::Data<AppState>,
    body: web::Json<DeployModelWithStrategyBody>,
    path: web::Path<DeployModelWithStrategyPathParams>,
    principal: Principal,
) -> impl Responder {
    let maybe_strategy = data.client_strategy_sets
        .iter()
        .find(|set| set.platform == path.platform.clone())
        .map(|css| css.strategies())
        .map(|strats| strats.iter().find(|strat| &strat.name == &path.strategy_name))
        .map(|x| x.cloned())
        .flatten();

    let strategy = match maybe_strategy {
        Some(s) => s,
        None => return build_error_response(404, format!("No strategy found with name {} for platform {}", &path.platform, &path.strategy_name))
    };

    let service = model_deployment_service_builder(
        &data.client,
        data.db_name.clone(),
        data.channel.clone(),
    );

    let input = DeployWithStrategyInput {
        owner: "mlhub".into(),
        model_author: body.model_author.clone(),
        model_name: body.model_name.clone(),
        platform: path.platform.clone(),
        strategy_name: strategy.name,
        params: body.params.clone(),
    };

    let output = match service.deploy_model_with_strategy(input).await {
        Ok(output) => output,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let resp = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };
    
    build_success_response(Some(resp), None, None)
}