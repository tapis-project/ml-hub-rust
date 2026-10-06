pub mod argument;

use crate::domain::entities::model::external_model::ExternalModelId;
use crate::impl_urn_generator;
use crate::shared_kernel::enums::DeploymentModality;
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::TimeStamp;
use openapiv3::OpenAPI;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

use super::deployment_option::{DeploymentOption, DeploymentOptionId, ServingRuntime};
use super::hpc_cluster::{BatchSchedulerQueueId, HpcClusterId};

#[derive(Clone, Debug, Error)]
pub enum ModelDeploymentError {
    #[error("Invalid state change. Cannot move from state '{0}' to {1}")]
    InvalidStateTransition(String, String),

    #[error("Invalid desired state change. Cannot move from desired state '{0}' to {1}")]
    InvalidDesiredStateTransition(String, String),

    #[error("Selected deployment option does not support modality {0}")]
    UnsupportedDeploymentModality(DeploymentModality),

    #[error("Deployment option snapshot does not match the selected deployment option")]
    InvalidDeploymentOptionSnapshot,
}

#[derive(Clone, Debug)]
pub struct ModelDeployment {
    /// The unique identifier of this deployment
    pub id: Uuid,
    /// Display name of the deployment
    pub name: String,
    /// The modality of the deployment
    pub deployment_modality: DeploymentModality,
    /// Description of the deployment
    pub description: Option<String>,
    /// The id of the tenant to which this model deployment belongs
    pub tenant_id: String,
    /// The user that owns this deployment
    pub owner: String,
    /// Id of the external model
    pub external_model_id: ExternalModelId,
    /// The curent state of the deployment
    pub state: State,
    /// The state the user would like the deployment to be in
    pub desired_state: DesiredState,
    /// The last message associated with the last state or desired state change
    pub last_message: Option<String>,
    /// The option the Model was deployed with
    pub deployment_option_id: DeploymentOptionId,
    /// Resolved execution information retained for the lifetime of the deployment.
    pub deployment_option_snapshot: DeploymentOptionSnapshot,
    pub visibility: Visibility,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    // pub last_observed: Option<TimeStamp>,
    pub last_desired_state_change: TimeStamp,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: ReplicaGroup,
    /// Metadata provided by and for deployment clients
    pub metadata: Option<ModelDeploymentMetadata>,
    /// Indicates changes to desired state over time. This field is incremented
    /// every time desired state changes.
    revision: u32,
}

impl_urn_generator!(ModelDeployment, tenant_id, "deployment", id);

impl ModelDeployment {
    /// Create the model deployment from a deployment option
    pub fn create_from_option(
        props: CreateFromOptionProps,
        option: &DeploymentOption,
        snapshot: DeploymentOptionSnapshot,
    ) -> Result<Self, ModelDeploymentError> {
        // Invariant: The selected option must support the selected deployment modality.
        if !option
            .supported_deployment_modalities()
            .contains(&props.deployment_modality)
        {
            return Err(ModelDeploymentError::UnsupportedDeploymentModality(
                props.deployment_modality,
            ));
        }

        if snapshot.external_model_id != *option.external_model_id()
            || snapshot.deployment_option_id != *option.id()
            || snapshot.serving_runtime != *option.serving_runtime()
        {
            return Err(ModelDeploymentError::InvalidDeploymentOptionSnapshot);
        }

        let now = TimeStamp::now();

        Ok(Self {
            id: Uuid::now_v7(),
            name: props.name,
            description: props.description,
            tenant_id: props.tenant_id,
            owner: props.owner,
            external_model_id: *option.external_model_id(),
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: props.last_message,
            deployment_modality: props.deployment_modality.clone(),
            deployment_option_id: *option.id(),
            deployment_option_snapshot: snapshot,
            visibility: props.visibility,
            created_at: now.clone(),
            last_modified: now.clone(),
            last_state_change: now.clone(),
            last_desired_state_change: now.clone(),
            deployment_interface: props.deployment_interface,
            replicas: props.replicas,
            metadata: props.metadata,
            revision: 0,
        })
    }

