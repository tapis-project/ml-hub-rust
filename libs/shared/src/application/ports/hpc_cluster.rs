use async_trait::async_trait;
use thiserror::Error;

use crate::{
    application::{
        inputs::hpc_cluster::ListHpcClustersInput, outputs::hpc_cluster::HpcClusterListOutput,
        ports::errors::InfrastructureError,
    },
    domain::entities::hpc_cluster::{DataCenter, HpcCluster, HpcClusterId},
};

#[derive(Debug, Error, Clone)]
pub enum HpcClusterRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait HpcClusterRepository: Send + Sync {
    async fn list_all(&self) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError>;

    async fn find_by_id(
        &self,
        id: &HpcClusterId,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError>;

    async fn find_by_id_and_data_center(
        &self,
        id: &HpcClusterId,
        data_center: &DataCenter,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError>;

    async fn find_by_ids(
        &self,
        ids: &[HpcClusterId],
    ) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError>;

    async fn list(
        &self,
        input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError>;
}
