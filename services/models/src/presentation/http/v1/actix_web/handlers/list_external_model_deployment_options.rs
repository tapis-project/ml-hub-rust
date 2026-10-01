use actix_web::{get, web, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::{
        inputs::deployment_option::ListDeploymentOptionsInput,
        ports::deployment_option::DeploymentOptionRepositoryError,
        services::deployment_option_query_service::{
            DeploymentOptionQueryService, DeploymentOptionQueryServiceError,
        },
    },
    domain::entities::model::external_model::ExternalModelId,
    presentation::http::v1::{
        contracts::responses::{self, ListExternalModelDeploymentOptionsResponse},
        requests::deployment_options::{
            ListDeploymentOptionsQuery, ListExternalModelDeploymentOptionsPath,
        },
        responses::deployment_options::DeploymentOption,
    },
    shared_kernel::context::RequestContext,
};
use uuid::Uuid;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/models-api/external-models/{external_model_id}/deployment-options",
    tag = "ExternalModels",
    summary = "List deployment options for an external model",
    params(
        ListExternalModelDeploymentOptionsPath,
        ListDeploymentOptionsQuery
    ),
    responses(
        (
            status = 200,
            description = "Deployment options listed",
            body = ListExternalModelDeploymentOptionsResponse
        ),
        (
            status = 400,
            description = "Invalid external model identifier or cursor",
            body = responses::BadRequestResponse
        ),
        (
            status = 404,
            description = "ExternalModel not found",
            body = responses::NotFoundResponse
        ),
        (
            status = 500,
            description = "Unable to list deployment options",
            body = responses::ServerErrorResponse
        ),
    ),
)]
#[get("models-api/external-models/{external_model_id}/deployment-options")]
pub async fn list_external_model_deployment_options(
    path: web::Path<String>,
    query: web::Query<ListDeploymentOptionsQuery>,
    ctx: RequestContext,
    service: web::Data<DeploymentOptionQueryService>,
) -> impl Responder {
    let external_model_id = match Uuid::parse_str(path.as_str()) {
        Ok(id) => ExternalModelId::reconstitute(id),
        Err(_) => return build_error_response(400, "Invalid ExternalModel identifier".into()),
    };

    let input = ListDeploymentOptionsInput::from(query.into_inner());

    let output = match service
        .list_external_model_deployment_options(&ctx, &external_model_id, &input)
        .await
    {
        Ok(output) => output,
        Err(DeploymentOptionQueryServiceError::ExternalModelNotFound) => {
            return build_error_response(404, "ExternalModel not found".into())
        }
        Err(DeploymentOptionQueryServiceError::DeploymentOptionRepository(
            DeploymentOptionRepositoryError::InvalidCursor,
        )) => return build_error_response(400, "Invalid deployment option cursor".into()),
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(
        output
            .deployment_options
            .into_iter()
            .map(DeploymentOption::from)
            .collect::<Vec<_>>(),
    ) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let mut metadata = Map::new();

    if let Some(cursor) = output.cursor {
        metadata.insert("cursor".into(), Value::String(cursor));
    }

    if let Some(count) = output.count {
        metadata.insert("count".into(), Value::Number(count.into()));
    }

    build_success_response(
        Some(result),
        Some("Successfully listed deployment options".into()),
        Some(Value::Object(metadata)),
    )
}
