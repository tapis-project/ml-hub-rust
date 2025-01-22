use serde_json::{Value, Map, from_str};
use actix_web::{web, get, HttpResponse, Responder};
use crate::config;
use log::debug;
use huggingface_client::client::{
    HuggingFaceClient,
    GetModelRequest,
    QueryParameters
};
use shared::responses::{ResponseFactory, Response};

#[get("/models/{model_id:.*}")]
async fn get_model(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_model");
    debug!("Fetching model: {}", path);

    // Determine which client to use based on the user-specified model source
    // TODO Perhaps this a job for some middleware

    // Initialize a HuggingFace client
    let client = HuggingFaceClient::new();

    // Fetch the list of models
    let result = client.get_model(
        GetModelRequest {
            path: path.to_string(),
            query_params: Some(QueryParameters {
                search: None,
                author: None,
                filter: None,
                sort: None,
                direction: None,
                limit: Some(10),
                full: None,
                config: None,
            })
        }
    );

    // Initialize response factory
    let response_factory = ResponseFactory::new();

    match result {
        Ok(response) => {
            let response_text = response
                .text()
                .unwrap_or_default();

            let body: Value = from_str(&response_text.trim())
                .unwrap();
            
            let resp = response_factory.build(
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
            let resp = response_factory.build(
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