pub trait ApiClient {
    fn new() -> Self;
}

pub trait ModelsClient: ApiClient {
    type ListModelsRequest;
    type GetModelRequest;
    type Response;
    type Err;

    fn list_models(&self, request: Self::ListModelsRequest) -> Result<Self::Response, Self::Err>;

    fn get_model(&self, request: Self::GetModelRequest) -> Result<Self::Response, Self::Err>;
}
pub trait DatasetsClient: ApiClient {
    type ListDatasetsRequest;
    type GetDatasetRequest;
    type Response;
    type Err;

    fn list_datasets(
        &self,
        request: Self::ListDatasetsRequest,
    ) -> Result<Self::Response, Self::Err>;

    fn get_dataset(&self, request: Self::GetDatasetRequest) -> Result<Self::Response, Self::Err>;
}
