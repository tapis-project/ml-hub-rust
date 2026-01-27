use std::collections::HashMap;
use openapiv3::OpenAPI;
use uuid::Uuid;
use thiserror::Error;
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::visibility::Visibility;

#[derive(Debug, Error)]
pub enum ModelDeploymentError {
    #[error("Invalid status change. Cannot move from status '{0}' to {1}")]
    InvalidStatusTransition(String, String),
}

#[derive(Clone, Debug)]
pub struct ModelReference {
    pub name: String,
    pub author: String,
}

#[derive(Clone, Debug)]
pub struct DeploymentStrategyReference {
    pub client: String,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ModelDeploymentStatus {
    /// A model deployment has been requested
    Submitted,
    /// The request has successfully been added to the queue
    Queued,
    /// Preparing the model artifact for the deployment. Pull, uploading, etc.
    Provisioning,
    /// The client responsible for deploying the model has picked up and is now
    /// processing the request
    Starting,
    /// The client has successfully started the model deployment
    Running,
    /// A request to stop the deployment has been picked and the client is now
    /// processing it
    Stopping,
    /// The client has successfully stopped the deployment
    Stopped,
    /// The deployment has failed (never started or crashed)
    Failed,
}

impl From<ModelDeploymentStatus> for String {
    fn from(value: ModelDeploymentStatus) -> Self {
        match value {
            ModelDeploymentStatus::Submitted => "Submitted".into(),
            ModelDeploymentStatus::Queued => "Queued".into(),
            ModelDeploymentStatus::Provisioning => "Provisioning".into(),
            ModelDeploymentStatus::Starting => "Starting".into(),
            ModelDeploymentStatus::Running => "Running".into(),
            ModelDeploymentStatus::Stopping => "Stopping".into(),
            ModelDeploymentStatus::Stopped => "Stopped".into(),
            ModelDeploymentStatus::Failed => "Failed".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModelDeployment {
    pub id: Uuid,
    pub owner: String,
    pub model: ModelReference,
    pub status: ModelDeploymentStatus,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<DeploymentStrategyReference>,
    pub visibility: Visibility,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub parallelism: Option<ReplicaGroup>,
}

impl ModelDeployment {
    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) {
        self.last_modified = TimeStamp::now()
    }

    fn valid_transitions() -> HashMap<ModelDeploymentStatus, Vec<ModelDeploymentStatus>> {
        let mut transitions = HashMap::new();
        transitions.insert(ModelDeploymentStatus::Submitted, vec![ModelDeploymentStatus::Queued]);
        transitions.insert(ModelDeploymentStatus::Queued, vec![ModelDeploymentStatus::Provisioning]);
        transitions.insert(ModelDeploymentStatus::Provisioning, vec![ModelDeploymentStatus::Starting]);
        transitions.insert(ModelDeploymentStatus::Starting, vec![ModelDeploymentStatus::Running, ModelDeploymentStatus::Failed]);
        transitions.insert(ModelDeploymentStatus::Running, vec![ModelDeploymentStatus::Stopping, ModelDeploymentStatus::Failed]);
        transitions.insert(ModelDeploymentStatus::Stopping, vec![ModelDeploymentStatus::Stopped, ModelDeploymentStatus::Failed]);
        transitions.insert(ModelDeploymentStatus::Stopped, vec![ModelDeploymentStatus::Provisioning]);
        transitions.insert(ModelDeploymentStatus::Failed, vec![ModelDeploymentStatus::Provisioning]);
        transitions
    }

    /// Returns whether a transition from one status to another is valid
    fn is_valid_status_transition(from: &ModelDeploymentStatus, to: &ModelDeploymentStatus) -> bool {
        Self::valid_transitions()
            .get(from)
            .map_or(false, |allowed| allowed.contains(to))
    }

    /// Changes the status. Returns an error if invalid status transition is detected
    pub fn change_status(&mut self, new_status: ModelDeploymentStatus, message: Option<String>) -> Result<(), ModelDeploymentError> {
        if !Self::is_valid_status_transition(&self.status, &new_status) {
            return Err(ModelDeploymentError::InvalidStatusTransition(self.status.clone().into(), new_status.into()))
        }

        // Changes the status
        self.status = new_status;

        // Update the last message if provided
        if let Some(m) = message {
            self.last_message = Some(m);
        }

        // Updates last_modified
        self.touch();

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ReplicaGroup {
    /// Number of replicas
    pub count: u8,
    /// Resources required by each replica
    pub resources: ResourceRequirements,
    /// Sharding / parallelism strategies actually employed by the deployment runtime.
    pub parallelism_strategies: Vec<ParallelismStrategy>,
}

#[derive(Clone, Debug)]
pub struct ResourceRequirements {
    /// Number of cpu cores (float for platforms that support fractional cores)
    pub cores: Option<f32>,
    /// Required disk space in GB
    pub disk: Option<f32>,
    /// Required memory in GB
    pub memory: Option<f32>,
    pub gpu: Option<GpuResource>,
}

#[derive(Clone, Debug)]
pub struct GpuResource {
    /// Number vram in GB
    pub memory: Option<f32>,
    /// Ex Nvida
    pub vendor: Option<String>,
    /// Ex H100
    pub gpu_type: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ParallelismStrategy {
    DataSharding,
    ModelSharding,
    PipelineSharding,
    TensorSharding,
}

#[derive(Clone, Debug)]
pub enum ModelDeploymentInterface {
    RestApi(RestApi)
}

#[derive(Clone, Debug)]
pub struct RestApi {
    pub spec: OpenAPI,
}