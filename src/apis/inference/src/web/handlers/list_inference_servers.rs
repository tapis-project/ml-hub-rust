use crate::bootstrap::factories::inference_server_repo_factory;
use crate::bootstrap::state::AppState;
use crate::application::inputs;
use crate::application::services::inference_server_service::InferenceServerService;
use crate::web::dto::{
    ListAll,
    InferenceServer as InferenceServerDto
};
use crate::web::helpers::{
    build_error_response,
    build_success_response
};
use actix_web::{
    web as act_web,
    get,
    Responder,
    // HttpRequest as ActixHttpRequest
};
use log::debug;
use serde_json::Value;

#[get("/inference-api/inference-servers")]
async fn list_inference_servers(
    // req: ActixHttpRequest,
    // body: act_web::Bytes,
    query: Option<act_web::Query<ListAll>>,
    data: act_web::Data<AppState>
) -> impl Responder {
    debug!("Operation list_inference_servers");

    let maybe_input = query
        .map(|q| q.into_inner())
        .map(|inner| {
            inputs::ListAll::try_from(inner)
                .map_err(|err| build_error_response(500, err.to_string()))
        })
        .transpose();

    let input = match maybe_input {
        Ok(inp) => inp,
        Err(response) => return response
    };

    let service = InferenceServerService::new(
        inference_server_repo_factory(data.db.clone())
    );

    let inference_servers = match service.list_all(input).await {
        Ok(inference_servers) => inference_servers,
        Err(err) => {
            return build_error_response(
                500,
                String::from(format!("Error creating inference server: {}", err.to_string()))
            );
        }
    };

    let inference_server_count = inference_servers.len();
    if inference_server_count == 0 {
        return build_success_response(Some(Value::Array(Vec::new())), None, None)
    }

    // Convert inference server domain entities into response dtos
    let mut dtos: Vec<InferenceServerDto> = Vec::with_capacity(inference_server_count);
    for inference_server in inference_servers {
        let inference_server_dto = match InferenceServerDto::try_from(inference_server) {
            Ok(dto) => dto,
            Err(err) => {
                return build_error_response(
                    500,
                    String::from(format!("Error converting inference server data: {}", err.to_string()))
                )
            }
        };
        dtos.push(inference_server_dto)
    }

    let mut serialized_dtos: serde_json::Value = Value::Array(Vec::with_capacity(inference_server_count));
    for dto in dtos {
        let serialized_dto = match serde_json::to_value(dto) {
            Ok(serialized_dto) => serialized_dto,
            Err(err) => {
                return build_error_response(
                    500,
                    String::from(format!("Error serializing inference server: {}", err.to_string()))
                )
            }
        };

        if let Value::Array(ref mut arr) = serialized_dtos {
            arr.push(serialized_dto);
        }
    }

    return build_success_response(
        Some(serialized_dtos),
        Some(200),
        Some(String::from("success"))
    );
}
