use serde_json::{Value, Map, from_str};
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::config;
use log::debug;
use huggingface_client::requests::{
    ListModelsRequest,
    ListModelsQueryParameters
};
use shared::responses::{ResponseBuilder, Response};
use shared::clients::ModelsClient;

#[get("/models")]
async fn list_models(req: HttpRequest) -> impl Responder {
    debug!("Operation list_models");

    // Fetch the client for listing models
    let mut client = req.extensions().get::<ModelsClient>().unwrap();

    // Fetch the list of models
    let result = client.list_models(
        ListModelsRequest {
            query_params: Some(ListModelsQueryParameters {
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