use std::error::Error;

use mongodb::bson::{from_document, to_document, DateTime, Uuid};

use crate::shared_kernel::value_objects::TimeStamp;

use super::*;

fn id() -> Uuid {
    Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes())
}

fn document() -> ModelDeployment {
    let now = DateTime::now();
    let external_model_id = id();
    let deployment_option_id = id();

    ModelDeployment {
        _id: None,
        id: id(),
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
            target_type: DeploymentTargetType::HpcClusterQueue,
            hpc_cluster_queue: Some(HpcClusterQueueSnapshot {
                hpc_cluster_id: id(),
                batch_scheduler_queue_id: id(),
                cluster_host: "test.example.org".into(),
                queue_name: "gpu".into(),
            }),
        },
        visibility: Visibility::Private,
        created_at: now,
        last_modified: now,
        last_state_change: now,
        last_observed: None,
        maximum_runtime_seconds: Some(7200),
        expires_at: Some(now),
        last_desired_state_change: now,
        deployment_interface: None,
        replicas: ReplicaGroup {
            count: 1,
            parallelism_strategies: Vec::new(),
        },
        metadata: None,
        revision: 0,
    }
}

#[test]
fn missing_expiration_fields_default_to_none() -> Result<(), Box<dyn Error>> {
    let mut bson = to_document(&document())?;

    bson.remove("maximum_runtime_seconds");
    bson.remove("expires_at");

    let deployment = from_document::<ModelDeployment>(bson)?;

    assert_eq!(deployment.maximum_runtime_seconds, None);
    assert_eq!(deployment.expires_at, None);

    Ok(())
}

#[test]
fn expiration_fields_round_trip() -> Result<(), Box<dyn Error>> {
    let source = document();
    let bson = to_document(&source)?;

    let deployment = from_document::<ModelDeployment>(bson)?;

    assert_eq!(deployment.maximum_runtime_seconds, Some(7200));
    assert_eq!(deployment.expires_at, source.expires_at);

    Ok(())
}

#[test]
fn expiration_fields_map_through_the_domain() -> Result<(), Box<dyn Error>> {
    let source = document();
    let deployment = crate::domain::entities::deployment::ModelDeployment::try_from(&source)
        .map_err(std::io::Error::other)?;

    assert_eq!(
        deployment.maximum_runtime(),
        Some(std::time::Duration::from_secs(7200))
    );
    assert_eq!(
        deployment.expires_at().map(TimeStamp::into_inner),
        source.expires_at.map(|expires_at| expires_at.to_chrono())
    );

    let persisted = ModelDeployment::from(&deployment);

    assert_eq!(persisted.maximum_runtime_seconds, Some(7200));
    assert_eq!(persisted.expires_at, source.expires_at);

    Ok(())
}
