use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Header = (String, String);

pub type Boundry = String;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Headers(Vec<Header>);

impl Headers {
    pub fn new(headers: Vec<Header>) -> Self {
        Self(headers)
    }

    pub fn into_inner(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        let tuples = self.0.clone();
        for kv in tuples {
            headers.push(kv)
        }

        headers
    }

    // By the http standard, some headers can be set more than once, so
    // we return an optional vector of strings
    pub fn get_all_values(&self, name: &str) -> Option<Vec<String>> {
        let header_values = self
            .0
            .iter()
            .filter(|(k, _)| k == &name)
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>();

        if header_values.len() < 1 {
            return None;
        }

        Some(header_values)
    }

    pub fn get_first_value(&self, name: &str) -> Option<String> {
        let values = self.get_all_values(name);
        if let Some(v) = values {
            return Some(v[0].clone());
        };

        return None;
    }

    pub fn validate_authorization_header(
        &self,
        auth_header_prefix: Option<&str>,
    ) -> Result<(), AuthorizationHeaderError> {
        // get authorization header if there isn't 1 return
        let header_value = match self.get_first_value("Authorization") {
            Some(value) => value,
            None => {
                return Ok(());
            }
        };

        // use auth_header_prefix if one is given else use 'Bearer '
        let prefix = match auth_header_prefix {
            Some(value) => value,
            None => "Bearer ",
        };

        // returns PrefixNotFound error if prefix string not found
        let header_index = match header_value.find(prefix) {
            Some(index) => index,
            None => {
                return Err(AuthorizationHeaderError::PrefixNotFound);
            }
        };

        // prefix should be at index 0 malformed error
        if header_index != 0 {
            return Err(AuthorizationHeaderError::MalformedHeader(header_value));
        }

        if prefix.len() == header_value.len() {
            return Err(AuthorizationHeaderError::NoValue);
        }

        return Ok(());
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationHeaderError {
    #[error("Header name error: {0}")]
    HeaderNameError(String),

    #[error("Malformed header: {0}")]
    MalformedHeader(String),

    #[error("No header value was provided")]
    NoValue,

    #[error("Provided prefix not found")]
    PrefixNotFound,
}