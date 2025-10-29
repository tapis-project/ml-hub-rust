use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::DiscoveryCriteria;
use crate::bootstrap::factories::model_metadata_service_factory;
use actix_web::{post, web, Responder};
use shared::logging::SharedLogger;
use crate::application::discover_model_inputs as inputs;
use crate::presentation::http::v1::contracts;
use crate::presentation::http::v1::requests::ModelMetadata;
use crate::bootstrap::state::AppState;
use serde_json::{to_value, Value};

#[utoipa::path(
    post,
    path = "/models-api/models/search",
    tag="Models",
    description="Discover models known to MLHub",
    request_body=DiscoveryCriteria,
    responses(
        (status=200, description="Discovered models", body=contracts::responses::DiscoverModelsResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[post("models-api/models")]
async fn discover_models(
    data: web::Data<AppState>,
    body: web::Json<DiscoveryCriteria>,
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("discover_models operation");

    let model_metadata_service = match model_metadata_service_factory(&data.db).await {
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

    let input = inputs::DiscoverModelsInput {
        confidence: discovery_criteria.confidence_threshold,
        criteria
    };

    let metadata_entries = match model_metadata_service.discover_models(input).await {
        Ok(e) => e,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let mut values: Vec<Value> = Vec::with_capacity(metadata_entries.len());
    for metadata_entity in metadata_entries {
        let metadata = match ModelMetadata::try_from(metadata_entity) {
            Ok(m) => m,
            Err(err) => return build_error_response(500, err.to_string())
        };

        match to_value(metadata) {
            Ok(v) => values.push(v),
            Err(err) => return build_error_response(500, err.to_string())
        }
    }

    let resp = match to_value(values) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(Some(resp), Some(String::from("success")), None)
}