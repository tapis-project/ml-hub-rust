use super::*;
use std::{collections::HashMap, path::PathBuf, rc::Rc};

use evaluations::{Arguments, Evaluator, FieldPath, ResolveValue};
use serde_json::{Map, Value};

use crate::domain::entities::model::external_model::{
    DerivedMetadata, ExternalModel, HuggingFaceRepoLocator, ModelLocator, ModelMetadata,
    ModelProvider,
};

fn hardware_profile() -> HardwareProfile {
    HardwareProfile::new(
        100,
        128,
        "x86_64".into(),
        "EPYC".into(),
        "AMD".into(),
        512,
        Some(Accelerator::new(
            4,
            AcceleratorProfile::Gpu(GpuProfile::new("H100".into(), "NVIDIA".into(), 80, false)),
        )),
    )
}

fn scheduling_policy() -> SchedulingPolicy {
    SchedulingPolicy::new(1, 16, 10, 32, 48, 100)
}

fn queue_props(name: &str) -> NewBatchSchedulerQueueProps {
    NewBatchSchedulerQueueProps {
        enabled: true,
        name: name.into(),
        scheduler_type: SchedulerType::Slurm,
        hardware_profile: hardware_profile(),
        scheduling_policy: scheduling_policy(),
        billing_policy: Some(BillingPolicy::new(BillingMetric::PerGpuHour, 1.5)),
    }
}

fn cluster_props() -> NewHpcClusterProps {
    NewHpcClusterProps {
        enabled: true,
        name: "Vista".into(),
        description: Some("TACC GPU cluster".into()),
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        documentation_url: Some("https://docs.tacc.utexas.edu/hpc/vista".into()),
        data_center: DataCenter::Tacc,
        queues: vec![queue_props("gh")],
    }
}

#[test]
fn creates_cluster_and_embedded_queues_with_uuid_v7_ids() -> Result<(), Box<dyn std::error::Error>>
{
    let cluster = HpcCluster::new(cluster_props())?;

    assert_eq!(cluster.id().as_uuid().get_version_num(), 7);
    assert!(cluster.enabled());
    assert_eq!(cluster.queues().len(), 1);
    assert_eq!(cluster.queues()[0].id().as_uuid().get_version_num(), 7);
    assert_eq!(cluster.queues()[0].cluster_id(), cluster.id());
    assert!(cluster.queues()[0].enabled());

    Ok(())
}

#[test]
fn rejects_invalid_new_cluster_fields() {
    let mut props = cluster_props();
    props.name = String::new();

    assert!(matches!(
        HpcCluster::new(props),
        Err(HpcClusterError::EmptyName)
    ));

    let mut props = cluster_props();
    props.host = String::new();

    assert!(matches!(
        HpcCluster::new(props),
        Err(HpcClusterError::EmptyHost)
    ));

    let mut props = cluster_props();
    props.port = 0;

    assert!(matches!(
        HpcCluster::new(props),
        Err(HpcClusterError::InvalidPort)
    ));
}

#[test]
fn rejects_duplicate_queue_names() -> Result<(), Box<dyn std::error::Error>> {
    let id = HpcClusterId::new();
    let first = BatchSchedulerQueue::new(id, queue_props("normal"))?;
    let second = BatchSchedulerQueue::new(id, queue_props("normal"))?;

    let result = HpcCluster::reconstitute(ReconstituteHpcClusterProps {
        id,
        enabled: true,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: vec![first, second],
    });

    assert!(matches!(
        result,
        Err(HpcClusterError::DataIntegrityError(_))
    ));

    Ok(())
}

#[test]
fn rejects_duplicate_queue_ids() -> Result<(), Box<dyn std::error::Error>> {
    let cluster_id = HpcClusterId::new();
    let queue_id = BatchSchedulerQueueId::new();
    let first = BatchSchedulerQueue::reconstitute(ReconstituteBatchSchedulerQueueProps {
        id: queue_id,
        cluster_id,
        enabled: true,
        name: "normal".into(),
        scheduler_type: SchedulerType::Slurm,
        hardware_profile: hardware_profile(),
        scheduling_policy: scheduling_policy(),
        billing_policy: None,
    })?;
    let second = BatchSchedulerQueue::reconstitute(ReconstituteBatchSchedulerQueueProps {
        id: queue_id,
        cluster_id,
        enabled: true,
        name: "development".into(),
        scheduler_type: SchedulerType::Slurm,
        hardware_profile: hardware_profile(),
        scheduling_policy: scheduling_policy(),
        billing_policy: None,
    })?;

    let result = HpcCluster::reconstitute(ReconstituteHpcClusterProps {
        id: cluster_id,
        enabled: true,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: vec![first, second],
    });

    assert!(matches!(
        result,
        Err(HpcClusterError::DataIntegrityError(_))
    ));

    Ok(())
}

#[test]
fn rejects_queue_belonging_to_another_cluster() -> Result<(), Box<dyn std::error::Error>> {
    let id = HpcClusterId::new();
    let queue = BatchSchedulerQueue::new(HpcClusterId::new(), queue_props("normal"))?;

    let result = HpcCluster::reconstitute(ReconstituteHpcClusterProps {
        id,
        enabled: true,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: vec![queue],
    });

    assert!(matches!(
        result,
        Err(HpcClusterError::DataIntegrityError(_))
    ));

    Ok(())
}

#[test]
fn resolves_gpu_memory_and_returns_null_for_cpu_queues() -> Result<(), Box<dyn std::error::Error>> {
    let cluster_id = HpcClusterId::new();
    let gpu_queue = BatchSchedulerQueue::new(cluster_id, queue_props("gpu"))?;
    let mut cpu_props = queue_props("cpu");

    cpu_props.hardware_profile = HardwareProfile::new(
        100,
        128,
        "x86_64".into(),
        "EPYC".into(),
        "AMD".into(),
        512,
        None,
    );

    let cpu_queue = BatchSchedulerQueue::new(cluster_id, cpu_props)?;
    let field_path = || {
        Some(FieldPath::new(vec![
            "hardware_profile".into(),
            "gpu".into(),
            "gpu_memory_gb".into(),
        ]))
    };

    assert_eq!(
        Value::from(gpu_queue.resolve_value(field_path())?),
        Value::from(80_u64)
    );
    assert_eq!(
        Value::from(cpu_queue.resolve_value(field_path())?),
        Value::Null
    );

    Ok(())
}

#[test]
fn evaluates_real_model_and_queue_compatibility() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../deploy/k8s/site-configs/base/evaluations.json");
    let evaluator = Evaluator::load(config_path)?;
    let metadata = DerivedMetadata::new(
        Some("model".into()),
        Some("author".into()),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
        40_000_000_000,
        false,
        false,
        None,
        None,
    )?;
    let locator = HuggingFaceRepoLocator::new("owner/repo".into(), "sha".into())?;
    let model = ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::HuggingFace(locator),
        ModelMetadata::new(metadata, Map::new()),
    )?;
    let queue = BatchSchedulerQueue::new(HpcClusterId::new(), queue_props("gpu"))?;
    let mut arguments: Arguments = HashMap::new();

    arguments.insert("model".into(), Rc::new(model));
    arguments.insert("queue".into(), Rc::new(queue));

    assert!(evaluator.evaluate("Compatible Deployment Target", &arguments)?);

    Ok(())
}
