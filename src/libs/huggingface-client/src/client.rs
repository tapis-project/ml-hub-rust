#![allow(unused_imports)]

use crate::constants;
use reqwest::Error;
use reqwest::blocking::{Client, ClientBuilder, Response};

pub struct HuggingFaceClient {
    client: Client
}

pub struct Model {}

impl HuggingFaceClient {
    pub fn new() -> Self {
        let client = Client::new();
        Self {
            client
        }
    }

    fn format_url(&self, url: &str) -> String {
        format!(
            "{}/{}",
            constants::HUGGING_FACE_BASE_URL,
            url.strip_prefix("/").unwrap_or(url).to_string()
        )
    }

    pub fn list_models(&self) -> Result<Response, Error> {
        self.client
            .get(self.format_url("models"))
            .send()
    }
}