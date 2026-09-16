use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::{
        inputs::hpc_cluster::ListHpcClustersInput,
        outputs::hpc_cluster::{HpcClusterListOutput, HpcClusterQueryOutput},
        ports::hpc_cluster::{HpcClusterRepository, HpcClusterRepositoryError},
    },
    domain::entities::hpc_cluster::{DataCenter, HpcClusterId},
    shared_kernel::context::RequestContext,
};

#[derive(Debug, Error)]
pub enum HpcClusterQueryServiceError {
    #[error(transparent)]
    Repository(#[from] HpcClusterRepositoryError),

    #[error("HPC cluster not found")]
    NotFound,
}

pub struct HpcClusterQueryService {
    repository: Arc<dyn HpcClusterRepository>,
}

impl HpcClusterQueryService {
    const REPOSITORY_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(repository: Arc<dyn HpcClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_hpc_cluster(
        &self,
        _ctx: &RequestContext,
        data_center: &DataCenter,
        id: &HpcClusterId,
    ) -> Result<HpcClusterQueryOutput, HpcClusterQueryServiceError> {
        let hpc_cluster = retry_async(
            || self.repository.find_by_id(data_center, id),
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(HpcClusterQueryServiceError::NotFound)?;

        Ok(HpcClusterQueryOutput { hpc_cluster })
    }

    pub async fn list_hpc_clusters(
        &self,
        _ctx: &RequestContext,
        input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterQueryServiceError> {
        Ok(retry_async(
            || self.repository.list(input),
            &Self::REPOSITORY_RETRY_POLICY,
            None,
        )
        .await?)
    }
}

#[cfg(test)]
#[path = "hpc_cluster_query_service.test.rs"]
mod hpc_cluster_query_service_test;
