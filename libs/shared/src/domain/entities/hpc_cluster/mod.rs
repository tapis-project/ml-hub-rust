use std::{collections::HashSet, fmt};

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HpcClusterId(Uuid);

impl HpcClusterId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn reconstitute(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for HpcClusterId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HpcClusterId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone)]
pub struct HpcCluster {
    id: HpcClusterId,
    enabled: bool,
    name: String,
    description: Option<String>,
    host: String,
    port: u16,
    documentation_url: Option<String>,
    data_center: DataCenter,
    queues: Vec<BatchSchedulerQueue>,
}

impl HpcCluster {
    pub fn new(props: NewHpcClusterProps) -> Result<Self, HpcClusterError> {
        let id = HpcClusterId::new();

        let queues = props
            .queues
            .into_iter()
            .map(|props| BatchSchedulerQueue::new(id, props))
            .collect::<Result<Vec<_>, _>>()?;

        Self::build(
            ReconstituteHpcClusterProps {
                id,
                enabled: props.enabled,
                name: props.name,
                description: props.description,
                host: props.host,
                port: props.port,
                documentation_url: props.documentation_url,
                data_center: props.data_center,
                queues,
            },
            false,
        )
    }

    pub fn reconstitute(props: ReconstituteHpcClusterProps) -> Result<Self, HpcClusterError> {
        Self::build(props, true)
    }

    fn build(props: ReconstituteHpcClusterProps, persisted: bool) -> Result<Self, HpcClusterError> {
        if let Err(error) = Self::validate(&props) {
            if persisted {
                return Err(HpcClusterError::DataIntegrityError(error.to_string()));
            }

            return Err(error);
        }

        Ok(Self {
            id: props.id,
            enabled: props.enabled,
            name: props.name,
            description: props.description,
            host: props.host,
            port: props.port,
            documentation_url: props.documentation_url,
            data_center: props.data_center,
            queues: props.queues,
        })
    }

    fn validate(props: &ReconstituteHpcClusterProps) -> Result<(), HpcClusterError> {
        if props.name.is_empty() {
            return Err(HpcClusterError::EmptyName);
        }

        if props.host.is_empty() {
            return Err(HpcClusterError::EmptyHost);
        }

        if props.port == 0 {
            return Err(HpcClusterError::InvalidPort);
        }

        let mut queue_ids = HashSet::new();
        let mut queue_names = HashSet::new();

        for queue in &props.queues {
            if queue.cluster_id() != &props.id {
                return Err(HpcClusterError::QueueClusterMismatch(*queue.id()));
            }

            if !queue_ids.insert(*queue.id()) {
                return Err(HpcClusterError::DuplicateQueueId(*queue.id()));
            }

            if !queue_names.insert(queue.name()) {
                return Err(HpcClusterError::DuplicateQueueName(queue.name().into()));
            }
        }

        Ok(())
    }

    pub fn id(&self) -> &HpcClusterId {
        &self.id
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn documentation_url(&self) -> Option<&str> {
        self.documentation_url.as_deref()
    }

    pub fn data_center(&self) -> &DataCenter {
        &self.data_center
    }

    pub fn queues(&self) -> &[BatchSchedulerQueue] {
        &self.queues
    }
}

#[derive(Debug, Clone)]
pub struct NewHpcClusterProps {
    pub enabled: bool,
    pub name: String,
    pub description: Option<String>,
    pub host: String,
    pub port: u16,
    pub documentation_url: Option<String>,
    pub data_center: DataCenter,
    pub queues: Vec<NewBatchSchedulerQueueProps>,
}

#[derive(Debug, Clone)]
pub struct ReconstituteHpcClusterProps {
    pub id: HpcClusterId,
    pub enabled: bool,
    pub name: String,
    pub description: Option<String>,
    pub host: String,
    pub port: u16,
    pub documentation_url: Option<String>,
    pub data_center: DataCenter,
    pub queues: Vec<BatchSchedulerQueue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataCenter {
    Tacc,
}

impl fmt::Display for DataCenter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tacc => write!(formatter, "Tacc"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatchSchedulerQueueId(Uuid);

impl BatchSchedulerQueueId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn reconstitute(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BatchSchedulerQueueId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BatchSchedulerQueueId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone)]
pub struct BatchSchedulerQueue {
    id: BatchSchedulerQueueId,
    cluster_id: HpcClusterId,
    enabled: bool,
    name: String,
    scheduler_type: SchedulerType,
    hardware_profile: HardwareProfile,
    scheduling_policy: SchedulingPolicy,
    billing_policy: Option<BillingPolicy>,
}

impl BatchSchedulerQueue {
    pub fn new(
        cluster_id: HpcClusterId,
        props: NewBatchSchedulerQueueProps,
    ) -> Result<Self, BatchSchedulerQueueError> {
        Self::build(
            ReconstituteBatchSchedulerQueueProps {
                id: BatchSchedulerQueueId::new(),
                cluster_id,
                enabled: props.enabled,
                name: props.name,
                scheduler_type: props.scheduler_type,
                hardware_profile: props.hardware_profile,
                scheduling_policy: props.scheduling_policy,
                billing_policy: props.billing_policy,
            },
            false,
        )
    }

