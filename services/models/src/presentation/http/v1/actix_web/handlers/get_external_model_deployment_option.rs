use actix_web::{get, web, Responder};
use serde_json::to_value;
use shared::{
    application::services::deployment_option_query_service::{
        DeploymentOptionQueryService, DeploymentOptionQueryServiceError,
    },
    domain::entities::{
        deployment_option::DeploymentOptionId, model::external_model::ExternalModelId,
    },
    presentation::http::v1::{
        contracts::responses::{self, GetExternalModelDeploymentOptionResponse},
        responses::deployment_options::DeploymentOptionDetail,
    },
    shared_kernel::context::RequestContext,
};
use uuid::Uuid;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/models-api/external-models/{external_model_id}/deployment-options/{deployment_option_id}",
    tag = "ExternalModels",
    summary = "Get a deployment option and its required parameters",
    params(
        (
            "external_model_id" = Uuid,
            Path,
            description = "ExternalModel identifier"
        ),
        (
            "deployment_option_id" = Uuid,
            Path,
            description = "DeploymentOption identifier"
        )
    ),
    responses(
        (
            status = 200,
            description = "Deployment option found",
            body = GetExternalModelDeploymentOptionResponse
        ),
        (
            status = 400,
            description = "Invalid identifier",
            body = responses::BadRequestResponse
        ),
        (
            status = 404,
            description = "ExternalModel or DeploymentOption not found",
            body = responses::NotFoundResponse
        ),
        (
            status = 500,
            description = "Unable to get deployment option",
            body = responses::ServerErrorResponse
        ),
    ),
)]
#[get("models-api/external-models/{external_model_id}/deployment-options/{deployment_option_id}")]
pub async fn get_external_model_deployment_option(
    path: web::Path<(String, String)>,
    ctx: RequestContext,
    service: web::Data<DeploymentOptionQueryService>,
) -> impl Responder {
    let (external_model_id, deployment_option_id) = path.into_inner();
    let external_model_id = match Uuid::parse_str(&external_model_id) {
        Ok(id) => ExternalModelId::reconstitute(id),
        Err(_) => return build_error_response(400, "Invalid ExternalModel identifier".into()),
    };

    let deployment_option_id = match Uuid::parse_str(&deployment_option_id) {
        Ok(id) => DeploymentOptionId::reconstitute(id),
        Err(_) => return build_error_response(400, "Invalid DeploymentOption identifier".into()),
    };

    let output = match service
        .get_external_model_deployment_option(&ctx, &external_model_id, &deployment_option_id)
        .await
    {
        Ok(output) => output,
        Err(DeploymentOptionQueryServiceError::ExternalModelNotFound)
        | Err(DeploymentOptionQueryServiceError::DeploymentOptionNotFound) => {
            return build_error_response(404, "Deployment option not found".into());
        }
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(DeploymentOptionDetail::from(output)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(result),
        Some("Successfully retrieved deployment option".into()),
        None,
    )
}
