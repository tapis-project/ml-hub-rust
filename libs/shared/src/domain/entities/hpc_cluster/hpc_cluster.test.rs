use super::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    rc::Rc,
};

use evaluations::{
    Arguments, Evaluator, FieldPath, FieldValue, ResolveValue, ValueResolutionError,
};
use serde_json::{Map, Value};

use crate::domain::entities::model::external_model::{
    DerivedMetadata, ExternalModel, HuggingFaceRepoLocator, ModelLocator, ModelMetadata,
    ModelProvider,
};
use crate::shared_kernel::enums::Task;

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
        container_runtimes: vec![ContainerRuntime::Apptainer, ContainerRuntime::SingularityCe],
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
    assert_eq!(
        cluster.container_runtimes(),
        [ContainerRuntime::Apptainer, ContainerRuntime::SingularityCe]
    );
    assert_eq!(cluster.queues().len(), 1);
    assert_eq!(cluster.queues()[0].id().as_uuid().get_version_num(), 7);
    assert_eq!(cluster.queues()[0].cluster_id(), cluster.id());
    assert!(cluster.queues()[0].enabled());
    assert_eq!(
        cluster.supported_batch_schedulers(),
        HashSet::from([SchedulerType::Slurm])
    );

    Ok(())
}

#[test]
fn cluster_and_queue_expose_deployment_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let cluster = HpcCluster::new(cluster_props())?;

    let cluster_parameters = cluster.provide_parameters();

    let queue_parameters = cluster.queues()[0].provide_parameters();

    assert!(cluster_parameters.is_empty());
    assert_eq!(queue_parameters.len(), 2);
    assert_eq!(queue_parameters[0].name, "project_allocation");
    assert!(queue_parameters[0].required);
    assert_eq!(queue_parameters[1].name, "reservation");
    assert!(!queue_parameters[1].required);

    Ok(())
}

#[test]
fn allows_an_empty_container_runtime_collection() -> Result<(), Box<dyn std::error::Error>> {
    let mut props = cluster_props();

    props.container_runtimes = Vec::new();

    let cluster = HpcCluster::new(props)?;

    assert!(cluster.container_runtimes().is_empty());

    Ok(())
}

#[test]
fn rejects_duplicate_container_runtimes() {
    let mut props = cluster_props();

    props.container_runtimes = vec![ContainerRuntime::Apptainer, ContainerRuntime::Apptainer];

    let result = HpcCluster::new(props);

    assert!(matches!(
        result,
        Err(HpcClusterError::DuplicateContainerRuntime(
            ContainerRuntime::Apptainer
        ))
    ));
}

