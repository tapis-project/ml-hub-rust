use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::application::ports::errors::InfrastructureError;
use crate::application::{
    inputs::endpoint::ListEndpointsInput, outputs::endpoint::EndpointListOutput,
};
use crate::domain::entities::endpoint::Endpoint;

#[derive(Debug, Error)]
pub enum EndpointRepositoryError {
    #[error("Endpoint already exists")]
    AlreadyExists,

    #[error("Invalid endpoint cursor")]
    InvalidCursor,

    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait EndpointRepository: Send + Sync {
    async fn list(
        &self,
        input: &ListEndpointsInput,
    ) -> Result<EndpointListOutput, EndpointRepositoryError> {
        let _ = input;

        Err(InfrastructureError::new_internal().into())
    }

    async fn get_by_id(
        &self,
        tenant_id: &str,
        owner: &str,
        id: &Uuid,
    ) -> Result<Option<Endpoint>, EndpointRepositoryError> {
        let _ = (tenant_id, owner, id);

        Err(InfrastructureError::new_internal().into())
    }

    async fn list_by_target_urn(
        &self,
        tenant_id: &str,
        target_resource_urn: &str,
    ) -> Result<Vec<Endpoint>, EndpointRepositoryError>;

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Endpoint>, EndpointRepositoryError>;

    async fn get_by_tenant_and_slug(
        &self,
        tenant_id: &str,
        slug: &str,
    ) -> Result<Option<Endpoint>, EndpointRepositoryError> {
        Ok(self
            .get_by_slug(slug)
            .await?
            .filter(|endpoint| endpoint.tenant_id() == tenant_id))
    }

    async fn get_by_target(
        &self,
        tenant_id: &str,
        target_resource_urn: &str,
        target_name: &str,
    ) -> Result<Option<Endpoint>, EndpointRepositoryError> {
        Ok(self
            .list_by_target_urn(tenant_id, target_resource_urn)
            .await?
            .into_iter()
            .find(|endpoint| endpoint.target_name() == target_name))
    }

    async fn save(&self, endpoint: &Endpoint) -> Result<(), EndpointRepositoryError>;
}
