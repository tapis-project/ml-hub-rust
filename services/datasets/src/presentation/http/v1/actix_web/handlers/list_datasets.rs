use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    contracts::responses::ListDatasetsResponse,
    requests::{ListDatasetsQueryParams, Scope},
    responses::Dataset,
};
use actix_web::{get, web, Responder};
use serde_json::{to_value, Map, Value};
use shared::{
    application::{inputs::dataset::ListDatasetsInput, services::dataset_service::DatasetService},
    shared_kernel::context::RequestContext,
};

#[utoipa::path(
    get,
    path = "/datasets-api/datasets",
    tag = "Datasets",
    summary = "List datasets with at most the first 50 items from each",
    params(ListDatasetsQueryParams),
    responses(
        (status = 200, description = "Datasets listed", body = ListDatasetsResponse),
        (status = 500, description = "Unable to list datasets"),
    )
)]
#[get("datasets-api/datasets")]
pub async fn list_datasets(
    query: web::Query<ListDatasetsQueryParams>,
    ctx: RequestContext,
    service: web::Data<DatasetService>,
) -> impl Responder {
    let input = ListDatasetsInput::from(&*query);

    let output = match query.scope {
        Scope::Owned => service.list_for_user(&ctx, &input).await,
        Scope::Shared => service.list_shared_with_user(&ctx, &input).await,
        Scope::Global => service.list_global(&ctx, &input).await,
    };

    let output = match output {
        Ok(v) => v,
        Err(e) => return build_error_response(500, e.to_string()),
    };

    let response = match to_value(
        output
            .datasets
            .into_iter()
            .map(Dataset::from)
            .collect::<Vec<_>>(),
    ) {
        Ok(v) => v,
        Err(e) => return build_error_response(500, e.to_string()),
    };

    let metadata = list_metadata(output.cursor, output.count);

    build_success_response(
        Some(response),
        Some("Successfully listed datasets".into()),
        Some(metadata),
    )
}

pub fn list_metadata(cursor: Option<String>, count: Option<u64>) -> Value {
    let mut metadata = Map::new();

    if let Some(cursor) = cursor {
        metadata.insert("cursor".into(), Value::String(cursor));
    }

    if let Some(count) = count {
        metadata.insert("count".into(), Value::Number(count.into()));
    }

    Value::Object(metadata)
}
