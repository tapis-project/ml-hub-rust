use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_deployment_service_builder;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{
    post,
    web,
    Responder
};
use serde_json::to_value;
use shared::application::inputs::deployment::StopModelDeploymentInput;
use crate::presentation::http::v1::requests::StopModelDeploymentPathParams;

#[utoipa::path(
    post,
    path="/deployments-api/deployments/{deployment_id}/stop",
    tag="Deployments",
    description="Stop a Model Deployment",
    params(
        ("deployment_id" = Uuid, Path, description = "The the id of the Model Deployment")
    ),
    responses(
        (status=200, description="Model deployment", body=contracts::responses::ModelDeploymentResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[post("deployments-api/deployments/{deployment_id}/stop")]
async fn stop_model_deployment(
    data: web::Data<AppState>,
    path: web::Path<StopModelDeploymentPathParams>,
) -> impl Responder {
    let service = model_deployment_service_builder(
        &data.db,
        data.channel.clone(),
    );

    let input = StopModelDeploymentInput {
       owner: "mlhub".into(),
       deployment_id: path.deployment_id,
    };

    let output = match service.stop_model_deployment(input).await {
        Ok(output) => output,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let resp = match to_value(ModelDeployment::from(output.deployment)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };
    
    build_success_response(Some(resp), None, None)
}