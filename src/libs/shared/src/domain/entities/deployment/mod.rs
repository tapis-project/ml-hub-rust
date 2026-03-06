use std::collections::HashMap;
use openapiv3::OpenAPI;
use platforms::Platform;
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
pub struct ModelDeployment {
    /// The unique identifier of this deployment
    pub id: Uuid,
    pub platform: Platform,
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
    pub deployment_strategy: Option<String>,
    pub visibility: Visibility,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    pub last_desired_state_change: TimeStamp,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: Option<ReplicaGroup>,
    /// Metadata provided by and for deployment clients
    pub metadata: Option<ModelDeploymentMetadata>,
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
            platform: props.platform,
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
            replicas: props.replicas,
            metadata: props.metadata,
            revision: 0,
        }
    }

    pub fn rehydrate(props: RehydrateModelDeploymentProps) -> Self {
        Self {
            id: props.id,
            platform: props.platform,
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
            replicas: props.replicas,
            metadata: props.metadata,
            revision: props.revision,
        }
    }

    pub fn is_state_syncronized(&self) -> bool {
        match (&self.state, &self.desired_state) {
            (State::Running, DesiredState::Running) => true,
            (State::Stopped, DesiredState::Stopped) => true,
            _ => false,
        }
    }

    pub fn revision(&self) -> &u32 {
        &self.revision
    }

    pub fn revise(&mut self) -> ModelDeploymentDraft<'_> {
        let revision = self.revision + 1;
        let draft = ModelDeploymentDraft { deployment: self, revision };
        
        draft
    }
}

#[derive(Clone, Debug)]
pub struct ModelReference {
    pub name: String,
    pub author: String,
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
pub struct ModelDeploymentMetadata(pub HashMap<String, Value>);

impl ModelDeploymentMetadata {
    pub fn into_inner(&self) -> &HashMap<String, Value> {
        &self.0
    }

    /// Get a metadata value by key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    /// Iterate over all metadata key/value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub enum ModelDeploymentMetadataDelta {
    NoChange,
    Delete,
    Merge(ModelDeploymentMetadata)
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
pub enum ReplicaGroupDelta {
    NoChange,
    Delete,
    Replace(ReplicaGroup)
}

#[derive(Clone, Debug)]
pub enum ModelDeploymentInterface {
    RestApi(RestApi)
}

#[derive(Clone, Debug)]
pub struct RestApi {
    pub spec: OpenAPI,
}

#[derive(Clone, Debug)]
pub enum ModelDeploymentInterfaceDelta {
    NoChange,
    Delete,
    Replace(ModelDeploymentInterface)
}

#[derive(Debug)]
pub struct ModelDeploymentDraft<'a> {
    deployment: &'a mut ModelDeployment,
    revision: u32
}

impl <'a>ModelDeploymentDraft<'a> {
    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) -> &mut Self {
        let now = TimeStamp::now();
        self.deployment.last_modified = now.clone();

        self
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
    pub fn transition_to_state(&mut self, new_state: State, message: Option<String>) -> Result<&mut Self, ModelDeploymentError> {
        if !Self::is_valid_state_transition(&self.deployment.state, &new_state) {
            return Err(ModelDeploymentError::InvalidStateTransition(self.deployment.state.clone().into(), new_state.into()))
        }

        // Changes the state
        self.deployment.state = new_state;

        // Update the last message if provided
        if let Some(m) = message {
            self.deployment.last_message = Some(m);
        }

        self.deployment.last_state_change = TimeStamp::now();

        Ok(self)
    }

    fn valid_desired_state_transitions() -> HashMap<DesiredState, Vec<DesiredState>> {
        let mut transitions = HashMap::new();
        transitions.insert(DesiredState::NotDeployed, vec![DesiredState::Running]);
        transitions.insert(DesiredState::Running, vec![DesiredState::Stopped]);
        transitions.insert(DesiredState::Running, vec![DesiredState::NotDeployed]);
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
    pub fn transition_to_desired(&mut self, new_state: DesiredState, message: Option<String>) -> Result<&mut Self, ModelDeploymentError> {
        if !Self::is_valid_desired_state_transition(&self.deployment.desired_state, &new_state) {
            return Err(ModelDeploymentError::InvalidDesiredStateTransition(self.deployment.state.clone().into(), new_state.into()))
        }

        self.deployment.desired_state = new_state;

        // Update the last message if provided
        if let Some(m) = message {
            self.deployment.last_message = Some(m);
        }

        self.deployment.last_desired_state_change = TimeStamp::now();

        Ok(self)
    }

    pub fn apply_interface_delta(&mut self, delta: ModelDeploymentInterfaceDelta) -> &mut Self {
        match delta {
            ModelDeploymentInterfaceDelta::Delete => { self.deployment.metadata = None },
            ModelDeploymentInterfaceDelta::Replace(i) => { self.deployment.deployment_interface = Some(i); },
            ModelDeploymentInterfaceDelta::NoChange => {},
        };

        self
    }

    pub fn apply_replica_group_delta(&mut self, delta: ReplicaGroupDelta) -> &mut Self {
        match delta {
            ReplicaGroupDelta::Delete => { self.deployment.metadata = None },
            ReplicaGroupDelta::Replace(r) => { self.deployment.replicas = Some(r); },
            ReplicaGroupDelta::NoChange => {},
        };

        self
    }

    pub fn apply_metadata_delta(&mut self, delta: ModelDeploymentMetadataDelta) -> &mut Self {
        match delta {
            ModelDeploymentMetadataDelta::Delete => { self.deployment.metadata = None },
            ModelDeploymentMetadataDelta::Merge(m) => { self.merge_metadata(m); },
            ModelDeploymentMetadataDelta::NoChange => {},
        };

        self
    }

    fn merge_metadata(&mut self, metadata: ModelDeploymentMetadata) -> &mut Self {
        for (k, v) in metadata.into_inner() {
            self.add_metadata(k.clone(), v.clone());
        }
        
        self
    }

    fn add_metadata(&mut self, k: String, v: Value) -> &mut Self {
        if let Some(ref mut m) = self.deployment.metadata.as_mut().and_then(|m| Some(m.into_inner().clone())) {
            m.insert(k, v);
            return self
        }

        let mut metadata = HashMap::with_capacity(1);

        metadata.insert(k, v);

        self.deployment.metadata = Some(ModelDeploymentMetadata(metadata));

        self
    }

    // TODO This method needs to consume self so that finish cannot be called more
    // than once on this draft
    pub fn finish(&mut self) -> ModelDeployment {
        self.touch();
        self.deployment.revision = self.revision;
        self.deployment.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RehydrateModelDeploymentProps {
    pub id: Uuid,
    pub platform: Platform,
    pub owner: String,
    pub model: ModelReference,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<String>,
    pub visibility: Visibility,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: Option<ReplicaGroup>,
    pub revision: u32,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    pub last_desired_state_change: TimeStamp,
    pub created_at: TimeStamp,
    pub metadata: Option<ModelDeploymentMetadata>,
}

#[derive(Clone, Debug)]
pub struct ModelDeploymentProps {
    pub id: Uuid,
    pub platform: Platform,
    pub owner: String,
    pub model: ModelReference,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_strategy: Option<String>,
    pub visibility: Visibility,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: Option<ReplicaGroup>,
    pub metadata: Option<ModelDeploymentMetadata>,
}