use std::error::Error;

use super::*;

struct ModelDeploymentTestBuilder {
    last_observed: Option<TimeStamp>,
    last_modified: Option<TimeStamp>,
    revision: u32,
}

impl ModelDeploymentTestBuilder {
    fn new() -> Self {
        Self {
            last_observed: None,
            last_modified: None,
            revision: 3,
        }
    }

    fn last_observed(mut self, last_observed: TimeStamp) -> Self {
        self.last_observed = Some(last_observed);

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
                state: State::Running,
                desired_state: DesiredState::Running,
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
                last_desired_state_change: created_at.clone(),
                created_at,
                metadata: None,
            },
        ))
    }
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
