use std::sync::Arc;

use shared::domain::entities::hpc_cluster::HpcCluster;

use crate::application::ports::{
    HpcClusterSeedError, HpcClusterSeedRepository, HpcClusterSeedSource,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HpcClusterSeedOutcome {
    SkippedCollectionExists,
    Seeded {
        hpc_cluster_count: usize,
        queue_count: usize,
    },
}

pub struct HpcClusterSeedService {
    source: Arc<dyn HpcClusterSeedSource>,
    repository: Arc<dyn HpcClusterSeedRepository>,
}

impl HpcClusterSeedService {
    pub fn new(
        source: Arc<dyn HpcClusterSeedSource>,
        repository: Arc<dyn HpcClusterSeedRepository>,
    ) -> Self {
        Self { source, repository }
    }

    pub async fn seed(&self) -> Result<HpcClusterSeedOutcome, HpcClusterSeedError> {
        if self.repository.collection_exists().await? {
            return Ok(HpcClusterSeedOutcome::SkippedCollectionExists);
        }

        let props = self.source.load()?;

        if props.is_empty() {
            return Err(HpcClusterSeedError::InvalidConfiguration(
                "at least one HPC cluster is required".into(),
            ));
        }

        let hpc_clusters = props
            .into_iter()
            .map(HpcCluster::new)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| HpcClusterSeedError::InvalidConfiguration(error.to_string()))?;

        let queue_count = hpc_clusters
            .iter()
            .map(|hpc_cluster| hpc_cluster.queues().len())
            .sum();

        self.repository.seed(&hpc_clusters).await?;

        Ok(HpcClusterSeedOutcome::Seeded {
            hpc_cluster_count: hpc_clusters.len(),
            queue_count,
        })
    }
}

#[cfg(test)]
#[path = "hpc_cluster_seed_service.test.rs"]
mod hpc_cluster_seed_service_test;