#[test]
fn reports_persisted_duplicate_container_runtimes_as_data_integrity_error() {
    let result = HpcCluster::reconstitute(ReconstituteHpcClusterProps {
        id: HpcClusterId::new(),
        enabled: true,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        container_runtimes: vec![ContainerRuntime::Apptainer, ContainerRuntime::Apptainer],
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: Vec::new(),
    });

    assert!(matches!(
        result,
        Err(HpcClusterError::DataIntegrityError(_))
    ));
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
        container_runtimes: Vec::new(),
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
        container_runtimes: Vec::new(),
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
        container_runtimes: Vec::new(),
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
fn resolves_gpu_fields_and_returns_undefined_for_cpu_queues(
) -> Result<(), Box<dyn std::error::Error>> {
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

    let gpu_path = || {
        Some(FieldPath::new(vec![
            "hardware_profile".into(),
            "gpu".into(),
        ]))
    };

    let gpu_memory_path = || {
        Some(FieldPath::new(vec![
            "hardware_profile".into(),
            "gpu".into(),
            "gpu_memory_gb".into(),
        ]))
    };

    assert_eq!(
        Value::from(gpu_queue.resolve_value(gpu_memory_path())?),
        Value::from(80_u64)
    );
    assert_eq!(
        Value::from(gpu_queue.resolve_value(gpu_path())?),
        serde_json::json!({
            "count_per_node": 4,
            "gpu_model": "H100",
            "gpu_vendor": "NVIDIA",
            "gpu_memory_gb": 80,
            "unified_memory": false,
        })
    );

    let missing_gpu = cpu_queue.resolve_value(gpu_path())?;

    let missing_gpu_memory = cpu_queue.resolve_value(gpu_memory_path())?;

    assert!(matches!(&missing_gpu, FieldValue::Undefined));
    assert!(matches!(&missing_gpu_memory, FieldValue::Undefined));
    assert_eq!(Value::from(missing_gpu), Value::Null);
    assert_eq!(Value::from(missing_gpu_memory), Value::Null);

    let invalid = cpu_queue.resolve_value(Some(FieldPath::new(vec!["unknown".into()])));

    assert!(matches!(
        invalid,
        Err(ValueResolutionError::InvalidFieldPath(_))
    ));

    Ok(())
}

#[test]
fn evaluates_has_gpus_for_gpu_and_cpu_queues() -> Result<(), Box<dyn std::error::Error>> {
    let config = serde_json::json!({
        "evaluations": [{
            "name": "Queue Compatibility",
            "evaluation_strategy": "DNF",
            "parameters": ["queue"],
            "expressions": [["Has Gpus"]]
        }],
        "statements": [{
            "name": "Has Gpus",
            "evaluation_strategy": "DNF",
            "parameters": ["queue"],
            "conditions": [[{
                "operator": "Neq",
                "operands": [
                    {
                        "type": "parameter",
                        "parameter": "queue",
                        "accessor": {
                            "field_path": ["hardware_profile", "gpu"]
                        }
                    },
                    {
                        "type": "literal",
                        "value": null
                    }
                ]
            }]]
        }]
    });

    let temp_dir = std::env::temp_dir();

    let config_name = format!("hpc-cluster-has-gpus-{}.json", Uuid::now_v7());

    let config_path = temp_dir.join(config_name);

    fs::write(&config_path, serde_json::to_vec(&config)?)?;

    let evaluator = Evaluator::load(&config_path)?;

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

    let mut arguments: Arguments = HashMap::new();

    arguments.insert("queue".into(), Rc::new(gpu_queue));

    assert!(evaluator.evaluate("Queue Compatibility", &arguments)?);

    arguments.insert("queue".into(), Rc::new(cpu_queue));

    assert!(!evaluator.evaluate("Queue Compatibility", &arguments)?);

    fs::remove_file(config_path)?;

    Ok(())
}

#[test]
fn evaluates_real_model_and_queue_compatibility() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../deploy/k8s/site-configs/base/evaluations.json");

    let evaluator = Evaluator::load(config_path)?;

    let metadata = DerivedMetadata::new(
        Some("model".into()),
        Some("Qwen".into()),
        vec!["transformers".into()],
        Vec::new(),
        vec![Task::TextGeneration],
        None,
        40_000_000_000,
        false,
        false,
        None,
        None,
    )?;

    let locator = HuggingFaceRepoLocator::new("owner/repo".into(), "sha".into())?;

    let mut canonical = Map::new();

    canonical.insert("id".into(), Value::String("Qwen/model".into()));
    canonical.insert("tags".into(), Value::Array(Vec::new()));
    canonical.insert("author".into(), Value::String("Qwen".into()));
    canonical.insert("config".into(), serde_json::json!({}));
    canonical.insert("gguf".into(), Value::Null);

    let model = ExternalModel::ingest(
        ModelProvider::HuggingFace,
        ModelLocator::HuggingFace(locator),
        ModelMetadata::new(metadata, canonical),
    )?;

    let queue = BatchSchedulerQueue::new(HpcClusterId::new(), queue_props("gpu"))?;

    let mut arguments: Arguments = HashMap::new();

    arguments.insert("model".into(), Rc::new(model));
    arguments.insert("queue".into(), Rc::new(queue));

    assert!(evaluator.evaluate("FlexServ Compatible Deployment Option", &arguments)?);

    Ok(())
}
