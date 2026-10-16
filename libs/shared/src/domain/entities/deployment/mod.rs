pub mod argument;
mod model_deployment_id;

pub use model_deployment_id::ModelDeploymentId;

use crate::domain::entities::endpoint::NetworkAddressableResource;
use crate::domain::entities::model::external_model::ExternalModelId;
use crate::domain::entities::tenancy::TenantScopedResource;
use crate::impl_urn_generator;
use crate::shared_kernel::enums::DeploymentModality;
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::TimeStamp;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use url::Url;

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

    #[error("Maximum runtime must be greater than zero")]
    InvalidMaximumRuntime,

    #[error("Maximum runtime exceeds the supported timestamp range")]
    MaximumRuntimeOverflow,

    #[error("Model deployment endpoint name must not be empty")]
    InvalidEndpointName,

    #[error("Model deployment endpoint base URL must be an absolute HTTP or HTTPS URL")]
    InvalidEndpointBaseUrl,

    #[error("Invalid reconciliation state: {0}")]
    InvalidReconciliationState(String),

    #[error("Invalid persisted ModelDeployment data: {0}")]
    DataIntegrityError(String),
}

#[derive(Clone, Debug)]
pub struct ModelDeployment {
    /// The unique identifier of this deployment
    id: ModelDeploymentId,
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
    last_observed: Option<TimeStamp>,
    maximum_runtime: Option<Duration>,
    expires_at: Option<TimeStamp>,
    pub last_desired_state_change: TimeStamp,
    target_endpoint: Option<ModelDeploymentEndpoint>,
    pub replicas: ReplicaGroup,
    /// Metadata provided by and for deployment clients
    pub metadata: Option<ModelDeploymentMetadata>,
    reconciliation_status: Option<ReconciliationStatus>,
    retries: u32,
    retry_after: Option<TimeStamp>,
    /// Indicates changes to desired state over time. This field is incremented
    /// every time desired state changes.
    revision: u32,
}

impl_urn_generator!(ModelDeployment, tenant_id, "deployment", id);

impl TenantScopedResource for ModelDeployment {
    fn tenant_id(&self) -> String {
        self.tenant_id.clone()
    }
}

impl NetworkAddressableResource for ModelDeployment {
    fn owner(&self) -> &str {
        self.owner()
    }

    fn network_target_names(&self) -> Vec<String> {
        self.target_endpoint
            .as_ref()
            .and_then(|endpoint| {
                endpoint
                    .base_url()
                    .and_then(|_| endpoint.name().map(String::from))
            })
            .into_iter()
            .collect()
    }

    fn resolve_target_url(&self, target_name: &str) -> Option<&str> {
        let endpoint = self.target_endpoint.as_ref()?;

        if endpoint.name()? != target_name {
            return None;
        }

        endpoint.base_url()
    }
}

impl ModelDeployment {
    /// Create the model deployment from a deployment option
    pub fn create_from_option(
        props: CreateFromOptionProps,
        option: &DeploymentOption,
        snapshot: DeploymentOptionSnapshot,
        maximum_runtime: Option<Duration>,
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

        Self::validate_maximum_runtime(maximum_runtime)?;

        let now = TimeStamp::now();

        Ok(Self {
            id: ModelDeploymentId::new(),
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
            last_observed: None,
            maximum_runtime,
            expires_at: None,
            last_desired_state_change: now.clone(),
            target_endpoint: props.target_endpoint,
            replicas: props.replicas,
            metadata: props.metadata,
            reconciliation_status: None,
            retries: 0,
            retry_after: None,
            revision: 0,
        })
    }

