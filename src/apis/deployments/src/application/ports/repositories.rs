use crate::domain::entities::{
    InferenceServerDeployment,
    InferenceServer
};
use shared::errors::Error;
use async_trait::async_trait;

#[async_trait]
pub trait InferenceServerRepository: Send + Sync {
    async fn save(&self, server: InferenceServer) -> Result<InferenceServer, Error>;
    async fn exists_by_metadata_name_version(&self, name: String, version: String) -> Result<bool, Error>;
    async fn find_by_metadata_name_version(&self, name: String, version: String) -> Result<Option<InferenceServer>, Error>;
    async fn delete_by_metadata_name_version(&self, name: String, version: String) -> Result<(), Error>;
    async fn list_all(&self) -> Result<Vec<InferenceServer>, Error>;
}

#[async_trait]
pub trait InferenceServerDeploymentRepository: Send + Sync {
    // async fn save(&self, server: InferenceServerDeployment) -> Result<InferenceServerDeployment, Error>;
    async fn find_by_labels(&self, key: String, value: String) -> Result<Option<InferenceServerDeployment>, Error>;
    // async fn find_by_inference_server(&self, server: InferenceServer) -> Result<Option<InferenceServer>, Error>;
}
