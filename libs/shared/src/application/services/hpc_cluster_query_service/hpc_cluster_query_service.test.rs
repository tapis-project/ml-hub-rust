use async_trait::async_trait;

use super::*;
use crate::{
    application::outputs::hpc_cluster::HpcClusterSummaryOutput,
    domain::entities::hpc_cluster::{ContainerRuntime, HpcCluster, NewHpcClusterProps},
};

struct TestRepository {
    found: Option<HpcCluster>,
    summaries: Vec<HpcClusterSummaryOutput>,
}

#[async_trait]
impl HpcClusterRepository for TestRepository {
    async fn list_all(&self) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError> {
        Ok(Vec::new())
    }

    async fn find_by_id(
        &self,
        _id: &HpcClusterId,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(self.found.clone())
    }

    async fn find_by_id_and_data_center(
        &self,
        _id: &HpcClusterId,
        _data_center: &DataCenter,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(self.found.clone())
    }

    async fn find_by_ids(
        &self,
        _ids: &[HpcClusterId],
    ) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError> {
        Ok(self.found.clone().into_iter().collect())
    }

    async fn list(
        &self,
        _input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError> {
        Ok(HpcClusterListOutput {
            hpc_clusters: self.summaries.clone(),
            cursor: Some("next".into()),
            count: Some(self.summaries.len() as u64),
        })
    }
}

fn cluster() -> Result<HpcCluster, HpcClusterQueryServiceError> {
    HpcCluster::new(NewHpcClusterProps {
        enabled: true,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        container_runtimes: vec![ContainerRuntime::Apptainer],
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: Vec::new(),
    })
    .map_err(|error| {
        HpcClusterRepositoryError::Persistence(
            crate::application::ports::errors::InfrastructureError::Transient {
                error_id: uuid::Uuid::now_v7(),
                reason: error.to_string(),
                retry_after: None,
            },
        )
        .into()
    })
}

#[tokio::test]
async fn gets_cluster_by_data_center_and_id() -> Result<(), Box<dyn std::error::Error>> {
    let cluster = cluster()?;

    let id = *cluster.id();

    let service = HpcClusterQueryService::new(Arc::new(TestRepository {
        found: Some(cluster),
        summaries: Vec::new(),
    }));

    let output = service
        .get_hpc_cluster(&RequestContext::system(None), &DataCenter::Tacc, &id)
        .await?;

    assert_eq!(output.hpc_cluster.id(), &id);

    Ok(())
}

#[tokio::test]
async fn reports_missing_cluster() {
    let service = HpcClusterQueryService::new(Arc::new(TestRepository {
        found: None,
        summaries: Vec::new(),
    }));

    let result = service
        .get_hpc_cluster(
            &RequestContext::system(None),
            &DataCenter::Tacc,
            &HpcClusterId::new(),
        )
        .await;

    assert!(matches!(result, Err(HpcClusterQueryServiceError::NotFound)));
}

#[tokio::test]
async fn lists_paginated_summaries() -> Result<(), Box<dyn std::error::Error>> {
    let summary = HpcClusterSummaryOutput {
        id: uuid::Uuid::now_v7(),
        enabled: false,
        name: "Vista".into(),
        data_center: DataCenter::Tacc,
    };

    let service = HpcClusterQueryService::new(Arc::new(TestRepository {
        found: None,
        summaries: vec![summary],
    }));

    let input = ListHpcClustersInput::new(DataCenter::Tacc, None, None, Some(true));

    let output = service
        .list_hpc_clusters(&RequestContext::system(None), &input)
        .await?;

    assert_eq!(output.hpc_clusters.len(), 1);
    assert!(!output.hpc_clusters[0].enabled);
    assert_eq!(output.cursor.as_deref(), Some("next"));
    assert_eq!(output.count, Some(1));

    Ok(())
}
