use serde_json::{Value, Map, from_str};
use actix_web::{get, web, HttpResponse, Responder};
use crate::config;
use log::debug;
use huggingface_client::{
    client::HuggingFaceClient,
    requests::{
        ListDatasetsRequest,
        ListDatasetsQueryParameters
    }
};
use shared::{
    responses::{ResponseBuilder, Response},
    clients::{ApiClient, DatasetsClient}
};

#[get("/datasets")]
async fn list_datasets(
    query: web::Query<ListDatasetsQueryParameters>
) -> impl Responder {
    debug!("Operation list_datasets");

    // Determine which client to use based on the user-specified dataset source
    // TODO Perhaps this a job for some middleware

    // Initialize a HuggingFace client
    let client = HuggingFaceClient::new();

    // Fetch the list of datasets
    let query_params = Some(
        ListDatasetsQueryParameters {
            search: query.search.clone(),
            author: query.author.clone(),
            filter: query.filter.clone(),
            sort: query.sort.clone(),
            direction: query.direction.clone(),
            limit: query.limit.clone(),
            full: query.full.clone(),
        }
    );
    let result = client.list_datasets(
        ListDatasetsRequest {
            query_params
        }
    );

    // Initialize response builder
    let response_builder = ResponseBuilder::new();

    match result {
        Ok(response) => {
            let response_text = response
                .text()
                .unwrap_or_default();

            let body: Value = from_str(&response_text.trim())
                .unwrap_or_default();
            
            let resp = response_builder.build(
                true,
                Response {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: Some(body),
                    version: Some(String::from(config::VERSION)),
                    metadata: Some(Value::Object(Map::new())),
                }
            );

            HttpResponse::Ok()
                .content_type("application/json")
                .json(resp)
        },

        Err(err) => {
            let resp = response_builder.build(
                false,
                Response {
                    status: Some(500),
                    message: Some(err.to_string()),
                    result: None,
                    version: Some(String::from(config::VERSION)),
                    metadata: Some(Value::Object(Map::new())),
                }
            );

            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(resp)
        },
    }
}