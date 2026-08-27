use async_trait::async_trait;
use thiserror::Error;

use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::endpoint::Endpoint;

#[derive(Debug, Error)]
pub enum EndpointRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait EndpointRepository: Send + Sync {
    async fn get_by_target_url(
        &self,
        tenant_id: &str,
        target_url: &str,
    ) -> Result<Option<Endpoint>, EndpointRepositoryError>;

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Endpoint>, EndpointRepositoryError>;

    async fn save(&self, endpoint: &Endpoint) -> Result<(), EndpointRepositoryError>;
}
