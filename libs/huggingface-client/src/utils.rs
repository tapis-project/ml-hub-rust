use clients::{ClientError, ClientErrorScope, ClientJsonResponse};
use serde_json::{from_str, Map, Value};
use reqwest::Response;

pub(crate) async fn deserialize_response_body(response: Response) -> Result<Value, ClientError> {
    response
        .text()
        .await
        .map_err(|err| {
            ClientError::Internal {
                msg: err.to_string(),
                scope: ClientErrorScope::Client
            }
        })
        .and_then(|text| {
            from_str::<Value>(&text.trim())
                .map_err(|err| {
                    ClientError::Internal {
                        msg: format!("Error deserializing response body: {}", err.to_string()),
                        scope: ClientErrorScope::Client
                    }
                })
        })
}

pub(crate) async fn build_client_response(response: Response) -> Result<ClientJsonResponse<Value, Value>, ClientError> {
    let status = response.status().as_u16();
    let mut metadata = Map::new();

    metadata.insert("remote_status".into(), Value::from(status));

    match status {
        0..299 => {
            let body = deserialize_response_body(response).await?;
            metadata.insert("remote_message".into(), Value::from("success"));
            Ok(ClientJsonResponse::new(
                Some(200),
                Some(String::from("success")),
                Some(body),
                Some(Value::Object(metadata)),
            ))
        },
        400 => {
            metadata.insert("remote_message".into(), Value::from("Bad Request"));
            Ok(ClientJsonResponse::new(
                Some(200),
                Some(String::from("success")),
                None,
                Some(Value::Object(metadata)),
            ))
        },
        401 => {
            metadata.insert("remote_message".into(), Value::from("Unauthenticated"));
            Ok(ClientJsonResponse::new(
                Some(200),
                Some(String::from("success")),
                None,
                Some(Value::Object(metadata)),
            ))
        },
        403 => {
            metadata.insert("remote_message".into(), Value::from("Forbidden"));
            Ok(ClientJsonResponse::new(
                Some(200),
                Some(String::from("success")),
                None,
                Some(Value::Object(metadata)),
            ))
        },
        400..499 => {
            metadata.insert("remote_message".into(), Value::from("Client Error"));
            Ok(ClientJsonResponse::new(
                Some(200),
                Some(String::from("success")),
                None,
                Some(Value::Object(metadata)),
            ))
        },
        _ => {
            metadata.insert("remote_message".into(), Value::from("Server Error"));
            Ok(ClientJsonResponse::new(
                Some(200),
                Some(String::from("success")),
                None,
                Some(Value::Object(metadata)),
            ))
        }
    }
}