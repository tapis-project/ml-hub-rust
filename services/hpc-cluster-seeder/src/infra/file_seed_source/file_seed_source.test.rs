use std::{fs::write, path::PathBuf};

use super::*;

fn temporary_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "hpc-cluster-seeder-{}-{name}.json",
        uuid::Uuid::now_v7()
    ))
}

fn seed_json(accelerator_type: &str, gpu: &str) -> String {
    format!(
        r#"[
            {{
                "enabled": true,
                "name": "Vista",
                "description": null,
                "host": "vista.tacc.utexas.edu",
                "port": 22,
                "documentation_url": null,
                "data_center": "Tacc",
                "queues": [
                    {{
                        "enabled": false,
                        "name": "gh",
                        "scheduler_type": "Slurm",
                        "hardware_profile": {{
                            "total_nodes": 600,
                            "cpu_cores_per_node": 72,
                            "cpu_architecture": "aarch64",
                            "cpu_model": "Grace",
                            "cpu_vendor": "NVIDIA",
                            "memory_gb": 116,
                            "accelerator_type": {accelerator_type},
                            "gpu": {gpu}
                        }},
                        "scheduling_policy": {{
                            "min_nodes_per_job": 1,
                            "max_nodes_per_job": 8,
                            "max_jobs_per_user": 1,
                            "max_nodes_per_user": 8,
                            "max_wall_time_per_job_hr": 2,
                            "max_job_submission": 3
                        }},
                        "billing_policy": null
                    }}
                ]
            }}
        ]"#
    )
}

#[test]
fn loads_seed_configuration_without_identity_fields() -> Result<(), Box<dyn std::error::Error>> {
    let path = temporary_path("valid");
    let gpu = r#"{
        "count_per_node": 1,
        "gpu_model": "H200",
        "gpu_vendor": "NVIDIA",
        "gpu_memory_gb": 96,
        "unified_memory": true
    }"#;

    write(&path, seed_json(r#""Gpu""#, gpu))?;

    let source = FileHpcClusterSeedSource::new(&path);
    let props = source.load()?;

    assert_eq!(props.len(), 1);
    assert!(props[0].enabled);
    assert_eq!(props[0].queues.len(), 1);
    assert!(!props[0].queues[0].enabled);

    std::fs::remove_file(path)?;

    Ok(())
}

#[test]
fn rejects_mismatched_accelerator_discriminator() -> Result<(), Box<dyn std::error::Error>> {
    let path = temporary_path("invalid-accelerator");

    write(&path, seed_json(r#""Gpu""#, "null"))?;

    let source = FileHpcClusterSeedSource::new(&path);
    let result = source.load();

    assert!(matches!(
        result,
        Err(HpcClusterSeedError::InvalidConfiguration(_))
    ));

    std::fs::remove_file(path)?;

    Ok(())
}

#[test]
fn rejects_domain_identity_fields_in_seed_configuration() -> Result<(), Box<dyn std::error::Error>>
{
    let path = temporary_path("configured-id");
    let contents = seed_json("null", "null").replacen(
        r#""enabled": true,"#,
        r#""id": "01994cab-b66c-7ed2-a396-d2c6dbef11a9", "enabled": true,"#,
        1,
    );

    write(&path, contents)?;

    let source = FileHpcClusterSeedSource::new(&path);
    let result = source.load();

    assert!(matches!(
        result,
        Err(HpcClusterSeedError::InvalidConfiguration(_))
    ));

    std::fs::remove_file(path)?;

    Ok(())
}

#[test]
fn deployment_seed_contains_expected_clusters_and_queues() -> Result<(), Box<dyn std::error::Error>>
{
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../deploy/k8s/seeders/hpc-cluster-seeder/base/tacc-hpc-clusters.json");
    let source = FileHpcClusterSeedSource::new(path);

    let hpc_clusters = source
        .load()?
        .into_iter()
        .map(domain::HpcCluster::new)
        .collect::<Result<Vec<_>, _>>()?;
    let queue_count = hpc_clusters
        .iter()
        .map(|hpc_cluster| hpc_cluster.queues().len())
        .sum::<usize>();

    assert_eq!(hpc_clusters.len(), 4);
    assert_eq!(queue_count, 26);
    assert!(hpc_clusters.iter().all(|hpc_cluster| {
        hpc_cluster.enabled()
            && hpc_cluster.id().as_uuid().get_version_num() == 7
            && hpc_cluster.queues().iter().all(|queue| {
                queue.enabled()
                    && queue.id().as_uuid().get_version_num() == 7
                    && queue.cluster_id() == hpc_cluster.id()
            })
    }));

    Ok(())
}
