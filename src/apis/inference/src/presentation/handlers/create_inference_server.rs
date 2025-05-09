use crate::bootstrap::factories::inference_server_repo_factory;
use crate::bootstrap::state::AppState;
use crate::application::services::inference_server_service::InferenceServerService;
use crate::application::inputs;
use crate::presentation::dto;
use crate::presentation::helpers::{build_error_response, build_success_response};
use shared::errors::Error;
use actix_web::{web, post, Responder, HttpRequest as ActixHttpRequest};
use serde_json::from_slice as json_from_slice;
use serde_yaml::from_slice as yaml_from_slice;
use log::debug;

#[post("/inference-api/inference-servers")]
async fn create_inference_server(
    req: ActixHttpRequest,
    body: web::Bytes,
    data: web::Data<AppState>
) -> impl Responder {
    debug!("Operation create_inference_server");

    let content_type = req
        .headers()
        .get("Content-Type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/json");

    let inference_server_dto: dto::InferenceServer = match if content_type.contains("yaml") {
        yaml_from_slice::<dto::InferenceServer>(&body)
            .map_err(|err| Error::new(String::from(format!("Deserialization error: {}", err.to_string()))))
    } else {
        json_from_slice::<dto::InferenceServer>(&body)
            .map_err(|err| Error::new(String::from(format!("Deserialization error: {}", err.to_string()))))
    } {
        Ok(dto) => dto,
        Err(err) => {
            return build_error_response(400, String::from(format!("Yaml deserialization error: {}", err.to_string())))
        }
    };

    let service = InferenceServerService::new(
        inference_server_repo_factory(data.db.clone())
    );

    let maybe_input = inputs::CreateInferenceServerInput::try_from(inference_server_dto);
    let input = match maybe_input {
        Ok(input) => input,
        Err(err) => {
            return build_error_response(400, String::from(format!("Bad request error: {}", err.to_string())))
        }
    };

    let inference_server = match service.create(input).await {
        Ok(inference_server) => inference_server,
        Err(err) => {
            return build_error_response(500, String::from(format!("Error creating inference server: {}", err.to_string())))
        }
    };

    let dto = match dto::InferenceServer::try_from(inference_server) {
        Ok(dto) => dto,
        Err(err) => {
            return build_error_response(500, String::from(format!("Error converting inference server data: {}", err.to_string())));
        }
    };

    match serde_json::to_value(dto) {
        Ok(result) => {
            return build_success_response(Some(result), Some(String::from("success")), None)
        },
        Err(err) => {
            return build_error_response(500, String::from(format!("Error serializing inference server: {}", err.to_string())))
        }
    }
}
