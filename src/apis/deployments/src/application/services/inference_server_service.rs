use crate::application::ports::repositories::InferenceServerRepository;
use crate::domain::entities::InferenceServer;
use crate::application::inputs;
use shared::errors::Error;
use std::sync::Arc;

pub struct InferenceServerService {
    repo: Arc<dyn InferenceServerRepository>
}

impl InferenceServerService {
    pub fn new(repo: Arc<dyn InferenceServerRepository>) -> Self {
        Self {
            repo
        }
    }

    pub async fn create(&self, input: inputs::CreateInferenceServerInput) -> Result<InferenceServer, Error> {
        let inference_server = self.repo
            .save(InferenceServer::try_from(input)?)
            .await?;
        
        return Ok(inference_server);
    }

    pub async fn list_all(&self, input: Option<inputs::ListAll>) -> Result<Vec<InferenceServer>, Error> {
        let _x = input.is_none();
        let inference_servers = self.repo
            .list_all()
            .await?;
        
        return Ok(inference_servers);
    }
}