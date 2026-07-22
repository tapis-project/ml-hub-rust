use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use shared::application::services::model_deployment_service::ListModelDeploymentsByAuthorInput;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::shared_kernel::context::RequestContext;
use crate::presentation::http::v1::responses;
use crate::presentation::http::v1::contracts;
use actix_web::{
    get,
    web,
    Responder
};
use serde_json::{Value, to_value};

#[utoipa::path(
    get,
    path="/deployments-api/deployments",
    tag="Deployments",
    description="Lists all available deployments",
    responses(
        (status=200, description="A list of model deployments", body=contracts::responses::ListModelDeploymentsResponse),
        (status=400, description="Bad Request", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not Found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Internal Server Error", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("deployments-api/deployments")]
async fn list_model_deployments(
    service: web::Data<ModelDeploymentService>,
    ctx: RequestContext,
) -> impl Responder {
    let maybe_deployments = service.list_by_author(
        ListModelDeploymentsByAuthorInput { author: ctx.actor_principal_id().clone() }, 
        &ctx
    ).await;

    let model_deployments = match maybe_deployments {
        Ok(d) => d,
        Err(e) => return build_error_response(500, e.to_string())
    };

    let mut values: Vec<Value> = Vec::with_capacity(model_deployments.len());
    for deployment in model_deployments{
        let model_metadata_resp = match responses::ModelDeployment::try_from(deployment) {
            Ok(m) => m,
            Err(err) => return build_error_response(500, err.to_string())
        };

        match to_value(model_metadata_resp) {
            Ok(v) => values.push(v),
            Err(err) => return build_error_response(500, err.to_string())
        };
    }

    let resp = match to_value(values) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(Some(resp), Some("success".into()), None)
}