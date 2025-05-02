use crate::config::VERSION;
use crate::bootstrap::factories::inference_server_repo_factory;
use crate::bootstrap::state::AppState;
use crate::application::services::inference_server_service::InferenceServerService;
use crate::application::inputs;
use crate::web::dto;
use crate::web::helpers::build_error_response;
use shared::errors::Error;
use actix_web::{web, post, HttpResponse, Responder, HttpRequest as ActixHttpRequest};
use shared::responses::JsonResponse;
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
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(400),
                    message: Some(String::from(format!("Yaml deserialization error: {}", err.to_string()))),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string()),
                });
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
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(String::from(format!("Error creating inference server: {}", err.to_string()))),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string()),
                })
        }
    };

    let dto = match dto::InferenceServer::try_from(inference_server) {
        Ok(dto) => dto,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(String::from(format!("Error converting inference server data: {}", err.to_string()))),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string()),
                })
        }
    };

    match serde_json::to_value(dto) {
        Ok(result) => {
            return HttpResponse::Ok()
            .content_type("application/json")
            .json(JsonResponse {
                status: Some(200),
                message: Some(String::from("success")),
                result: Some(serde_json::to_value(result).unwrap()),
                metadata: None,
                version: Some(VERSION.to_string()),
            })
        },
        Err(err) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(200),
                    message: Some(String::from(format!("Error serializing inference server: {}", err.to_string()))),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string()),
                })
        }
    }
}
