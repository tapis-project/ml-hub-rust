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
    async fn find_by_id(
        &self,
        data_center: &DataCenter,
        id: &HpcClusterId,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError>;

    async fn list(
        &self,
        input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError>;
}
