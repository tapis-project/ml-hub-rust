use clients::{ClientError, ClientErrorScope};
use reqwest::blocking::Response;
use serde_json::{from_str, Value};

pub(crate) fn deserialize_response_body(response: Response) -> Result<Value, ClientError> {
    response
        .text()
        .map_err(|err| ClientError::Internal {
            msg: err.to_string(),
            scope: ClientErrorScope::Client,
        })
        .and_then(|text| {
            from_str::<Value>(&text.trim()).map_err(|err| ClientError::Internal {
                msg: format!("Error deserializing response: {}", err.to_string()),
                scope: ClientErrorScope::Client,
            })
        })
}