    pub fn reconstitute(
        props: ReconstituteModelDeploymentProps,
    ) -> Result<Self, ModelDeploymentError> {
        Self::validate_maximum_runtime(props.maximum_runtime)
            .map_err(|error| ModelDeploymentError::DataIntegrityError(error.to_string()))?;

        if props.expires_at.is_some() && props.maximum_runtime.is_none() {
            return Err(ModelDeploymentError::DataIntegrityError(
                "expires_at requires maximum_runtime".into(),
            ));
        }

        if props.state == State::Running
            && props.maximum_runtime.is_some()
            && props.expires_at.is_none()
        {
            return Err(ModelDeploymentError::DataIntegrityError(
                "Running deployments with a maximum runtime require expires_at".into(),
            ));
        }

        if props
            .expires_at
            .as_ref()
            .is_some_and(|expires_at| expires_at.into_inner() < props.created_at.into_inner())
        {
            return Err(ModelDeploymentError::DataIntegrityError(
                "expires_at cannot precede created_at".into(),
            ));
        }

        Self::validate_reconciliation_state(
            props.reconciliation_status.as_ref(),
            props.retries,
            props.retry_after.as_ref(),
            &props.state,
            &props.desired_state,
        )
        .map_err(|error| ModelDeploymentError::DataIntegrityError(error.to_string()))?;

        Ok(Self {
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
            last_observed: props.last_observed,
            maximum_runtime: props.maximum_runtime,
            expires_at: props.expires_at,
            last_desired_state_change: props.last_desired_state_change,
            target_endpoint: props.target_endpoint,
            replicas: props.replicas,
            metadata: props.metadata,
            reconciliation_status: props.reconciliation_status,
            retries: props.retries,
            retry_after: props.retry_after,
            revision: props.revision,
        })
    }

