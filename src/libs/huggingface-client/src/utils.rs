use clients::{ClientError, ClientErrorScope};
use serde_json::{Value, from_str};
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
                        msg: err.to_string(),
                        scope: ClientErrorScope::Client
                    }
                })
        })
}