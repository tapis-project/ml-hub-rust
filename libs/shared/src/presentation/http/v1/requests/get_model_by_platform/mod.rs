use std::collections::HashMap;

use super::common::headers::Headers;

pub mod path;

pub struct GetModelByPlatformRequest {
    pub headers: Headers,
    pub path: path::GetModelByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: bytes::Bytes,
}