    pub fn reconstitute(props: ReconstituteModelDeploymentProps) -> Self {
        Self {
            id: props.id,
            name: props.name,
            description: props.description,
            tenant_id: props.tenant_id,
            deployment_modality: props.deployment_modality,
            deployment_option_id: props.deployment_option_id,
            deployment_option_snapshot: props.deployment_option_snapshot,
            owner: props.owner,
            external_model_id: props.external_model_id,
            state: props.state,
            desired_state: props.desired_state,
            last_message: props.last_message,
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

    pub fn resolve_reconciliation_requirement(&self) -> Option<ReconciliationRequirement> {
        if self.is_state_syncronized() {
            return None
        }

        match (&self.state, &self.desired_state) {
            (State::NotDeployed, DesiredState::Running)
            | (State::Stopped, DesiredState::Running)
            | (State::Failed, DesiredState::Running)
            | (State::Blocked, DesiredState::Running) => Some(ReconciliationRequirement::Start),

            (_, DesiredState::NotDeployed) => Some(ReconciliationRequirement::Undeploy),

            (State::Running, DesiredState::Stopped) => Some(ReconciliationRequirement::Stop),

            _ => None,
        }
    }

    pub fn is_state_syncronized(&self) -> bool {
        match (&self.state, &self.desired_state) {
            (State::Running, DesiredState::Running) => true,
            (State::Stopped, DesiredState::Stopped) => true,
            (State::NotDeployed, DesiredState::NotDeployed) => true,
            _ => false,
        }
    }

    pub fn revision(&self) -> &u32 {
        &self.revision
    }

    pub fn revise(&mut self) -> ModelDeploymentDraft<'_> {
        let revision = self.revision + 1;

        let draft = ModelDeploymentDraft {
            deployment: self,
            revision,
        };

        draft
    }

    pub fn mark_as_failed(&mut self, message: Option<String>) -> Result<(), ModelDeploymentError> {
        self.revise()
            .transition_to_state(State::Failed, message)?
            .finish();

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReconciliationRequirement {
    Start,
    Stop,
    Undeploy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeploymentOptionSnapshot {
    pub deployment_option_id: DeploymentOptionId,
    pub external_model_id: ExternalModelId,
    pub serving_runtime: ServingRuntime,
    pub target: DeploymentTargetSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeploymentTargetSnapshot {
    HpcClusterQueue(HpcClusterQueueSnapshot),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HpcClusterQueueSnapshot {
    pub hpc_cluster_id: HpcClusterId,
    pub batch_scheduler_queue_id: BatchSchedulerQueueId,
    pub cluster_host: String,
    pub queue_name: String,
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

impl PartialEq<DesiredState> for State {
    fn eq(&self, other: &DesiredState) -> bool {
        match (self, other) {
            (&State::NotDeployed, &DesiredState::NotDeployed) => true,
            (&State::Stopped, &DesiredState::Stopped) => true,
            (&State::Running, &DesiredState::Running) => true,
            _ => false,
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

#[derive(Clone, Debug, Default)]
pub enum ModelDeploymentMetadataDelta {
    #[default]
    NoChange,
    Delete,
    Merge(ModelDeploymentMetadata),
}

#[derive(Clone, Debug)]
pub struct ReplicaGroup {
    /// Number of replicas
    pub count: u8,

    /// Sharding / parallelism strategies actually employed by the deployment runtime.
    pub parallelism_strategies: Vec<ParallelismStrategy>,
}

impl Default for ReplicaGroup {
    fn default() -> Self {
        Self {
            count: 1,
            parallelism_strategies: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum ParallelismStrategy {
    PipelineParallelism,
    TensorParallelism,
    SequenceParallelism,
    ContextParallelism,
    ExpertParallelism,
}

#[derive(Clone, Debug, Default)]
pub enum ReplicaGroupDelta {
    #[default]
    NoChange,
    Delete,
    Replace(ReplicaGroup),
}

#[derive(Clone, Debug)]
pub enum ModelDeploymentInterface {
    RestApi(RestApi),
}

#[derive(Clone, Debug)]
pub struct RestApi {
    pub spec: OpenAPI,
}

#[derive(Clone, Debug, Default)]
pub enum ModelDeploymentInterfaceDelta {
    #[default]
    NoChange,
    Delete,
    Replace(ModelDeploymentInterface),
}

#[derive(Debug)]
pub struct ModelDeploymentDraft<'a> {
    deployment: &'a mut ModelDeployment,
    revision: u32,
}

impl<'a> ModelDeploymentDraft<'a> {
    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) -> &mut Self {
        let now = TimeStamp::now();

        self.deployment.last_modified = now.clone();

        self
    }

    fn valid_state_transitions() -> HashMap<State, Vec<State>> {
        let mut transitions = HashMap::new();

        transitions.insert(
            State::NotDeployed,
            vec![State::Blocked, State::Running, State::Failed],
        );
        transitions.insert(
            State::Running,
            vec![State::Blocked, State::Stopped, State::Failed],
        );
        transitions.insert(
            State::Stopped,
            vec![State::Blocked, State::Running, State::Failed],
        );
        transitions.insert(State::Failed, vec![State::Blocked, State::Running]);
        transitions.insert(
            State::Blocked,
            vec![State::Running, State::Stopped, State::Failed],
        );
        transitions
    }

    /// Returns whether a transition from one state to another is valid
    fn is_valid_state_transition(from: &State, to: &State) -> bool {
        // Unknown can transition to or from any state
        if from == &State::Unknown || to == &State::Unknown {
            return true;
        }

        Self::valid_state_transitions()
            .get(from)
            .map_or(false, |allowed| allowed.contains(to))
    }

    /// Changes the state. Returns an error if invalid state transition is detected
    pub fn transition_to_state(
        &mut self,
        new_state: State,
        message: Option<String>,
    ) -> Result<&mut Self, ModelDeploymentError> {
        if !Self::is_valid_state_transition(&self.deployment.state, &new_state) {
            return Err(ModelDeploymentError::InvalidStateTransition(
                self.deployment.state.clone().into(),
                new_state.into(),
            ));
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
    pub fn transition_to_desired(
        &mut self,
        new_state: DesiredState,
        message: Option<String>,
    ) -> Result<&mut Self, ModelDeploymentError> {
        if !Self::is_valid_desired_state_transition(&self.deployment.desired_state, &new_state) {
            return Err(ModelDeploymentError::InvalidDesiredStateTransition(
                self.deployment.state.clone().into(),
                new_state.into(),
            ));
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
            ModelDeploymentInterfaceDelta::Delete => self.deployment.metadata = None,
            ModelDeploymentInterfaceDelta::Replace(i) => {
                self.deployment.deployment_interface = Some(i);
            }
            ModelDeploymentInterfaceDelta::NoChange => {}
        };

        self
    }

    pub fn apply_replica_group_delta(&mut self, delta: ReplicaGroupDelta) -> &mut Self {
        match delta {
            ReplicaGroupDelta::Delete => self.deployment.metadata = None,
            ReplicaGroupDelta::Replace(r) => {
                self.deployment.replicas = r;
            }
            ReplicaGroupDelta::NoChange => {}
        };

        self
    }

    pub fn apply_metadata_delta(&mut self, delta: ModelDeploymentMetadataDelta) -> &mut Self {
        match delta {
            ModelDeploymentMetadataDelta::Delete => self.deployment.metadata = None,
            ModelDeploymentMetadataDelta::Merge(m) => {
                self.merge_metadata(m);
            }
            ModelDeploymentMetadataDelta::NoChange => {}
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
        if let Some(ref mut m) = self
            .deployment
            .metadata
            .as_mut()
            .and_then(|m| Some(m.into_inner().clone()))
        {
            m.insert(k, v);
            return self;
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
pub struct ReconstituteModelDeploymentProps {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub deployment_modality: DeploymentModality,
    pub tenant_id: String,
    pub owner: String,
    pub external_model_id: ExternalModelId,
    pub state: State,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
    pub deployment_option_id: DeploymentOptionId,
    pub deployment_option_snapshot: DeploymentOptionSnapshot,
    pub visibility: Visibility,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: ReplicaGroup,
    pub revision: u32,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    pub last_desired_state_change: TimeStamp,
    pub created_at: TimeStamp,
    pub metadata: Option<ModelDeploymentMetadata>,
}

#[derive(Clone, Debug)]
pub struct CreateFromOptionProps {
    pub name: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub owner: String,
    pub last_message: Option<String>,
    pub visibility: Visibility,
    pub deployment_modality: DeploymentModality,
    pub deployment_interface: Option<ModelDeploymentInterface>,
    pub replicas: ReplicaGroup,
    pub metadata: Option<ModelDeploymentMetadata>,
}
