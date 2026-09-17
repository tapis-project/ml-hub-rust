use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;
use shared::domain::entities::hpc_cluster as domain;

use crate::application::ports::{HpcClusterSeedError, HpcClusterSeedSource};

pub struct FileHpcClusterSeedSource {
    path: PathBuf,
}

impl FileHpcClusterSeedSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl HpcClusterSeedSource for FileHpcClusterSeedSource {
    fn load(&self) -> Result<Vec<domain::NewHpcClusterProps>, HpcClusterSeedError> {
        let contents = read_to_string(&self.path).map_err(|error| {
            HpcClusterSeedError::InvalidConfiguration(format!(
                "unable to read {}: {error}",
                self.path.display()
            ))
        })?;

        let hpc_clusters = serde_json::from_str::<Vec<HpcCluster>>(&contents).map_err(|error| {
            HpcClusterSeedError::InvalidConfiguration(format!(
                "unable to parse {}: {error}",
                self.path.display()
            ))
        })?;

        hpc_clusters.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct HpcCluster {
    enabled: bool,
    name: String,
    description: Option<String>,
    host: String,
    port: u16,
    documentation_url: Option<String>,
    data_center: DataCenter,
    queues: Vec<BatchSchedulerQueue>,
}

#[derive(Debug, Deserialize)]
enum DataCenter {
    Tacc,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BatchSchedulerQueue {
    enabled: bool,
    name: String,
    scheduler_type: SchedulerType,
    hardware_profile: HardwareProfile,
    scheduling_policy: SchedulingPolicy,
    billing_policy: Option<BillingPolicy>,
}

#[derive(Debug, Deserialize)]
enum SchedulerType {
    Slurm,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct HardwareProfile {
    total_nodes: u32,
    cpu_cores_per_node: u16,
    cpu_architecture: String,
    cpu_model: String,
    cpu_vendor: String,
    memory_gb: u32,
    accelerator_type: Option<AcceleratorType>,
    gpu: Option<GpuProfile>,
}

#[derive(Debug, Deserialize)]
enum AcceleratorType {
    Gpu,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GpuProfile {
    count_per_node: u8,
    gpu_model: String,
    gpu_vendor: String,
    gpu_memory_gb: u32,
    unified_memory: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SchedulingPolicy {
    min_nodes_per_job: u32,
    max_nodes_per_job: u32,
    max_jobs_per_user: u16,
    max_nodes_per_user: u32,
    max_wall_time_per_job_hr: u16,
    max_job_submission: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BillingPolicy {
    metric: BillingMetric,
    queue_multiplier: f64,
}

#[derive(Debug, Deserialize)]
enum BillingMetric {
    PerNodeHour,
    PerGpuHour,
    PerCoreHour,
}

impl TryFrom<HpcCluster> for domain::NewHpcClusterProps {
    type Error = HpcClusterSeedError;

    fn try_from(value: HpcCluster) -> Result<Self, Self::Error> {
        let queues = value
            .queues
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
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

impl From<DataCenter> for domain::DataCenter {
    fn from(value: DataCenter) -> Self {
        match value {
            DataCenter::Tacc => Self::Tacc,
        }
    }
}

impl TryFrom<BatchSchedulerQueue> for domain::NewBatchSchedulerQueueProps {
    type Error = HpcClusterSeedError;

    fn try_from(value: BatchSchedulerQueue) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: value.enabled,
            name: value.name,
            scheduler_type: value.scheduler_type.into(),
            hardware_profile: value.hardware_profile.try_into()?,
            scheduling_policy: value.scheduling_policy.into(),
            billing_policy: value.billing_policy.map(Into::into),
        })
    }
}

impl From<SchedulerType> for domain::SchedulerType {
    fn from(value: SchedulerType) -> Self {
        match value {
            SchedulerType::Slurm => Self::Slurm,
        }
    }
}

impl TryFrom<HardwareProfile> for domain::HardwareProfile {
    type Error = HpcClusterSeedError;

    fn try_from(value: HardwareProfile) -> Result<Self, Self::Error> {
        let accelerator = match (value.accelerator_type, value.gpu) {
            (None, None) => None,
            (Some(AcceleratorType::Gpu), Some(gpu)) => Some(domain::Accelerator::new(
                gpu.count_per_node,
                domain::AcceleratorProfile::Gpu(domain::GpuProfile::new(
                    gpu.gpu_model,
                    gpu.gpu_vendor,
                    gpu.gpu_memory_gb,
                    gpu.unified_memory,
                )),
            )),
            (Some(AcceleratorType::Gpu), None) => {
                return Err(HpcClusterSeedError::InvalidConfiguration(
                    "GPU accelerator discriminator requires a GPU definition".into(),
                ));
            }
            (None, Some(_)) => {
                return Err(HpcClusterSeedError::InvalidConfiguration(
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

impl From<BillingPolicy> for domain::BillingPolicy {
    fn from(value: BillingPolicy) -> Self {
        Self::new(value.metric.into(), value.queue_multiplier)
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
#[path = "file_seed_source.test.rs"]
mod file_seed_source_test;