    pub fn id(&self) -> &ModelDeploymentId {
        &self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn resolve_reconciliation_requirement(
        &self,
        evaluated_at: &TimeStamp,
    ) -> Option<ReconciliationRequirement> {
        if self.is_expired(evaluated_at) || self.is_state_syncronized() {
            return None;
        }

        if matches!(
            self.reconciliation_status,
            Some(ReconciliationStatus::Failed)
        ) {
            return None;
        }

        if self.is_waiting_for_retry(evaluated_at) {
            return None;
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

    pub fn reconciliation_status(&self) -> Option<&ReconciliationStatus> {
        self.reconciliation_status.as_ref()
    }

    pub fn retries(&self) -> u32 {
        self.retries
    }

    pub fn retry_after(&self) -> Option<&TimeStamp> {
        self.retry_after.as_ref()
    }

    pub fn is_waiting_for_retry(&self, evaluated_at: &TimeStamp) -> bool {
        matches!(
            self.reconciliation_status,
            Some(ReconciliationStatus::Retrying)
        ) && self
            .retry_after
            .as_ref()
            .is_some_and(|retry_after| retry_after.into_inner() > evaluated_at.into_inner())
    }

    pub fn accept_for_reconciliation(&mut self) {
        self.set_reconciliation_state(Some(ReconciliationStatus::Pending), 0, None);
    }

    pub fn begin_reconciliation(&mut self, requirement: &ReconciliationRequirement) {
        let status = match requirement {
            ReconciliationRequirement::Start => ReconciliationStatus::Starting,
            ReconciliationRequirement::Stop => ReconciliationStatus::Stopping,
            ReconciliationRequirement::Undeploy => ReconciliationStatus::Undeploying,
        };

        self.set_reconciliation_state(Some(status), self.retries, None);
    }

    pub fn schedule_reconciliation_retry(
        &mut self,
        retry_after: TimeStamp,
    ) -> Result<(), ModelDeploymentError> {
        let retries = self.retries.checked_add(1).ok_or_else(|| {
            ModelDeploymentError::InvalidReconciliationState(
                "reconciliation retry count overflowed".into(),
            )
        })?;

        self.set_reconciliation_state(
            Some(ReconciliationStatus::Retrying),
            retries,
            Some(retry_after),
        );

        Ok(())
    }

    pub fn mark_reconciliation_failed(&mut self) {
        self.set_reconciliation_state(Some(ReconciliationStatus::Failed), self.retries, None);
    }

    pub fn mark_reconciliation_synchronized(&mut self) -> Result<(), ModelDeploymentError> {
        if !self.is_state_syncronized() {
            return Err(ModelDeploymentError::InvalidReconciliationState(
                "a deployment can only be synchronized when actual and desired state agree".into(),
            ));
        }

        self.set_reconciliation_state(Some(ReconciliationStatus::Synchronized), 0, None);

        Ok(())
    }

    pub fn reset_reconciliation(&mut self) {
        self.set_reconciliation_state(None, 0, None);
    }

    pub fn last_observed(&self) -> Option<&TimeStamp> {
        self.last_observed.as_ref()
    }

    pub fn target_endpoint(&self) -> Option<&ModelDeploymentEndpoint> {
        self.target_endpoint.as_ref()
    }

    pub fn maximum_runtime(&self) -> Option<Duration> {
        self.maximum_runtime
    }

    pub fn expires_at(&self) -> Option<&TimeStamp> {
        self.expires_at.as_ref()
    }

    pub fn is_expired(&self, evaluated_at: &TimeStamp) -> bool {
        self.expires_at
            .as_ref()
            .is_some_and(|expires_at| evaluated_at.into_inner() >= expires_at.into_inner())
    }

    pub fn record_observation(&mut self, observed_at: TimeStamp) {
        let should_update = self
            .last_observed
            .as_ref()
            .map(|current| current.into_inner() < observed_at.into_inner())
            .unwrap_or(true);

        if should_update {
            self.last_observed = Some(observed_at);
        }
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
            .apply_target_endpoint_delta(ModelDeploymentEndpointDelta::Delete)
            .finish();

        Ok(())
    }

    fn validate_maximum_runtime(
        maximum_runtime: Option<Duration>,
    ) -> Result<(), ModelDeploymentError> {
        let Some(maximum_runtime) = maximum_runtime else {
            return Ok(());
        };

        if maximum_runtime.is_zero() {
            return Err(ModelDeploymentError::InvalidMaximumRuntime);
        }

        chrono::Duration::from_std(maximum_runtime)
            .map_err(|_| ModelDeploymentError::MaximumRuntimeOverflow)?;

        Ok(())
    }

    fn validate_reconciliation_state(
        status: Option<&ReconciliationStatus>,
        retries: u32,
        retry_after: Option<&TimeStamp>,
        state: &State,
        desired_state: &DesiredState,
    ) -> Result<(), ModelDeploymentError> {
        if matches!(status, Some(ReconciliationStatus::Retrying)) {
            if retries == 0 || retry_after.is_none() {
                return Err(ModelDeploymentError::InvalidReconciliationState(
                    "Retrying requires a positive retry count and retry_after".into(),
                ));
            }
        } else if retry_after.is_some() {
            return Err(ModelDeploymentError::InvalidReconciliationState(
                "retry_after is only valid while Retrying".into(),
            ));
        }

        if matches!(status, Some(ReconciliationStatus::Synchronized)) && state != desired_state {
            return Err(ModelDeploymentError::InvalidReconciliationState(
                "Synchronized requires actual and desired state to agree".into(),
            ));
        }

        Ok(())
    }

    fn set_reconciliation_state(
        &mut self,
        status: Option<ReconciliationStatus>,
        retries: u32,
        retry_after: Option<TimeStamp>,
    ) {
        self.reconciliation_status = status;
        self.retries = retries;
        self.retry_after = retry_after;

        self.last_modified = TimeStamp::now();
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReconciliationStatus {
    Pending,
    Starting,
    Stopping,
    Undeploying,
    Retrying,
    Synchronized,
    Failed,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Protocol {
    RestHttp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDeploymentEndpoint {
    name: Option<String>,
    protocol: Protocol,
    base_url: Option<String>,
}

impl ModelDeploymentEndpoint {
    pub fn new(
        name: Option<String>,
        protocol: Protocol,
        base_url: Option<String>,
    ) -> Result<Self, ModelDeploymentError> {
        Self::validate_name(name.as_deref())?;
        Self::validate_base_url(&protocol, base_url.as_deref())?;

        Ok(Self {
            name,
            protocol,
            base_url,
        })
    }

    pub fn reconstitute(
        name: Option<String>,
        protocol: Protocol,
        base_url: Option<String>,
    ) -> Result<Self, ModelDeploymentError> {
        Self::new(name, protocol, base_url)
            .map_err(|error| ModelDeploymentError::DataIntegrityError(error.to_string()))
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    fn validate_name(name: Option<&str>) -> Result<(), ModelDeploymentError> {
        if name.is_some_and(|name| name.trim().is_empty()) {
            return Err(ModelDeploymentError::InvalidEndpointName);
        }

        Ok(())
    }

    fn validate_base_url(
        protocol: &Protocol,
        base_url: Option<&str>,
    ) -> Result<(), ModelDeploymentError> {
        let Some(base_url) = base_url else {
            return Ok(());
        };

        if base_url.trim().is_empty() {
            return Err(ModelDeploymentError::InvalidEndpointBaseUrl);
        }

        let parsed =
            Url::parse(base_url).map_err(|_| ModelDeploymentError::InvalidEndpointBaseUrl)?;

        match protocol {
            Protocol::RestHttp
                if matches!(parsed.scheme(), "http" | "https") && parsed.has_host() => {}
            Protocol::RestHttp => return Err(ModelDeploymentError::InvalidEndpointBaseUrl),
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ModelDeploymentEndpointDelta {
    #[default]
    NoChange,
    Delete,
    Replace(ModelDeploymentEndpoint),
}

impl ModelDeploymentEndpointDelta {
    pub fn is_no_change(&self) -> bool {
        matches!(self, Self::NoChange)
    }
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
        self.transition_to_state_at(new_state, message, TimeStamp::now())
    }

    pub fn transition_to_state_at(
        &mut self,
        new_state: State,
        message: Option<String>,
        transitioned_at: TimeStamp,
    ) -> Result<&mut Self, ModelDeploymentError> {
        if !Self::is_valid_state_transition(&self.deployment.state, &new_state) {
            return Err(ModelDeploymentError::InvalidStateTransition(
                self.deployment.state.clone().into(),
                new_state.into(),
            ));
        }

        let expires_at = if new_state == State::Running && self.deployment.expires_at.is_none() {
            self.deployment
                .maximum_runtime
                .map(|maximum_runtime| {
                    let duration = chrono::Duration::from_std(maximum_runtime)
                        .map_err(|_| ModelDeploymentError::MaximumRuntimeOverflow)?;

                    transitioned_at
                        .into_inner()
                        .checked_add_signed(duration)
                        .map(TimeStamp::from)
                        .ok_or(ModelDeploymentError::MaximumRuntimeOverflow)
                })
                .transpose()?
        } else {
            self.deployment.expires_at.clone()
        };

        // Changes the state
        self.deployment.state = new_state;

        self.deployment.expires_at = expires_at;

        // Update the last message if provided
        if let Some(m) = message {
            self.deployment.last_message = Some(m);
        }

        self.deployment.last_state_change = transitioned_at;

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

        self.deployment.reset_reconciliation();

        Ok(self)
    }

    pub fn restart_reconciliation(&mut self, message: Option<String>) -> &mut Self {
        if let Some(message) = message {
            self.deployment.last_message = Some(message);
        }

        self.deployment.last_desired_state_change = TimeStamp::now();

        self.deployment.reset_reconciliation();

        self
    }

    pub fn apply_target_endpoint_delta(
        &mut self,
        delta: ModelDeploymentEndpointDelta,
    ) -> &mut Self {
        match delta {
            ModelDeploymentEndpointDelta::Delete => self.deployment.target_endpoint = None,
            ModelDeploymentEndpointDelta::Replace(endpoint) => {
                self.deployment.target_endpoint = Some(endpoint);
            }
            ModelDeploymentEndpointDelta::NoChange => {}
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
    pub id: ModelDeploymentId,
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
    pub target_endpoint: Option<ModelDeploymentEndpoint>,
    pub replicas: ReplicaGroup,
    pub revision: u32,
    pub last_modified: TimeStamp,
    pub last_state_change: TimeStamp,
    pub last_observed: Option<TimeStamp>,
    pub maximum_runtime: Option<Duration>,
    pub expires_at: Option<TimeStamp>,
    pub last_desired_state_change: TimeStamp,
    pub created_at: TimeStamp,
    pub metadata: Option<ModelDeploymentMetadata>,
    pub reconciliation_status: Option<ReconciliationStatus>,
    pub retries: u32,
    pub retry_after: Option<TimeStamp>,
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
    pub target_endpoint: Option<ModelDeploymentEndpoint>,
    pub replicas: ReplicaGroup,
    pub metadata: Option<ModelDeploymentMetadata>,
}

#[cfg(test)]
#[path = "model_deployment.test.rs"]
mod model_deployment_test;
