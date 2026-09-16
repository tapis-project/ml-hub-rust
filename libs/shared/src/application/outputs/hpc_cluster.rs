use crate::domain::entities::hpc_cluster::{DataCenter, HpcCluster};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct HpcClusterSummaryOutput {
    pub id: Uuid,
    pub name: String,
    pub data_center: DataCenter,
}

#[derive(Clone, Debug)]
pub struct HpcClusterListOutput {
    pub hpc_clusters: Vec<HpcClusterSummaryOutput>,
    pub cursor: Option<String>,
    pub count: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct HpcClusterQueryOutput {
    pub hpc_cluster: HpcCluster,
}
