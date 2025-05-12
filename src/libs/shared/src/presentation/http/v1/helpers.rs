pub use actix_web::HttpRequest;
use crate::presentation::http::v1::dto::Parameters;
use crate::errors::Error;
use std::collections::hash_map::HashMap;

pub fn param_to_string(params: Option<Parameters>, prop: &str) -> Result<Option<String>, Error> {
    return params.unwrap_or_else(HashMap::new)
        .get(prop)
        .map(|value| {
            if value.is_string() {
                return Ok(value.to_string())
            }

            Err(Error::new(String::from("Parameter 'branch' must be a string")))
        })
        .transpose();
}

pub fn get_header_value(header_key: &str, request: &HttpRequest) -> Option<String> {
    request
        .headers()
        .get(header_key)
        .and_then(|value| value.to_str().ok())
        .map(|value| String::from(value))
}