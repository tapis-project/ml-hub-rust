use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{DiscoveryCriteria, DisocverModelsQueryParams};
use crate::bootstrap::factories::model_metadata_service_factory;
use actix_web::{post, web, Responder};
use shared::logging::SharedLogger;
use crate::application::discover_model_inputs as inputs;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::ModelMetadata;
use crate::bootstrap::state::AppState;
use serde_json::{to_value, Value, Map};

#[utoipa::path(
    post,
    path = "/models-api/models/search",
    tag="Models",
    description="Discover models on MLHub",
    request_body=DiscoveryCriteria,
    params(
        ("limit" = Option<u16>, Query, description = "The maximum number of models to return"),
        ("cursor" = Option<String>, Query, description = "The pagination cursor for fetching the next batch of models"),
        ("include_count" = Option<bool>, Query, description = "A flag for including the total count of available models"),
    ),
    responses(
        (status=200, description="Discovered models", body=contracts::responses::DiscoverModelsResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[post("models-api/models/search")]
async fn discover_models(
    data: web::Data<AppState>,
    body: web::Json<DiscoveryCriteria>,
    query: web::Query<DisocverModelsQueryParams>
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("discover_models operation");

    let model_metadata_service = match model_metadata_service_factory(&data.db, data.client_strategy_sets.clone()).await {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let discovery_criteria = match DiscoveryCriteria::try_from(body.into_inner()) {
        Ok(c) => c,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let mut criteria: Vec<inputs::SearchCriterion> = Vec::with_capacity(discovery_criteria.criteria.len());
    for criterion in discovery_criteria.criteria {
        let c = match inputs::SearchCriterion::try_from(&criterion) {
            Ok(c) => c,
            Err(err) => return build_error_response(500, err.to_string())
        };
        
        criteria.push(c);
    }

    let options = inputs::SearchOptions::new(
        query.limit,
        query.cursor.clone(),
        query.include_count
    );

    let input = inputs::DiscoverModelsInput {
        confidence: discovery_criteria.confidence_threshold,
        criteria,
        options
    };

    let output = match model_metadata_service.discover_models(input).await {
        Ok(e) => e,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let metadata_entries = output.models;

    let mut values: Vec<Value> = Vec::with_capacity(metadata_entries.len());
    for metadata_entity in metadata_entries {
        let model_metadata = match ModelMetadata::try_from(metadata_entity) {
            Ok(m) => m,
            Err(err) => return build_error_response(500, err.to_string())
        };

        match to_value(model_metadata) {
            Ok(v) => values.push(v),
            Err(err) => return build_error_response(500, err.to_string())
        }
    }

    let resp = match to_value(values) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let mut resp_metadata: Map<String, Value> = Map::new();
    if let Some(cursor) = output.cursor {
        resp_metadata.insert("cursor".into(), Value::String(cursor));
    }

    if let Some(count) = output.count {
        resp_metadata.insert("count".into(), Value::Number(count.into()));
    }

    build_success_response(
        Some(resp),
        Some(String::from("success")),
        Some(Value::Object(resp_metadata))
    )
}