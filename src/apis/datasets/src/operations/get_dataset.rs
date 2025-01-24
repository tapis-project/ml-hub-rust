use serde_json::{Value, Map, from_str};
use actix_web::{web, get, HttpResponse, Responder};
use crate::config;
use log::debug;
use huggingface_client::client::HuggingFaceClient;
use huggingface_client::requests:: GetDatasetRequest;
use shared::responses::{ResponseBuilder, Response};
use shared::clients::{ApiClient, DatasetsClient};

#[get("/datasets/{dataset_id:.*}")]
async fn get_dataset(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_dataset");
    debug!("Fetching dataset: {}", path);

    // Determine which client to use based on the user-specified dataset source
    // TODO

    // Initialize a HuggingFace client
    let client = HuggingFaceClient::new();

    // Fetch the list of datasets
    let result = client.get_dataset(
        GetDatasetRequest {
            dataset_id: path.to_string()
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
                .unwrap();
            
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
            });

            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(resp)
        },
    }
}