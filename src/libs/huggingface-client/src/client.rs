use crate::constants;
use crate::requests::{
    // GetDatasetRequest,
    // GetModelRequest,
    // DownloadModelRequest,
    // ListDatasetsRequest as HFListDatasetsRequest,
    // ListModelsRequest as HFListModelsRequest,
    ListModelsQueryParameters,
    ListDatasetsQueryParameters,
    // DownloadDatasetRequest,
};
use crate::utils::derserialize_response_body;
use reqwest::blocking::Client as ReqwestClient;
use shared::clients::{
    ModelsClient,
    DatasetsClient,
    ClientResponse,
    ClientError,
};
use shared::requests::{
    GetModelRequest,
    ListModelsRequest,
    DownloadModelRequest,
    ListDatasetsRequest,
    GetDatasetRequest,
    DownloadDatasetRequest
};
use serde_json::{Value, Map};
use log::{debug, error};


#[derive(Debug)]
pub struct HuggingFaceClient {
    client: ReqwestClient,
}

impl ModelsClient for HuggingFaceClient {
    fn list_models(
        &self,
        request: &ListModelsRequest,
    ) -> Result<ClientResponse, ClientError> {
        // Parse the limit from the query string
        let limit = match request.query.get("limit").cloned() {
            Some(num) => num.parse(),
            None => Ok(10)
        };

        // Build the query parameters
        let query_params = Some(
            ListModelsQueryParameters {
                search: request.query.get("search").cloned(),
                author: request.query.get("author").cloned(),
                filter: request.query.get("filter").cloned(),
                sort: request.query.get("sort").cloned(),
                direction: request.query.get("direction").cloned(),
                limit: Some(limit.unwrap_or(10)),
                full: None,
                config: None,
            }
        );
        
        // Construct the url for the request
        let url = Self::format_url("models");

        debug!("Request url: {}", url);
        debug!("Query Params: {:#?}", &query_params);

        // Make a GET request to Hugging Face to fetch the models
        let result = self.client
            .get(url)
            .query(&query_params)
            .send();
        
        match result {
            Ok(response) => {
                let body = derserialize_response_body(response)?;
                
                Ok(ClientResponse {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: Some(body),
                    metadata: Some(Value::Object(Map::new())),
                })
            },
            
            Err(err) => {
                error!("{:#?}", err);
                return Err(ClientError::new(err.to_string()))
            },
        }
    }
    
    fn get_model(&self, request: &GetModelRequest) -> Result<ClientResponse, ClientError> {
        let result = self.client
            .get(Self::format_url(format!("{}/{}", "models", request.path.model_id).as_str()))
            .send();

        match result {
            Ok(response) => {
                let body = derserialize_response_body(response)?;
                
                Ok(ClientResponse {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: Some(body),
                    metadata: Some(Value::Object(Map::new())),
                })
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(ClientError::new(err.to_string()))
            }
        }
    }

    fn download_model(&self, _: &DownloadModelRequest) -> Result<ClientResponse, ClientError> {
        Err(ClientError::new(String::from("Download model not implemented")))
    }
}

impl DatasetsClient for HuggingFaceClient {
    fn list_datasets(
        &self,
        request: &ListDatasetsRequest,
    ) -> Result<ClientResponse, ClientError> {
        // Parse the limit from the query string
        let limit = match request.query.get("limit").cloned() {
            Some(num) => num.parse(),
            None => Ok(10)
        };

        // Build the query parameters
        let query_params = Some(
            ListDatasetsQueryParameters {
                search: request.query.get("search").cloned(),
                author: request.query.get("author").cloned(),
                filter: request.query.get("filter").cloned(),
                sort: request.query.get("sort").cloned(),
                direction: request.query.get("direction").cloned(),
                limit: Some(limit.unwrap_or(10)),
                full: None,
            }
        );

        // Make a GET request to Hugging Face to fetch the datasets
        let result = self.client
            .get(Self::format_url("datasets"))
            .query(&query_params)
            .send();

        match result {
            Ok(response) => {
                let body = derserialize_response_body(response)?;
                
                Ok(ClientResponse {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: Some(body),
                    metadata: Some(Value::Object(Map::new())),
                })
            },

            Err(err) => {
                error!("{:#?}", err);
                return Err(ClientError::new(err.to_string()))
            },
        }
    }

    fn get_dataset(&self, request: &GetDatasetRequest) -> Result<ClientResponse, ClientError> {
        let result = self.client
            .get(Self::format_url(format!("{}/{}", "datasets", request.path.dataset_id).as_str()))
            .send();

        match result {
            Ok(response) => {
                let body = derserialize_response_body(response)?;

                Ok(ClientResponse {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: Some(body),
                    metadata: Some(Value::Object(Map::new())),
                })
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(ClientError::new(err.to_string()))
            },
        }
    }

    fn download_dataset(&self, _: &DownloadDatasetRequest) -> Result<ClientResponse, ClientError> {
        Err(ClientError::new(String::from("Download dataset not implemented")))
    }
}

impl HuggingFaceClient {
    pub fn new() -> Self {
        let client = ReqwestClient::new();
        Self { client }
    }
    
    fn format_url(url: &str) -> String {
        format!(
            "{}/{}",
            constants::HUGGING_FACE_BASE_URL,
            url.strip_prefix("/").unwrap_or(url).to_string()
        )
    }
}
