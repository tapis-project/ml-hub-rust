use std::error::Error;
use std::time::Duration;

use nonempty::nonempty;

use crate::domain::entities::deployment_option::{
    DeploymentTarget, HpcClusterQueueReference, NewDeploymentOptionProps,
};

use super::*;

fn create_deployment(maximum_runtime: Option<Duration>) -> Result<ModelDeployment, Box<dyn Error>> {
    let external_model_id = ExternalModelId::new();
    let hpc_cluster_id = HpcClusterId::new();
    let batch_scheduler_queue_id = BatchSchedulerQueueId::new();
    let option = DeploymentOption::new(NewDeploymentOptionProps {
        external_model_id,
        supported_deployment_modalities: nonempty![DeploymentModality::Batch],
        deployment_target: DeploymentTarget::HpcClusterQueue(HpcClusterQueueReference::new(
            hpc_cluster_id,
            batch_scheduler_queue_id,
        )),
        serving_runtime: ServingRuntime::FlexServ,
    })?;

    let snapshot = DeploymentOptionSnapshot {
        deployment_option_id: *option.id(),
        external_model_id,
        serving_runtime: ServingRuntime::FlexServ,
        target: DeploymentTargetSnapshot::HpcClusterQueue(HpcClusterQueueSnapshot {
            hpc_cluster_id,
            batch_scheduler_queue_id,
            cluster_host: "test.example.org".into(),
            queue_name: "gpu".into(),
        }),
    };

    Ok(ModelDeployment::create_from_option(
        CreateFromOptionProps {
            name: "Test deployment".into(),
            description: None,
            tenant_id: "test-tenant".into(),
            owner: "test-user".into(),
            last_message: None,
            visibility: Visibility::Private,
            deployment_modality: DeploymentModality::Batch,
            deployment_interface: None,
            replicas: ReplicaGroup::default(),
            metadata: None,
        },
        &option,
        snapshot,
        maximum_runtime,
    )?)
}

struct ModelDeploymentTestBuilder {
    last_observed: Option<TimeStamp>,
    last_modified: Option<TimeStamp>,
    maximum_runtime: Option<Duration>,
    expires_at: Option<TimeStamp>,
    state: State,
    desired_state: DesiredState,
    revision: u32,
}

impl ModelDeploymentTestBuilder {
    fn new() -> Self {
        Self {
            last_observed: None,
            last_modified: None,
            maximum_runtime: None,
            expires_at: None,
            state: State::Running,
            desired_state: DesiredState::Running,
            revision: 3,
        }
    }

    fn last_observed(mut self, last_observed: TimeStamp) -> Self {
        self.last_observed = Some(last_observed);

        self
    }

    fn maximum_runtime(mut self, maximum_runtime: Duration) -> Self {
        self.maximum_runtime = Some(maximum_runtime);

        self
    }

    fn expires_at(mut self, expires_at: TimeStamp) -> Self {
        self.expires_at = Some(expires_at);

        self
    }

    fn state(mut self, state: State) -> Self {
        self.state = state;

        self
    }

    fn desired_state(mut self, desired_state: DesiredState) -> Self {
        self.desired_state = desired_state;

        self
    }

    fn build(self) -> Result<ModelDeployment, Box<dyn Error>> {
        let created_at = TimeStamp::parse_string("2026-10-01T10:00:00Z")?;
        let last_modified = self.last_modified.unwrap_or(created_at.clone());
        let external_model_id = ExternalModelId::new();
        let deployment_option_id = DeploymentOptionId::new();

        Ok(ModelDeployment::reconstitute(
            ReconstituteModelDeploymentProps {
                id: ModelDeploymentId::new(),
                name: "Test deployment".into(),
                description: None,
                deployment_modality: DeploymentModality::Batch,
                tenant_id: "test-tenant".into(),
                owner: "test-user".into(),
                external_model_id,
                state: self.state,
                desired_state: self.desired_state,
                last_message: None,
                deployment_option_id,
                deployment_option_snapshot: DeploymentOptionSnapshot {
                    deployment_option_id,
                    external_model_id,
                    serving_runtime: ServingRuntime::FlexServ,
                    target: DeploymentTargetSnapshot::HpcClusterQueue(HpcClusterQueueSnapshot {
                        hpc_cluster_id: HpcClusterId::new(),
                        batch_scheduler_queue_id: BatchSchedulerQueueId::new(),
                        cluster_host: "test.example.org".into(),
                        queue_name: "gpu".into(),
                    }),
                },
                visibility: Visibility::Private,
                deployment_interface: None,
                replicas: ReplicaGroup::default(),
                revision: self.revision,
                last_modified,
                last_state_change: created_at.clone(),
                last_observed: self.last_observed,
                maximum_runtime: self.maximum_runtime,
                expires_at: self.expires_at,
                last_desired_state_change: created_at.clone(),
                created_at,
                metadata: None,
            },
        )?)
    }
}

#[test]
fn creation_preserves_optional_maximum_runtime_without_assigning_expiration(
) -> Result<(), Box<dyn Error>> {
    let maximum_runtime = Duration::from_secs(2 * 60 * 60);

    let timed = create_deployment(Some(maximum_runtime))?;
    let untimed = create_deployment(None)?;

    assert_eq!(timed.maximum_runtime(), Some(maximum_runtime));
    assert_eq!(timed.expires_at(), None);
    assert_eq!(untimed.maximum_runtime(), None);
    assert_eq!(untimed.expires_at(), None);

    Ok(())
}

