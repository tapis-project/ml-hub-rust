use std::collections::HashMap;
use openapiv3::OpenAPI;
use uuid::Uuid;
use thiserror::Error;
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::visibility::Visibility;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum ModelDeploymentError {
    #[error("Invalid state change. Cannot move from state '{0}' to {1}")]
    InvalidStateTransition(String, String),

    #[error("Invalid desired state change. Cannot move from desired state '{0}' to {1}")]
    InvalidDesiredStateTransition(String, String),
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
pub enum State {
    /// The deployment infrastructure does not exist
    NotDeployed,
    /// The deployment infrastructure exists and is running
    Running,
    /// The client has successfully stopped the deployment
    Stopped,
    /// The deployment has failed (never started or crashed)
    Failed,
    /// The deployment cannot be acted up or controlled
    Blocked,
    /// Observability gap. The state of the deployment cannot be known
    Unknown,
}

impl From<State> for String {
    fn from(value: State) -> Self {
        match value {
            State::NotDeployed => "NotDeployed".into(),
            State::Running => "Running".into(),
            State::Stopped => "Stopped".into(),
            State::Failed => "Failed".into(),
            State::Blocked => "Blocked".into(),
            State::Unknown => "Unknown".into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DesiredState {
    Running,
    Stopped,
    NotDeployed,
}

impl From<DesiredState> for String {
    fn from(value: DesiredState) -> Self {
        match value {
            DesiredState::Running => "Running".into(),
            DesiredState::Stopped => "Stopped".into(),
            DesiredState::NotDeployed => "NotDeployed".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RehydrateModelDeploymentProps {
    pub id: Uuid,
    pub owner: String,
    pub model: ModelReference,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<DeploymentStrategyReference>,
    pub visibility: Visibility,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub parallelism: Option<ReplicaGroup>,
    pub revision: u32,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    pub last_desired_state_change: TimeStamp,
    pub created_at: TimeStamp,
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Clone, Debug)]
pub struct ModelDeploymentProps {
    pub id: Uuid,
    pub owner: String,
    pub model: ModelReference,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<DeploymentStrategyReference>,
    pub visibility: Visibility,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub parallelism: Option<ReplicaGroup>,
}

#[derive(Clone, Debug)]
pub struct ModelDeployment {
    /// The unique identifier of this deployment
    pub id: Uuid,
    /// The user that owns this deployment
    pub owner: String,
    /// A reference to the model metadata
    pub model: ModelReference,
    /// The curent state of the delpoyment
    pub state: State,
    /// The state the user would like the deployment to be in
    pub desired_state: DesiredState,
    /// The last message associated with the last state or desired state change
    pub last_message: Option<String>,
    pub deployment_strategy: Option<DeploymentStrategyReference>,
    pub visibility: Visibility,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    pub last_desired_state_change: TimeStamp,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub parallelism: Option<ReplicaGroup>,
    /// Metadata provided by and for deployment clients
    pub metadata: Option<HashMap<String, Value>>,
    /// Indicates changes to desired state over time. This field is incremented
    /// every time desired state changes.
    revision: u32, 
}

impl ModelDeployment {
    /// Create the model deployment from props
    pub fn new(props: ModelDeploymentProps) -> Self {
        let now = TimeStamp::now();

        Self {
            id: props.id,
            owner: props.owner,
            model: props.model,
            state: props.state,
            desired_state: props.desired_state,
            last_message: props.last_message,
            deployment_strategy: props.deployment_strategy,
            visibility: props.visibility,
            created_at: now.clone(),
            last_modified: now.clone(),
            last_state_change: now.clone(),
            last_desired_state_change: now.clone(),
            deployment_interface: props.deployment_interface,
            parallelism: props.parallelism,
            metadata: None,
            revision: 0,
        }
    }

    pub fn rehydrate(props: RehydrateModelDeploymentProps) -> Self {
        Self {
            id: props.id,
            owner: props.owner,
            model: props.model,
            state: props.state,
            desired_state: props.desired_state,
            last_message: props.last_message,
            deployment_strategy: props.deployment_strategy,
            visibility: props.visibility,
            created_at: props.created_at,
            last_modified: props.last_modified,
            last_state_change: props.last_state_change,
            last_desired_state_change: props.last_desired_state_change,
            deployment_interface: props.deployment_interface,
            parallelism: props.parallelism,
            metadata: props.metadata,
            revision: props.revision,
        }
    }

    pub fn revision(&self) -> &u32 {
        &self.revision
    }

    /// Updates last modified to the UTC timestamp
    fn touch(&mut self, state_updated: Option<bool>) {
        let now = TimeStamp::now();
        self.last_modified = now.clone();

        if state_updated.unwrap_or(false) {
            self.last_state_change = now;
        }
    }

    fn valid_state_transitions() -> HashMap<State, Vec<State>> {
        let mut transitions = HashMap::new();
        transitions.insert(State::NotDeployed, vec![State::Blocked, State::Running, State::Failed]);
        transitions.insert(State::Running, vec![State::Blocked, State::Stopped, State::Failed]);
        transitions.insert(State::Stopped, vec![State::Blocked, State::Running, State::Failed]);
        transitions.insert(State::Failed, vec![State::Blocked, State::Running]);
        transitions.insert(State::Blocked, vec![State::Running, State::Stopped, State::Failed]);
        transitions
    }

    pub fn is_state_syncronized(&self) -> bool {
        match (&self.state, &self.desired_state) {
            (State::Running, DesiredState::Running) => true,
            (State::Stopped, DesiredState::Stopped) => true,
            _ => false,
        }
    }

    /// Returns whether a transition from one state to another is valid
    fn is_valid_state_transition(from: &State, to: &State) -> bool {
        // Unknown can transition to any state 
        if from == &State::Unknown {
            return true
        }

        Self::valid_state_transitions()
            .get(from)
            .map_or(false, |allowed| allowed.contains(to))
    }

    /// Changes the state. Returns an error if invalid state transition is detected
    pub fn change_state(&mut self, new_state: State, message: Option<String>) -> Result<(), ModelDeploymentError> {
        if !Self::is_valid_state_transition(&self.state, &new_state) {
            return Err(ModelDeploymentError::InvalidStateTransition(self.state.clone().into(), new_state.into()))
        }

        // Changes the state
        self.state = new_state;

        // Update the last message if provided
        if let Some(m) = message {
            self.last_message = Some(m);
        }

        // Updates last_modified
        self.touch(Some(true));

        Ok(())
    }

    fn valid_desired_state_transitions() -> HashMap<DesiredState, Vec<DesiredState>> {
        let mut transitions = HashMap::new();
        transitions.insert(DesiredState::Running, vec![DesiredState::Stopped]);
        transitions.insert(DesiredState::Stopped, vec![DesiredState::Running]);
        transitions
    }

    /// Returns whether a transition from one state to another is valid
    fn is_valid_desired_state_transition(from: &DesiredState, to: &DesiredState) -> bool {
        Self::valid_desired_state_transitions()
            .get(from)
            .map_or(false, |allowed| allowed.contains(to))
    }

    /// Changes the state. Returns an error if invalid state transition is detected
    pub fn change_desired_state(&mut self, new_state: DesiredState, message: Option<String>) -> Result<(), ModelDeploymentError> {
        if !Self::is_valid_desired_state_transition(&self.desired_state, &new_state) {
            return Err(ModelDeploymentError::InvalidDesiredStateTransition(self.state.clone().into(), new_state.into()))
        }

        self.desired_state = new_state;

        // Update the last message if provided
        if let Some(m) = message {
            self.last_message = Some(m);
        }

        // Updates last_modified
        self.touch(Some(false));

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