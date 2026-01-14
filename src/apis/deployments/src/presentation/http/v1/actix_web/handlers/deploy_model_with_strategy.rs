use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::bootstrap::state::AppState;
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

#[utoipa::path(
    post,
    path="/deployments-api/platforms/{platform}/strategies/{strategy_name}",
    tag="Strategies",
    description="Deploy a model to a target platform",
    request_body=DeployModelWithStrategyBody,
    path=(
        ("platform" = Platform, Path, description = "The target platform for the Model Deployment")
    ),
    responses(
        (status=200, description="Model deployment", body=contracts::responses::ListDeploymentStrategiesResponse),
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
    build_success_response(None, Some("Success".into()), None)
}