#[test]
fn reconstitution_preserves_last_observed() -> Result<(), Box<dyn Error>> {
    let observed_at = TimeStamp::parse_string("2026-10-01T11:00:00Z")?;
    let deployment = ModelDeploymentTestBuilder::new()
        .last_observed(observed_at.clone())
        .build()?;

    assert_eq!(deployment.last_observed(), Some(&observed_at));

    Ok(())
}

#[test]
fn recording_observation_does_not_revise_or_touch_deployment() -> Result<(), Box<dyn Error>> {
    let mut deployment = ModelDeploymentTestBuilder::new().build()?;
    let revision = *deployment.revision();
    let last_modified = deployment.last_modified.clone();
    let observed_at = TimeStamp::parse_string("2026-10-01T11:00:00Z")?;

    deployment.record_observation(observed_at.clone());

    assert_eq!(deployment.last_observed(), Some(&observed_at));
    assert_eq!(deployment.revision(), &revision);
    assert_eq!(deployment.last_modified, last_modified);

    Ok(())
}

#[test]
fn recording_observation_never_moves_timestamp_backward() -> Result<(), Box<dyn Error>> {
    let current = TimeStamp::parse_string("2026-10-01T12:00:00Z")?;
    let older = TimeStamp::parse_string("2026-10-01T11:00:00Z")?;
    let mut deployment = ModelDeploymentTestBuilder::new()
        .last_observed(current.clone())
        .build()?;

    deployment.record_observation(older);

    assert_eq!(deployment.last_observed(), Some(&current));

    Ok(())
}

#[test]
fn first_running_transition_sets_expiration_from_transition_time() -> Result<(), Box<dyn Error>> {
    let transitioned_at = TimeStamp::parse_string("2026-10-01T13:00:00Z")?;
    let expected_expiration = TimeStamp::parse_string("2026-10-01T15:00:00Z")?;
    let mut deployment = ModelDeploymentTestBuilder::new()
        .maximum_runtime(Duration::from_secs(2 * 60 * 60))
        .state(State::Unknown)
        .build()?;

    deployment
        .revise()
        .transition_to_state_at(State::Running, None, transitioned_at)?
        .finish();

    assert_eq!(deployment.expires_at(), Some(&expected_expiration));

    Ok(())
}

#[test]
fn restarting_does_not_extend_expiration() -> Result<(), Box<dyn Error>> {
    let expiration = TimeStamp::parse_string("2026-10-01T15:00:00Z")?;
    let stopped_at = TimeStamp::parse_string("2026-10-01T14:00:00Z")?;
    let restarted_at = TimeStamp::parse_string("2026-10-01T14:30:00Z")?;
    let mut deployment = ModelDeploymentTestBuilder::new()
        .maximum_runtime(Duration::from_secs(2 * 60 * 60))
        .expires_at(expiration.clone())
        .build()?;

    deployment
        .revise()
        .transition_to_state_at(State::Stopped, None, stopped_at)?
        .finish();

    deployment
        .revise()
        .transition_to_state_at(State::Running, None, restarted_at)?
        .finish();

    assert_eq!(deployment.expires_at(), Some(&expiration));

    Ok(())
}

#[test]
fn expiration_is_inclusive_and_suppresses_reconciliation() -> Result<(), Box<dyn Error>> {
    let expiration = TimeStamp::parse_string("2026-10-01T15:00:00Z")?;
    let deployment = ModelDeploymentTestBuilder::new()
        .maximum_runtime(Duration::from_secs(2 * 60 * 60))
        .expires_at(expiration.clone())
        .state(State::Stopped)
        .desired_state(DesiredState::Running)
        .build()?;

    assert!(deployment.is_expired(&expiration));
    assert_eq!(
        deployment.resolve_reconciliation_requirement(&expiration),
        None
    );

    Ok(())
}

#[test]
fn reconstitution_rejects_expiration_without_maximum_runtime() -> Result<(), Box<dyn Error>> {
    let expiration = TimeStamp::parse_string("2026-10-01T15:00:00Z")?;
    let result = ModelDeploymentTestBuilder::new()
        .expires_at(expiration)
        .build();

    assert!(matches!(
        result,
        Err(error) if error.to_string().contains("expires_at requires maximum_runtime")
    ));

    Ok(())
}

#[test]
fn reconstitution_rejects_running_deployment_without_expiration() {
    let result = ModelDeploymentTestBuilder::new()
        .maximum_runtime(Duration::from_secs(2 * 60 * 60))
        .build();

    assert!(matches!(
        result,
        Err(error) if error
            .to_string()
            .contains("Running deployments with a maximum runtime require expires_at")
    ));
}

#[test]
fn rejects_zero_and_overflowing_maximum_runtime() {
    assert!(matches!(
        ModelDeployment::validate_maximum_runtime(Some(Duration::ZERO)),
        Err(ModelDeploymentError::InvalidMaximumRuntime)
    ));
    assert!(matches!(
        ModelDeployment::validate_maximum_runtime(Some(Duration::MAX)),
        Err(ModelDeploymentError::MaximumRuntimeOverflow)
    ));
}

#[test]
fn running_transition_rejects_timestamp_overflow() -> Result<(), Box<dyn Error>> {
    let transitioned_at = TimeStamp::from(chrono::DateTime::<chrono::Utc>::MAX_UTC);
    let mut deployment = ModelDeploymentTestBuilder::new()
        .maximum_runtime(Duration::from_secs(1))
        .state(State::Unknown)
        .build()?;

    let mut draft = deployment.revise();

    let result = draft.transition_to_state_at(State::Running, None, transitioned_at);

    assert!(matches!(
        result,
        Err(ModelDeploymentError::MaximumRuntimeOverflow)
    ));

    Ok(())
}
