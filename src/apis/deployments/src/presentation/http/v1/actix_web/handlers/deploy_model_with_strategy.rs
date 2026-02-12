use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_deployment_service_builder;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::{
    DeployModelWithStrategyBody,
    DeployModelWithStrategyPathParams
};
use platforms::Platform;
use actix_web::{
    post,
    web,
    Responder
};
use serde_json::{Value, to_value};
use shared::application::inputs::deployment::DeployWithStrategyInput;

#[utoipa::path(
    post,
    path="/deployments-api/platforms/{platform}/strategies/{strategy_name}",
    tag="Strategies",
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
) -> impl Responder {
    let service = model_deployment_service_builder(
        &data.db,
        &data.message_publisher_connection_args.host,
        data.message_publisher_connection_args.port.clone(),
        &data.message_publisher_connection_args.username,
        &data.message_publisher_connection_args.password,
    );

    let input = DeployWithStrategyInput {
        owner: "mlhub".into(),
        model_author: body.model_author.clone(),
        model_name: body.model_name.clone(),
        platform: path.platform.clone(),
        strategy_name: body.strategy_name.clone(),
        params: body.params.clone(),
    };

    match service.deploy_model_with_strategy(input).await {
        Ok(_) => build_success_response(None, None, None),
        Err(err) => build_error_response(500, err.to_string()),
    }
}