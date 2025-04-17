pub(crate) mod files {
    use crate::utils::build_operation_url;
    use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;
    use std::collections::hash_map::HashMap;
    use shared::errors::Error;
    use reqwest::header::{HeaderMap, HeaderValue};
    use reqwest::blocking::{Client, Response};
    use reqwest::blocking::multipart::{Form, Part};
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Debug, Deserialize)]
    pub struct MkdirResponse {
        pub _status: String,
        pub message: String,
        pub _result: String,
        pub _version: String,
        pub _commit: String,
        pub _build: String,
        pub _metadata: HashMap<String, Value>
    }

    pub fn mkdir(tenant: String, system_id: String, path: String, token: Option<String>) -> Result<Response, Error> {
        let client = Client::new();

        let url = build_operation_url(
            tenant,
            String::from("files"),
            Some(format!(
                "{}/{}",
                String::from("ops"),
                system_id
            ),
        ));

        let body = HashMap::new()
            .insert("path", path);

        let request = client.post(url)
            .form(&body);
        
        // Add X-Tapis-Token header and value
        let mut headers = HeaderMap::new();
        if let Some(value) = token {
            HeaderValue::from_str(value.as_str())
                .map_err(|err| Error::new(err.to_string()))
                .map(|value| {
                    headers.insert("X-Tapis-Token", value);
                })?;
        }
        
        // Add token header
        request.headers(headers)
            .send()
            .map_err(|err| Error::new(err.to_string()))
    }

    pub async fn insert(
        tenant: String,
        system_id: String,
        source_path: String,
        target_path: String,
        token: Option<String>
    ) -> Result<Response, Error> {
        let url = build_operation_url(
            tenant,
            String::from("files"),
            Some(format!(
                "{}/{}/{}",
                String::from("ops"),
                system_id,
                target_path.strip_prefix("/")
                    .unwrap_or(&target_path)
            ),
        ));

        let file = File::open(&source_path)
            .map_err(|err| Error::new(err.to_string()))?;

        let file_name = Path::new(&target_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();

        let reader = BufReader::with_capacity(10 * 1024 * 1024, file);

        // Create the multipart form with a file stream
        let part = Part::reader(reader)
            .file_name(file_name);
        
        let form = Form::new().part("file", part);

        let client = Client::new();

        let request = client.post(url)
            .multipart(form);
        
        // Add X-Tapis-Token header and value
        let mut headers = HeaderMap::new();
        if let Some(value) = token {
            HeaderValue::from_str(value.as_str())
                .map_err(|err| Error::new(err.to_string()))
                .map(|value| {
                    headers.insert("X-Tapis-Token", value);
                })?;
        }
        
        // Add token header
        request.headers(headers)
            .send()
            .map_err(|err| Error::new(err.to_string()))
    }
}