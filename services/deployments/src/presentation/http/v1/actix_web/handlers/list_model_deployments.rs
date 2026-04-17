use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::bootstrap::state::AppState;
use crate::bootstrap::factories::model_deployment_service_builder;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::responses::ModelDeployment;
use actix_web::{
    get,
    web,
    Responder,
};
use serde_json::to_value;
use shared::application::identity_context::IdentityContext;

#[utoipa::path(
    get,
    path="/deployments-api/deployments",
    tag="Deployments",
    description="List model deployments for the current tenant",
    responses(
        (status=200, description="List of model deployments", body=contracts::responses::ListModelDeploymentsResponse),
        (status=400, description="Bad request", body=contracts::responses::BadRequestResponse),
        (status=500, description="Internal server error", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("deployments-api/deployments")]
async fn list_model_deployments(
    data: web::Data<AppState>,
    identity_context: IdentityContext,
) -> impl Responder {
    let service = model_deployment_service_builder(
        &data.client,
        data.db_name.clone(),
        data.channel.clone(),
    );

    let output = match service.list_model_deployments(&identity_context).await {
        Ok(output) => output,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let deployments: Vec<ModelDeployment> = output.deployments
        .into_iter()
        .map(ModelDeployment::from)
        .collect();

    let resp = match to_value(deployments) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(Some(resp), None, None)
}
