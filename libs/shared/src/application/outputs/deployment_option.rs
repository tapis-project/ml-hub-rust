use crate::domain::entities::{
    deployment_option::{deployment_parameters::Parameter, DeploymentOption},
    hpc_cluster::{BatchSchedulerQueueId, DataCenter, HpcClusterId},
};

#[derive(Debug)]
pub struct DeploymentOptionListOutput {
    pub deployment_options: Vec<DeploymentOptionOutput>,
    pub cursor: Option<String>,
    pub count: Option<u64>,
}

#[derive(Debug)]
pub struct DeploymentOptionDetailOutput {
    pub deployment_option: DeploymentOptionOutput,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct DeploymentOptionOutput {
    pub deployment_option: DeploymentOption,
    pub target: HpcClusterQueueTargetOutput,
    pub available: bool,
}

#[derive(Debug)]
pub struct HpcClusterQueueTargetOutput {
    pub hpc_cluster_id: HpcClusterId,
    pub hpc_cluster_name: String,
    pub data_center: DataCenter,
    pub hpc_cluster_enabled: bool,
    pub batch_scheduler_queue_id: BatchSchedulerQueueId,
    pub batch_scheduler_queue_name: String,
    pub batch_scheduler_queue_enabled: bool,
}
