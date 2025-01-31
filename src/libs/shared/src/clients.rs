use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ClientType {
    Model,
    Dataset,
    Inference,
    Training
}

pub trait Client: Send {}

pub trait ModelsClient: Client {
    type ListModelsRequest;
    type GetModelRequest;
    type DownloadModelRequest;
    type Response;
    type Err;

    fn list_models(&self, request: Self::ListModelsRequest) -> Result<Self::Response, Self::Err>;

    fn get_model(&self, request: Self::GetModelRequest) -> Result<Self::Response, Self::Err>;

    fn download_model(&self, request: Self::DownloadModelRequest) -> Result<Self::Response, Self::Err>;
}

pub trait DatasetsClient: Client {
    type ListDatasetsRequest;
    type GetDatasetRequest;
    type DownloadDatasetRequest;
    type Response;
    type Err;

    fn list_datasets(
        &self,
        request: Self::ListDatasetsRequest,
    ) -> Result<Self::Response, Self::Err>;

    fn get_dataset(&self, request: Self::GetDatasetRequest) -> Result<Self::Response, Self::Err>;

    fn download_dataset(&self, request: Self::DownloadDatasetRequest) -> Result<Self::Response, Self::Err>;
}

pub trait InferenceClient {
    type CreateInferenceRequest;
    type StartInferenceServerRequest;
    type RunInferenceRequest;
    type Response;
    type Err;
    
    fn create_inference(&self, request: Self::CreateInferenceRequest) -> Result<Self::Response, Self::Err>;
    fn start_inference_server(&self, request: Self::StartInferenceServerRequest) -> Result<Self::Response, Self::Err>;
    fn run_inference(&self, request: Self::RunInferenceRequest) -> Result<Self::Response, Self::Err>;
}

pub trait TrainingClient {
    type CreateTrainingRequest;
    type StartTrainingRequest;
    type Response;
    type Err;
    
    fn create_training(&self, request: Self::CreateTrainingRequest) -> Result<Self::Response, Self::Err>;
    fn start_training(&self, request: Self::StartTrainingRequest) -> Result<Self::Response, Self::Err>;
}

#[derive(Clone)]
pub struct PlatformClientRegistrar {
    pub registries: HashMap<String, HashMap<ClientType, Arc<Mutex<Box<dyn Client>>>>>,
}

impl PlatformClientRegistrar {
    pub fn new() -> Self {
        PlatformClientRegistrar {
            registries: HashMap::new()
        }
    }

    pub fn register(
        &mut self,
        platform_name: String,
        client_type: ClientType,
        client: Arc<Mutex<Box<dyn Client>>>
    ) -> &Self {
        self.registries
            .entry(platform_name)
            .or_insert_with(HashMap::new)
            .entry(client_type)
            .or_insert( client);

        self
    }

    pub fn get_client(&mut self, platform_name: String, client_type: ClientType) -> Option<&mut Arc<Mutex<Box<dyn Client>>>> {
        self.registries
            .entry(platform_name)
            .or_insert_with(HashMap::new)
            .get_mut(&client_type)
    }
}
