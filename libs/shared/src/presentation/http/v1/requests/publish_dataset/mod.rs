pub mod path;

use std::collections::HashMap;

use actix_multipart::Multipart;

use crate::presentation::http::v1::requests::common::headers::Headers;

pub struct PublishDatasetRequest {
    pub headers: Headers,
    pub path: path::PublishDatasetPath,
    pub query: HashMap<String, String>,
    pub payload: Multipart,
}
