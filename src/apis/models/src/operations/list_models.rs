use serde_json;
use actix_web::{get, HttpResponse, Responder};
// use crate::dtos::model_dto::ModelDto;
use crate::config;
use log::debug;
use huggingface_client::HuggingFaceClient;
use shared::responses::{ResponseFactory, Response};
use shared::wrappers::{JsonObject, JsonValue};

#[get("/models")]
async fn list_models() -> impl Responder {
    debug!("Operation list_models");

    // Determine which client to use based on the user-specified model source
    // TODO Perhaps this a job for some middleware

    // Initialize a HuggingFace client
    let client = HuggingFaceClient::new();

    // Fetch the list of models
    let result = client.list_models();

    // Initialize response factory
    let response_factory = ResponseFactory::new();

    match result {
        Ok(response) => {
            let response_text = response.text().unwrap_or("".to_string());
            let body: JsonValue = serde_json::from_str(&response_text).unwrap();
            
            let resp = response_factory.build(Response {
                success: true,
                status: Some(200),
                message: Some(String::from("success")),
                result: Some(body),
                version: Some(String::from(config::VERSION)),
                metadata: Some(JsonObject::new()),
            });

            HttpResponse::Ok()
                .content_type("application/json")
                .json(resp)
        },

        Err(err) => {
            let resp = response_factory.build(Response {
                success: false,
                status: Some(200),
                message: Some(err.to_string()),
                result: None,
                version: Some(String::from(config::VERSION)),
                metadata: Some(JsonObject::new()),
            });
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(resp)
        },
    }
}