use async_trait::async_trait;
use shared::domain::entities::hpc_cluster::{HpcCluster, NewHpcClusterProps};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HpcClusterSeedError {
    #[error("Invalid HPC cluster seed configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Unable to seed HPC clusters: {0}")]
    Persistence(String),
}

pub trait HpcClusterSeedSource: Send + Sync {
    fn load(&self) -> Result<Vec<NewHpcClusterProps>, HpcClusterSeedError>;
}

#[async_trait]
pub trait HpcClusterSeedRepository: Send + Sync {
    async fn collection_exists(&self) -> Result<bool, HpcClusterSeedError>;

    async fn seed(&self, hpc_clusters: &[HpcCluster]) -> Result<(), HpcClusterSeedError>;
}
