pub mod path;

use std::collections::HashMap;

use bytes::Bytes;

use crate::presentation::http::v1::requests::common::headers::Headers;

pub struct GetDatasetByPlatformRequest {
    pub headers: Headers,
    pub path: path::GetDatasetByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}
