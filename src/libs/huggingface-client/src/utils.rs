use clients::ClientError;
use serde_json::{Value, from_str};
use reqwest::blocking::Response;

pub(crate) fn deserialize_response_body(response: Response) -> Result<Value, ClientError> {
    response
        .text()
        .map_err(|err| ClientError::new(err.to_string()))
        .and_then(|text| {
            from_str::<Value>(&text.trim())
                .map_err(|err| ClientError::new(err.to_string()))
        })
}