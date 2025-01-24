use crate::constants;
use crate::requests::{
    GetDatasetRequest, GetModelRequest, ListDatasetsRequest, ListModelsRequest,
};
use reqwest::blocking::{Client, Response};
use reqwest::Error;
use shared::clients::{ApiClient, DatasetsClient, ModelsClient};

#[derive(Debug)]
pub struct HuggingFaceClient {
    client: Client,
}

impl ApiClient for HuggingFaceClient {
    fn new() -> Self {
        let client = Client::new();
        Self { client }
    }
}

impl DatasetsClient for HuggingFaceClient {
    type ListDatasetsRequest = ListDatasetsRequest;
    type GetDatasetRequest = GetDatasetRequest;
    type Response = Response;
    type Err = Error;

    fn list_datasets(
        &self,
        request: Self::ListDatasetsRequest,
    ) -> Result<Self::Response, Self::Err> {
        let result = self.client
            .get(self.format_url("datasets"))
            .query(&request.query_params)
            .send();

        println!("{:#?}", &result);
        return result;
    }

    fn get_dataset(
        &self,
        request: Self::GetDatasetRequest,
    ) -> Result<Self::Response, Self::Err> {
        self.client
            .get(self.format_url(
                format!("{}/{}", "datasets", request.dataset_id.as_str()).as_str(),
            ))
            .send()
    }
}

impl ModelsClient for HuggingFaceClient {
    type ListModelsRequest = ListModelsRequest;
    type GetModelRequest = GetModelRequest;
    type Response = Response;
    type Err = Error;

    fn list_models(
        &self,
        request: ListModelsRequest,
    ) -> Result<Response, Error> {
        self.client
            .get(self.format_url("models"))
            .query(&request.query_params)
            .send()
    }

    fn get_model(&self, request: GetModelRequest) -> Result<Response, Error> {
        self.client
            .get(self.format_url(
                format!("{}/{}", "models", request.model_id.as_str()).as_str(),
            ))
            .send()
    }
}

impl HuggingFaceClient {
    fn format_url(&self, url: &str) -> String {
        format!(
            "{}/{}",
            constants::HUGGING_FACE_BASE_URL,
            url.strip_prefix("/").unwrap_or(url).to_string()
        )
    }
}
