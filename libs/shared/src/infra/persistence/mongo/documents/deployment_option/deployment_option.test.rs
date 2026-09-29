use nonempty::nonempty;

use super::*;
use crate::infra::_common::mongo::Index;
use crate::infra::persistence::mongo::documents::deployment_option::indexes::{
    DeploymentOptionIdIndexUnique, DeploymentOptionSemanticIdentityIndexUnique,
};
use crate::{
    domain::entities::{
        deployment_option::{
            DeploymentOption as DomainDeploymentOption, DeploymentTarget,
            HpcClusterQueueReference as DomainHpcClusterQueueReference, NewDeploymentOptionProps,
        },
        hpc_cluster::{BatchSchedulerQueueId, HpcClusterId},
    },
    shared_kernel::enums::DeploymentModality as DomainDeploymentModality,
};

#[test]
fn round_trips_deployment_option() -> Result<(), Box<dyn std::error::Error>> {
    let domain = DomainDeploymentOption::new(NewDeploymentOptionProps {
        external_model_id: ExternalModelId::new(),
        supported_deployment_modalities: nonempty![DomainDeploymentModality::Batch],
        deployment_target: DeploymentTarget::HpcClusterQueue(DomainHpcClusterQueueReference::new(
            HpcClusterId::new(),
            BatchSchedulerQueueId::new(),
        )),
        serving_runtime: domain::ServingRuntime::FlexServ,
    })?;

    let document = DeploymentOption::from(&domain);
    let reconstituted = DomainDeploymentOption::try_from(document)?;

    assert_eq!(reconstituted.id(), domain.id());
    assert!(reconstituted.has_same_semantic_identity(&domain));

    Ok(())
}

#[test]
fn rejects_missing_discriminated_target() -> Result<(), Box<dyn std::error::Error>> {
    let domain = DomainDeploymentOption::new(NewDeploymentOptionProps {
        external_model_id: ExternalModelId::new(),
        supported_deployment_modalities: nonempty![DomainDeploymentModality::Batch],
        deployment_target: DeploymentTarget::HpcClusterQueue(DomainHpcClusterQueueReference::new(
            HpcClusterId::new(),
            BatchSchedulerQueueId::new(),
        )),
        serving_runtime: domain::ServingRuntime::FlexServ,
    })?;
    let mut document = DeploymentOption::from(&domain);
    document.hpc_cluster_queue = None;

    let result = DomainDeploymentOption::try_from(document);

    assert!(matches!(
        result,
        Err(domain::DeploymentOptionError::DataIntegrityError(_))
    ));

    Ok(())
}

#[test]
fn defines_identity_and_semantic_uniqueness_indexes() {
    let id_index = DeploymentOptionIdIndexUnique::index();
    let semantic_index = DeploymentOptionSemanticIdentityIndexUnique::index();

    assert_eq!(id_index.keys, mongodb::bson::doc! { "id": 1 });
    assert_eq!(
        id_index.options.and_then(|options| options.unique),
        Some(true)
    );
    assert_eq!(
        semantic_index.keys,
        mongodb::bson::doc! {
            "external_model_id": 1,
            "serving_runtime": 1,
            "deployment_target_type": 1,
            "hpc_cluster_queue.hpc_cluster_id": 1,
            "hpc_cluster_queue.batch_scheduler_queue_id": 1,
        }
    );
    assert_eq!(
        semantic_index.options.and_then(|options| options.unique),
        Some(true)
    );
}