    pub fn reconstitute(
        props: ReconstituteBatchSchedulerQueueProps,
    ) -> Result<Self, BatchSchedulerQueueError> {
        Self::build(props, true)
    }

    fn build(
        props: ReconstituteBatchSchedulerQueueProps,
        persisted: bool,
    ) -> Result<Self, BatchSchedulerQueueError> {
        if props.name.is_empty() {
            if persisted {
                return Err(BatchSchedulerQueueError::DataIntegrityError(
                    BatchSchedulerQueueError::EmptyName.to_string(),
                ));
            }

            return Err(BatchSchedulerQueueError::EmptyName);
        }

        Ok(Self {
            id: props.id,
            cluster_id: props.cluster_id,
            enabled: props.enabled,
            name: props.name,
            scheduler_type: props.scheduler_type,
            hardware_profile: props.hardware_profile,
            scheduling_policy: props.scheduling_policy,
            billing_policy: props.billing_policy,
        })
    }

    pub fn id(&self) -> &BatchSchedulerQueueId {
        &self.id
    }

    pub fn cluster_id(&self) -> &HpcClusterId {
        &self.cluster_id
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scheduler_type(&self) -> &SchedulerType {
        &self.scheduler_type
    }

    pub fn hardware_profile(&self) -> &HardwareProfile {
        &self.hardware_profile
    }

    pub fn scheduling_policy(&self) -> &SchedulingPolicy {
        &self.scheduling_policy
    }

    pub fn billing_policy(&self) -> Option<&BillingPolicy> {
        self.billing_policy.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct NewBatchSchedulerQueueProps {
    pub enabled: bool,
    pub name: String,
    pub scheduler_type: SchedulerType,
    pub hardware_profile: HardwareProfile,
    pub scheduling_policy: SchedulingPolicy,
    pub billing_policy: Option<BillingPolicy>,
}

#[derive(Debug, Clone)]
pub struct ReconstituteBatchSchedulerQueueProps {
    pub id: BatchSchedulerQueueId,
    pub cluster_id: HpcClusterId,
    pub enabled: bool,
    pub name: String,
    pub scheduler_type: SchedulerType,
    pub hardware_profile: HardwareProfile,
    pub scheduling_policy: SchedulingPolicy,
    pub billing_policy: Option<BillingPolicy>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerType {
    Slurm,
}

#[derive(Debug, Clone)]
pub struct HardwareProfile {
    total_nodes: u32,
    cpu_cores_per_node: u16,
    cpu_architecture: String,
    cpu_model: String,
    cpu_vendor: String,
    memory_gb: u32,
    accelerator: Option<Accelerator>,
}

impl HardwareProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        total_nodes: u32,
        cpu_cores_per_node: u16,
        cpu_architecture: String,
        cpu_model: String,
        cpu_vendor: String,
        memory_gb: u32,
        accelerator: Option<Accelerator>,
    ) -> Self {
        Self {
            total_nodes,
            cpu_cores_per_node,
            cpu_architecture,
            cpu_model,
            cpu_vendor,
            memory_gb,
            accelerator,
        }
    }

    pub fn total_nodes(&self) -> u32 {
        self.total_nodes
    }

    pub fn cpu_cores_per_node(&self) -> u16 {
        self.cpu_cores_per_node
    }

    pub fn cpu_architecture(&self) -> &str {
        &self.cpu_architecture
    }

    pub fn cpu_model(&self) -> &str {
        &self.cpu_model
    }

    pub fn cpu_vendor(&self) -> &str {
        &self.cpu_vendor
    }

    pub fn memory_gb(&self) -> u32 {
        self.memory_gb
    }

    pub fn accelerator(&self) -> Option<&Accelerator> {
        self.accelerator.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Accelerator {
    count_per_node: u8,
    profile: AcceleratorProfile,
}

impl Accelerator {
    pub fn new(count_per_node: u8, profile: AcceleratorProfile) -> Self {
        Self {
            count_per_node,
            profile,
        }
    }

    pub fn count_per_node(&self) -> u8 {
        self.count_per_node
    }

    pub fn profile(&self) -> &AcceleratorProfile {
        &self.profile
    }
}

#[derive(Debug, Clone)]
pub enum AcceleratorProfile {
    Gpu(GpuProfile),
}

#[derive(Debug, Clone)]
pub struct GpuProfile {
    gpu_model: String,
    gpu_vendor: String,
    gpu_memory_gb: u32,
    unified_memory: bool,
}

impl GpuProfile {
    pub fn new(
        gpu_model: String,
        gpu_vendor: String,
        gpu_memory_gb: u32,
        unified_memory: bool,
    ) -> Self {
        Self {
            gpu_model,
            gpu_vendor,
            gpu_memory_gb,
            unified_memory,
        }
    }

    pub fn gpu_model(&self) -> &str {
        &self.gpu_model
    }

    pub fn gpu_vendor(&self) -> &str {
        &self.gpu_vendor
    }

    pub fn gpu_memory_gb(&self) -> u32 {
        self.gpu_memory_gb
    }

    pub fn unified_memory(&self) -> bool {
        self.unified_memory
    }
}

#[derive(Debug, Clone)]
pub struct SchedulingPolicy {
    min_nodes_per_job: u32,
    max_nodes_per_job: u32,
    max_jobs_per_user: u16,
    max_nodes_per_user: u32,
    max_wall_time_per_job_hr: u16,
    max_job_submission: u16,
}

impl SchedulingPolicy {
    pub fn new(
        min_nodes_per_job: u32,
        max_nodes_per_job: u32,
        max_jobs_per_user: u16,
        max_nodes_per_user: u32,
        max_wall_time_per_job_hr: u16,
        max_job_submission: u16,
    ) -> Self {
        Self {
            min_nodes_per_job,
            max_nodes_per_job,
            max_jobs_per_user,
            max_nodes_per_user,
            max_wall_time_per_job_hr,
            max_job_submission,
        }
    }

    pub fn min_nodes_per_job(&self) -> u32 {
        self.min_nodes_per_job
    }

    pub fn max_nodes_per_job(&self) -> u32 {
        self.max_nodes_per_job
    }

    pub fn max_jobs_per_user(&self) -> u16 {
        self.max_jobs_per_user
    }

    pub fn max_nodes_per_user(&self) -> u32 {
        self.max_nodes_per_user
    }

    pub fn max_wall_time_per_job_hr(&self) -> u16 {
        self.max_wall_time_per_job_hr
    }

    pub fn max_job_submission(&self) -> u16 {
        self.max_job_submission
    }
}

#[derive(Debug, Clone)]
pub struct BillingPolicy {
    metric: BillingMetric,
    queue_multiplier: f64,
}

impl BillingPolicy {
    pub fn new(metric: BillingMetric, queue_multiplier: f64) -> Self {
        Self {
            metric,
            queue_multiplier,
        }
    }

    pub fn metric(&self) -> &BillingMetric {
        &self.metric
    }

    pub fn queue_multiplier(&self) -> f64 {
        self.queue_multiplier
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BillingMetric {
    PerNodeHour,
    PerGpuHour,
    PerCoreHour,
}

#[derive(Debug, Clone, Error)]
pub enum HpcClusterError {
    #[error("HPC cluster name MUST not be empty")]
    EmptyName,

    #[error("HPC cluster host MUST not be empty")]
    EmptyHost,

    #[error("HPC cluster port MUST be greater than zero")]
    InvalidPort,

    #[error("HPC cluster contains queue {0} belonging to another cluster")]
    QueueClusterMismatch(BatchSchedulerQueueId),

    #[error("HPC cluster contains duplicate queue id {0}")]
    DuplicateQueueId(BatchSchedulerQueueId),

    #[error("HPC cluster contains duplicate queue name {0}")]
    DuplicateQueueName(String),

    #[error(transparent)]
    InvalidQueue(#[from] BatchSchedulerQueueError),

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[derive(Debug, Clone, Error)]
pub enum BatchSchedulerQueueError {
    #[error("Batch scheduler queue name MUST not be empty")]
    EmptyName,

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
#[path = "hpc_cluster.test.rs"]
mod hpc_cluster_test;
