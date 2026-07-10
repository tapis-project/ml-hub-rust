use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{DiscoveryCriteria, DiscoverModelsQueryParams};
use actix_web::{post, web, Responder};
use shared::shared_kernal::identity::IdentityContext;
use shared::application::services::model_metadata_service::ModelMetadataService;
use crate::application::discover_model_inputs as inputs;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::responses::ModelMetadata;
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
        ("include_global_models" = Option<bool>, Query, description = "A flag for including global models in the response"),
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
    body: web::Json<DiscoveryCriteria>,
    query: web::Query<DiscoverModelsQueryParams>,
    identity_context: IdentityContext,
    model_metadata_service: web::Data<ModelMetadataService>,
) -> impl Responder {
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
        query.include_count,
        query.include_global_models,
    );

    let input = inputs::SearchModelsInput {
        criteria,
        options
    };

    let output = match model_metadata_service.discover_models(input, &identity_context).await {
        Ok(e) => e,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let metadata_entries = output.models;

    let mut values: Vec<Value> = Vec::with_capacity(metadata_entries.len());
    for metadata_entity in metadata_entries {
        let model_metadata = match ModelMetadata::try_from(&metadata_entity) {
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