use nonempty::NonEmpty;

use super::DeploymentOption as DeploymentOptionResponse;
use crate::{
    application::outputs::deployment_option::{
        DeploymentOptionOutput, HpcClusterQueueTargetOutput,
    },
    domain::entities::{
        deployment_option::{
            DeploymentOption, DeploymentTarget, HpcClusterQueueReference, NewDeploymentOptionProps,
            ServingRuntime,
        },
        hpc_cluster::{BatchSchedulerQueueId, DataCenter, HpcClusterId},
    },
    shared_kernel::{enums::DeploymentModality, identifiers::ExternalModelId},
};

#[test]
fn maps_display_ready_hpc_target() -> Result<(), Box<dyn std::error::Error>> {
    let hpc_cluster_id = HpcClusterId::new();
    let batch_scheduler_queue_id = BatchSchedulerQueueId::new();
    let deployment_option = DeploymentOption::new(NewDeploymentOptionProps {
        external_model_id: ExternalModelId::new(),
        supported_deployment_modalities: NonEmpty::new(DeploymentModality::Batch),
        deployment_target: DeploymentTarget::HpcClusterQueue(HpcClusterQueueReference::new(
            hpc_cluster_id,
            batch_scheduler_queue_id,
        )),
        serving_runtime: ServingRuntime::FlexServ,
    })?;
    let output = DeploymentOptionOutput {
        deployment_option,
        target: HpcClusterQueueTargetOutput {
            hpc_cluster_id,
            hpc_cluster_name: "Vista".into(),
            data_center: DataCenter::Tacc,
            hpc_cluster_enabled: true,
            batch_scheduler_queue_id,
            batch_scheduler_queue_name: "gh".into(),
            batch_scheduler_queue_enabled: false,
        },
        available: false,
    };

    let response = serde_json::to_value(DeploymentOptionResponse::from(output))?;

    assert_eq!(
        response.pointer("/deployment_target_type"),
        Some(&serde_json::json!("HpcClusterQueue"))
    );
    assert_eq!(
        response.pointer("/hpc_cluster_queue/hpc_cluster_name"),
        Some(&serde_json::json!("Vista"))
    );
    assert_eq!(
        response.pointer("/hpc_cluster_queue/batch_scheduler_queue_name"),
        Some(&serde_json::json!("gh"))
    );
    assert_eq!(
        response.pointer("/available"),
        Some(&serde_json::json!(false))
    );

    Ok(())
}
