pub mod indexes;

use mongodb::bson::{oid::ObjectId, Uuid};
use serde::{Deserialize, Serialize};

use crate::domain::entities::hpc_cluster as domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpcCluster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    pub name: String,
    pub description: Option<String>,
    pub host: String,
    pub port: u16,
    pub documentation_url: Option<String>,
    pub data_center: DataCenter,
    pub queues: Vec<BatchSchedulerQueue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpcClusterSummary {
    pub _id: ObjectId,
    pub id: Uuid,
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    pub name: String,
    pub data_center: DataCenter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCenter {
    Tacc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSchedulerQueue {
    pub id: Uuid,
    pub cluster_id: Uuid,
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    pub name: String,
    pub scheduler_type: SchedulerType,
    pub hardware_profile: HardwareProfile,
    pub scheduling_policy: SchedulingPolicy,
    pub billing_policy: Option<BillingPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulerType {
    Slurm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub total_nodes: u32,
    pub cpu_cores_per_node: u16,
    pub cpu_architecture: String,
    pub cpu_model: String,
    pub cpu_vendor: String,
    pub memory_gb: u32,
    pub accelerator_type: Option<AcceleratorType>,
    pub gpu: Option<GpuProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcceleratorType {
    Gpu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuProfile {
    pub count_per_node: u8,
    pub gpu_model: String,
    pub gpu_vendor: String,
    pub gpu_memory_gb: u32,
    pub unified_memory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingPolicy {
    pub min_nodes_per_job: u32,
    pub max_nodes_per_job: u32,
    pub max_jobs_per_user: u16,
    pub max_nodes_per_user: u32,
    pub max_wall_time_per_job_hr: u16,
    pub max_job_submission: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPolicy {
    pub metric: BillingMetric,
    pub queue_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingMetric {
    PerNodeHour,
    PerGpuHour,
    PerCoreHour,
}

fn enabled_by_default() -> bool {
    true
}

impl From<&domain::HpcCluster> for HpcCluster {
    fn from(value: &domain::HpcCluster) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(*value.id().as_uuid().as_bytes()),
            enabled: value.enabled(),
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

impl TryFrom<HpcCluster> for domain::HpcCluster {
    type Error = domain::HpcClusterError;

    fn try_from(value: HpcCluster) -> Result<Self, Self::Error> {
        let queues = value
            .queues
            .into_iter()
            .map(domain::BatchSchedulerQueue::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| domain::HpcClusterError::DataIntegrityError(error.to_string()))?;

        domain::HpcCluster::reconstitute(domain::ReconstituteHpcClusterProps {
            id: domain::HpcClusterId::reconstitute(uuid::Uuid::from_bytes(value.id.bytes())),
            enabled: value.enabled,
            name: value.name,
            description: value.description,
            host: value.host,
            port: value.port,
            documentation_url: value.documentation_url,
            data_center: value.data_center.into(),
            queues,
        })
    }
}

impl From<&domain::DataCenter> for DataCenter {
    fn from(value: &domain::DataCenter) -> Self {
        match value {
            domain::DataCenter::Tacc => Self::Tacc,
        }
    }
}

impl From<DataCenter> for domain::DataCenter {
    fn from(value: DataCenter) -> Self {
        match value {
            DataCenter::Tacc => Self::Tacc,
        }
    }
}

impl From<&domain::BatchSchedulerQueue> for BatchSchedulerQueue {
    fn from(value: &domain::BatchSchedulerQueue) -> Self {
        Self {
            id: Uuid::from_bytes(*value.id().as_uuid().as_bytes()),
            cluster_id: Uuid::from_bytes(*value.cluster_id().as_uuid().as_bytes()),
            enabled: value.enabled(),
            name: value.name().into(),
            scheduler_type: value.scheduler_type().into(),
            hardware_profile: value.hardware_profile().into(),
            scheduling_policy: value.scheduling_policy().into(),
            billing_policy: value.billing_policy().map(Into::into),
        }
    }
}

impl TryFrom<BatchSchedulerQueue> for domain::BatchSchedulerQueue {
    type Error = domain::BatchSchedulerQueueError;

    fn try_from(value: BatchSchedulerQueue) -> Result<Self, Self::Error> {
        let hardware_profile = value.hardware_profile.try_into()?;

        domain::BatchSchedulerQueue::reconstitute(domain::ReconstituteBatchSchedulerQueueProps {
            id: domain::BatchSchedulerQueueId::reconstitute(uuid::Uuid::from_bytes(
                value.id.bytes(),
            )),
            cluster_id: domain::HpcClusterId::reconstitute(uuid::Uuid::from_bytes(
                value.cluster_id.bytes(),
            )),
            enabled: value.enabled,
            name: value.name,
            scheduler_type: value.scheduler_type.into(),
            hardware_profile,
            scheduling_policy: value.scheduling_policy.into(),
            billing_policy: value.billing_policy.map(Into::into),
        })
    }
}

impl From<&domain::SchedulerType> for SchedulerType {
    fn from(value: &domain::SchedulerType) -> Self {
        match value {
            domain::SchedulerType::Slurm => Self::Slurm,
        }
    }
}

impl From<SchedulerType> for domain::SchedulerType {
    fn from(value: SchedulerType) -> Self {
        match value {
            SchedulerType::Slurm => Self::Slurm,
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

impl TryFrom<HardwareProfile> for domain::HardwareProfile {
    type Error = domain::BatchSchedulerQueueError;

    fn try_from(value: HardwareProfile) -> Result<Self, Self::Error> {
        let accelerator = match (value.accelerator_type, value.gpu) {
            (None, None) => None,
            (Some(AcceleratorType::Gpu), Some(gpu)) => Some(gpu.into()),
            (Some(AcceleratorType::Gpu), None) => {
                return Err(domain::BatchSchedulerQueueError::DataIntegrityError(
                    "GPU accelerator discriminator requires a GPU definition".into(),
                ));
            }
            (None, Some(_)) => {
                return Err(domain::BatchSchedulerQueueError::DataIntegrityError(
                    "GPU definition requires an accelerator discriminator".into(),
                ));
            }
        };

        Ok(Self::new(
            value.total_nodes,
            value.cpu_cores_per_node,
            value.cpu_architecture,
            value.cpu_model,
            value.cpu_vendor,
            value.memory_gb,
            accelerator,
        ))
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

impl From<GpuProfile> for domain::GpuProfile {
    fn from(value: GpuProfile) -> Self {
        Self::new(
            value.gpu_model,
            value.gpu_vendor,
            value.gpu_memory_gb,
            value.unified_memory,
        )
    }
}

impl From<GpuProfile> for domain::Accelerator {
    fn from(value: GpuProfile) -> Self {
        let count_per_node = value.count_per_node;
        let profile = domain::GpuProfile::from(value);

        Self::new(count_per_node, domain::AcceleratorProfile::Gpu(profile))
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

impl From<SchedulingPolicy> for domain::SchedulingPolicy {
    fn from(value: SchedulingPolicy) -> Self {
        Self::new(
            value.min_nodes_per_job,
            value.max_nodes_per_job,
            value.max_jobs_per_user,
            value.max_nodes_per_user,
            value.max_wall_time_per_job_hr,
            value.max_job_submission,
        )
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

impl From<BillingPolicy> for domain::BillingPolicy {
    fn from(value: BillingPolicy) -> Self {
        Self::new(value.metric.into(), value.queue_multiplier)
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

impl From<BillingMetric> for domain::BillingMetric {
    fn from(value: BillingMetric) -> Self {
        match value {
            BillingMetric::PerNodeHour => Self::PerNodeHour,
            BillingMetric::PerGpuHour => Self::PerGpuHour,
            BillingMetric::PerCoreHour => Self::PerCoreHour,
        }
    }
}

#[cfg(test)]
#[path = "hpc_cluster.test.rs"]
mod hpc_cluster_test;
