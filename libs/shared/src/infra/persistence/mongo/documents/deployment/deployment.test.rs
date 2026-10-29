use std::error::Error;

use mongodb::bson::{from_document, to_document, DateTime, Uuid};

use crate::infra::_common::mongo::Index;
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
        batch_execution_outcome: None,
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
        target_endpoint: None,
        replicas: ReplicaGroup {
            count: 1,
            parallelism_strategies: Vec::new(),
        },
        metadata: None,
        reconciliation_status: Some(ReconciliationStatus::Synchronized),
        retries: 0,
        retry_after: None,
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
fn reconciliation_fields_default_for_legacy_documents() -> Result<(), Box<dyn Error>> {
    let mut bson = to_document(&document())?;

    bson.remove("reconciliation_status");
    bson.remove("retries");
    bson.remove("retry_after");

    let deployment = from_document::<ModelDeployment>(bson)?;

    assert!(deployment.reconciliation_status.is_none());
    assert_eq!(deployment.retries, 0);
    assert!(deployment.retry_after.is_none());

    Ok(())
}

#[test]
fn batch_execution_outcome_defaults_to_none_and_round_trips() -> Result<(), Box<dyn Error>> {
    let mut source = document();

    let mut legacy_bson = to_document(&source)?;

    legacy_bson.remove("batch_execution_outcome");

    let legacy = from_document::<ModelDeployment>(legacy_bson)?;

    assert!(legacy.batch_execution_outcome.is_none());

    source.state = State::Stopped;
    source.batch_execution_outcome = Some(BatchExecutionOutcome::TimedOut);

    let persisted = to_document(&source)?;

    let round_trip = from_document::<ModelDeployment>(persisted)?;

    assert!(matches!(
        round_trip.batch_execution_outcome,
        Some(BatchExecutionOutcome::TimedOut)
    ));

    Ok(())
}

#[test]
fn reconciliation_index_supports_status_and_deadline_queries() {
    let index = indexes::ModelDeploymentReconciliationIndex::index();

    assert_eq!(
        index.keys,
        mongodb::bson::doc! {
            "reconciliation_status": 1,
            "retry_after": 1,
        }
    );
}

#[test]
fn target_endpoint_defaults_to_none_and_round_trips() -> Result<(), Box<dyn Error>> {
    let mut source = document();

    let mut legacy_bson = to_document(&source)?;

    legacy_bson.remove("target_endpoint");

    let legacy = from_document::<ModelDeployment>(legacy_bson)?;

    assert!(legacy.target_endpoint.is_none());

    source.target_endpoint = Some(ModelDeploymentEndpoint {
        name: Some("default".into()),
        protocol: Protocol::RestHttp,
        base_url: Some("https://vista.tacc.utexas.edu:60048".into()),
    });

    let persisted = from_document::<ModelDeployment>(to_document(&source)?)?;

    let endpoint = persisted
        .target_endpoint
        .ok_or_else(|| std::io::Error::other("target endpoint should round trip"))?;

    assert_eq!(endpoint.name.as_deref(), Some("default"));
    assert!(matches!(endpoint.protocol, Protocol::RestHttp));
    assert_eq!(
        endpoint.base_url.as_deref(),
        Some("https://vista.tacc.utexas.edu:60048")
    );

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
