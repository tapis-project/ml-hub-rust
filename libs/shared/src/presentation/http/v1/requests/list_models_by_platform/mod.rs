pub mod path;

use std::collections::HashMap;

use crate::presentation::http::v1::requests::common::headers::Headers;

pub struct ListModelsByPlatformRequest {
    pub headers: Headers,
    pub path: path::ListModelsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: bytes::Bytes,
}