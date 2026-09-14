use actix_web::{get, web, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::{
        inputs::model::ListModelsInput, services::model_query_service::ModelQueryService,
    },
    presentation::http::v1::{
        contracts::responses::ListModelsResponse,
        requests::models::{ListModelsQueryParams, ModelScope},
        responses::models::Model,
    },
    shared_kernel::context::RequestContext,
};

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    get,
    path = "/models-api/models",
    tag = "Models",
    summary = "List owned or shared models",
    params(ListModelsQueryParams),
    responses(
        (
            status = 200,
            description = "Models listed",
            body = ListModelsResponse
        ),
        (
            status = 500,
            description = "Unable to list models"
        ),
    ),
)]
#[get("models-api/models")]
pub async fn list_models(
    query: web::Query<ListModelsQueryParams>,
    ctx: RequestContext,
    service: web::Data<ModelQueryService>,
) -> impl Responder {
    let input = ListModelsInput::from(&*query);

    let output = match query.scope {
        ModelScope::Owned => service.list_owned(&ctx, &input).await,
        ModelScope::Shared => service.list_shared(&ctx, &input).await,
    };

    let output = match output {
        Ok(output) => output,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(
        output
            .models
            .into_iter()
            .map(Model::from)
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
        Some("Successfully listed models".into()),
        Some(Value::Object(metadata)),
    )
}
