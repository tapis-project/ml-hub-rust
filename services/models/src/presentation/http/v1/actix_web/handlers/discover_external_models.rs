use actix_web::{post, web, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::{
        inputs::discover_models::{SearchExternalModelsInput, SearchOptions},
        services::external_model_discovery_service::ExternalModelDiscoveryService,
    },
    presentation::http::v1::{
        contracts::responses::DiscoverExternalModelsResponse,
        requests::discover_models::{
            DiscoverExternalModelsBody, DiscoverExternalModelsQueryParams,
        },
        responses::models::ExternalModel,
    },
};

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    post,
    path = "/models-api/external-models/search",
    tag = "ExternalModels",
    summary = "Search the external model catalog",
    params(DiscoverExternalModelsQueryParams),
    request_body = DiscoverExternalModelsBody,
    responses(
        (
            status = 200,
            description = "ExternalModels found",
            body = DiscoverExternalModelsResponse
        ),
        (
            status = 400,
            description = "Invalid search"
        ),
        (
            status = 500,
            description = "Unable to search ExternalModels"
        ),
    ),
)]
#[post("models-api/external-models/search")]
pub async fn discover_external_models(
    body: web::Json<DiscoverExternalModelsBody>,
    query: web::Query<DiscoverExternalModelsQueryParams>,
    service: web::Data<ExternalModelDiscoveryService>,
) -> impl Responder {
    let input = SearchExternalModelsInput {
        criteria: body.criteria.clone().into_iter().map(Into::into).collect(),
        options: SearchOptions::new(query.limit, query.cursor.clone(), query.include_count),
    };

    let output = match service.discover_external_models(&input).await {
        Ok(output) => output,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(
        output
            .external_models
            .into_iter()
            .map(ExternalModel::from)
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
        Some("Successfully searched ExternalModels".into()),
        Some(Value::Object(metadata)),
    )
}
