use crate::constants;
use crate::requests::{
    GetDatasetRequest,
    GetModelRequest,
    DownloadModelRequest,
    ListDatasetsRequest,
    ListModelsRequest,
    DownloadDatasetRequest,
};
use reqwest::blocking::{Client as ReqwestClient, Response};
use reqwest::Error;
use shared::clients::{Client, DatasetsClient, ModelsClient};

#[derive(Debug)]
pub struct HuggingFaceClient {
    client: ReqwestClient,
}

impl Client for HuggingFaceClient {}

impl DatasetsClient for HuggingFaceClient {
    type ListDatasetsRequest = ListDatasetsRequest;
    type GetDatasetRequest = GetDatasetRequest;
    type DownloadDatasetRequest = DownloadDatasetRequest;
    type Response = Response;
    type Err = Error;

    fn list_datasets(
        &self,
        request: Self::ListDatasetsRequest,
    ) -> Result<Self::Response, Self::Err> {
        let result = self.client
            .get(Self::format_url("datasets"))
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
            .get(Self::format_url(
                format!("{}/{}", "datasets", request.dataset_id.as_str()).as_str(),
            ))
            .send()
    }

    fn download_dataset(&self, _: DownloadDatasetRequest) -> Result<Self::Response, Self::Err> {
        unimplemented!("Dataset download functionlity not implemented")
    }
}

impl ModelsClient for HuggingFaceClient {
    type ListModelsRequest = ListModelsRequest;
    type GetModelRequest = GetModelRequest;
    type DownloadModelRequest = DownloadModelRequest;
    type Response = Response;
    type Err = Error;

    fn list_models(
        &self,
        request: ListModelsRequest,
    ) -> Result<Self::Response, Self::Err> {
        self.client
            .get(Self::format_url("models"))
            .query(&request.query_params)
            .send()
    }

    fn get_model(&self, request: GetModelRequest) -> Result<Self::Response, Self::Err> {
        self.client
            .get(Self::format_url(
                format!("{}/{}", "models", request.model_id.as_str()).as_str(),
            ))
            .send()
    }

    fn download_model(&self, _: DownloadModelRequest) -> Result<Self::Response, Self::Err> {
        unimplemented!("Dataset download functionlity not implemented")
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
