use crate::domain::entities::{
    InferenceServerDeployment,
    InferenceServer
};
use shared::errors::Error;

pub trait InferenceServerRepository {
    // async fn save(&self, server: InferenceServer) -> Result<InferenceServer, Error>;
    // async fn find_by_metadata_name_version(&self, name: String, version: String) -> Result<Option<InferenceServer>, Error>;
    async fn list_all(&self) -> Result<Vec<InferenceServer>, Error>;
}

pub trait InferenceServerDeploymentRepository {
    // async fn save(&self, server: InferenceServerDeployment) -> Result<InferenceServerDeployment, Error>;
    async fn find_by_metadata_name_version(&self, name: String, version: String) -> Result<Option<InferenceServerDeployment>, Error>;
    // async fn find_by_inference_server(&self, server: InferenceServer) -> Result<Option<InferenceServer>, Error>;
}
