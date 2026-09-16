use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    application::outputs::hpc_cluster::HpcClusterSummaryOutput,
    domain::entities::hpc_cluster as domain,
};

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct HpcClusterSummary {
    pub id: Uuid,
    pub name: String,
    pub data_center: DataCenter,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct HpcCluster {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub host: String,
    pub port: u16,
    pub documentation_url: Option<String>,
    pub data_center: DataCenter,
    pub queues: Vec<BatchSchedulerQueue>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum DataCenter {
    Tacc,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct BatchSchedulerQueue {
    pub id: Uuid,
    pub cluster_id: Uuid,
    pub name: String,
    pub scheduler_type: SchedulerType,
    pub hardware_profile: HardwareProfile,
    pub scheduling_policy: SchedulingPolicy,
    pub billing_policy: Option<BillingPolicy>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum SchedulerType {
    Slurm,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct HardwareProfile {
    pub total_nodes: u32,
    pub cpu_cores_per_node: u16,
    pub cpu_architecture: String,
    pub cpu_model: String,
    pub cpu_vendor: String,
    pub memory_gb: u32,
    #[schema(required = true, nullable)]
    pub accelerator_type: Option<AcceleratorType>,
    #[schema(required = true, nullable)]
    pub gpu: Option<GpuProfile>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum AcceleratorType {
    Gpu,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct GpuProfile {
    pub count_per_node: u8,
    pub gpu_model: String,
    pub gpu_vendor: String,
    pub gpu_memory_gb: u32,
    pub unified_memory: bool,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct SchedulingPolicy {
    pub min_nodes_per_job: u32,
    pub max_nodes_per_job: u32,
    pub max_jobs_per_user: u16,
    pub max_nodes_per_user: u32,
    pub max_wall_time_per_job_hr: u16,
    pub max_job_submission: u16,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct BillingPolicy {
    pub metric: BillingMetric,
    pub queue_multiplier: f64,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum BillingMetric {
    PerNodeHour,
    PerGpuHour,
    PerCoreHour,
}

impl From<HpcClusterSummaryOutput> for HpcClusterSummary {
    fn from(value: HpcClusterSummaryOutput) -> Self {
        Self {
            id: value.id,
            name: value.name,
            data_center: (&value.data_center).into(),
        }
    }
}

impl From<domain::HpcCluster> for HpcCluster {
    fn from(value: domain::HpcCluster) -> Self {
        Self {
            id: *value.id().as_uuid(),
            name: value.name().into(),
            description: value.description().map(Into::into),
            host: value.host().into(),
            port: value.port(),
            documentation_url: value.documentation_url().map(Into::into),
            data_center: value.data_center().into(),
            queues: value.queues().iter().map(Into::into).collect(),
        }
    }
}

impl From<&domain::DataCenter> for DataCenter {
    fn from(value: &domain::DataCenter) -> Self {
        match value {
            domain::DataCenter::Tacc => Self::Tacc,
        }
    }
}

impl From<&domain::BatchSchedulerQueue> for BatchSchedulerQueue {
    fn from(value: &domain::BatchSchedulerQueue) -> Self {
        Self {
            id: *value.id().as_uuid(),
            cluster_id: *value.cluster_id().as_uuid(),
            name: value.name().into(),
            scheduler_type: value.scheduler_type().into(),
            hardware_profile: value.hardware_profile().into(),
            scheduling_policy: value.scheduling_policy().into(),
            billing_policy: value.billing_policy().map(Into::into),
        }
    }
}

impl From<&domain::SchedulerType> for SchedulerType {
    fn from(value: &domain::SchedulerType) -> Self {
        match value {
            domain::SchedulerType::Slurm => Self::Slurm,
        }
    }
}

impl From<&domain::HardwareProfile> for HardwareProfile {
    fn from(value: &domain::HardwareProfile) -> Self {
        let (accelerator_type, gpu) = match value.accelerator() {
            Some(accelerator) => (
                Some(AcceleratorType::Gpu),
                Some(GpuProfile::from(accelerator)),
            ),
            None => (None, None),
        };

        Self {
            total_nodes: value.total_nodes(),
            cpu_cores_per_node: value.cpu_cores_per_node(),
            cpu_architecture: value.cpu_architecture().into(),
            cpu_model: value.cpu_model().into(),
            cpu_vendor: value.cpu_vendor().into(),
            memory_gb: value.memory_gb(),
            accelerator_type,
            gpu,
        }
    }
}

impl From<&domain::Accelerator> for GpuProfile {
    fn from(value: &domain::Accelerator) -> Self {
        match value.profile() {
            domain::AcceleratorProfile::Gpu(profile) => Self {
                count_per_node: value.count_per_node(),
                gpu_model: profile.gpu_model().into(),
                gpu_vendor: profile.gpu_vendor().into(),
                gpu_memory_gb: profile.gpu_memory_gb(),
                unified_memory: profile.unified_memory(),
            },
        }
    }
}

impl From<&domain::SchedulingPolicy> for SchedulingPolicy {
    fn from(value: &domain::SchedulingPolicy) -> Self {
        Self {
            min_nodes_per_job: value.min_nodes_per_job(),
            max_nodes_per_job: value.max_nodes_per_job(),
            max_jobs_per_user: value.max_jobs_per_user(),
            max_nodes_per_user: value.max_nodes_per_user(),
            max_wall_time_per_job_hr: value.max_wall_time_per_job_hr(),
            max_job_submission: value.max_job_submission(),
        }
    }
}

impl From<&domain::BillingPolicy> for BillingPolicy {
    fn from(value: &domain::BillingPolicy) -> Self {
        Self {
            metric: value.metric().into(),
            queue_multiplier: value.queue_multiplier(),
        }
    }
}

impl From<&domain::BillingMetric> for BillingMetric {
    fn from(value: &domain::BillingMetric) -> Self {
        match value {
            domain::BillingMetric::PerNodeHour => Self::PerNodeHour,
            domain::BillingMetric::PerGpuHour => Self::PerGpuHour,
            domain::BillingMetric::PerCoreHour => Self::PerCoreHour,
        }
    }
}
