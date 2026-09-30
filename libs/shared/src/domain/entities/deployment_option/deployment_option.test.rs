use nonempty::nonempty;
use uuid::Version;

use super::*;

fn props() -> NewDeploymentOptionProps {
    NewDeploymentOptionProps {
        external_model_id: ExternalModelId::new(),
        supported_deployment_modalities: nonempty![DeploymentModality::Batch],
        deployment_target: DeploymentTarget::HpcClusterQueue(HpcClusterQueueReference::new(
            HpcClusterId::new(),
            BatchSchedulerQueueId::new(),
        )),
        serving_runtime: ServingRuntime::FlexServ,
    }
}

#[test]
fn creates_uuid_v7_deployment_option() -> Result<(), DeploymentOptionError> {
    let option = DeploymentOption::new(props())?;

    assert_eq!(option.id().as_uuid().get_version(), Some(Version::SortRand));
    assert_eq!(option.serving_runtime(), &ServingRuntime::FlexServ);
    assert_eq!(
        option.supported_deployment_modalities().head,
        DeploymentModality::Batch
    );
    assert_eq!(option.created_at(), option.updated_at());

    Ok(())
}

#[test]
fn rejects_duplicate_modalities() {
    let mut props = props();
    props.supported_deployment_modalities =
        nonempty![DeploymentModality::Batch, DeploymentModality::Batch];

    let result = DeploymentOption::new(props);

    assert!(matches!(
        result,
        Err(DeploymentOptionError::DuplicateDeploymentModality(
            DeploymentModality::Batch
        ))
    ));
}

#[test]
fn persisted_duplicate_modalities_are_data_integrity_errors(
) -> Result<(), Box<dyn std::error::Error>> {
    let option = DeploymentOption::new(props())?;

    let result = DeploymentOption::reconstitute(ReconstituteDeploymentOptionProps {
        id: *option.id(),
        external_model_id: *option.external_model_id(),
        supported_deployment_modalities: nonempty![
            DeploymentModality::Batch,
            DeploymentModality::Batch
        ],
        deployment_target: option.deployment_target().clone(),
        serving_runtime: *option.serving_runtime(),
        created_at: option.created_at().clone(),
        updated_at: option.updated_at().clone(),
    });

    assert!(matches!(
        result,
        Err(DeploymentOptionError::DataIntegrityError(_))
    ));

    Ok(())
}

#[test]
fn replacing_supported_deployment_modalities_preserves_identity_and_creation_time(
) -> Result<(), DeploymentOptionError> {
    let mut option = DeploymentOption::new(props())?;
    let id = *option.id();
    let created_at = option.created_at().clone();

    option.replace_supported_deployment_modalities(nonempty![DeploymentModality::Batch])?;

    assert_eq!(option.id(), &id);
    assert_eq!(option.created_at(), &created_at);

    Ok(())
}

#[test]
fn persisted_invalid_timestamp_order_is_a_data_integrity_error(
) -> Result<(), Box<dyn std::error::Error>> {
    let option = DeploymentOption::new(props())?;

    let result = DeploymentOption::reconstitute(ReconstituteDeploymentOptionProps {
        id: *option.id(),
        external_model_id: *option.external_model_id(),
        supported_deployment_modalities: option.supported_deployment_modalities().clone(),
        deployment_target: option.deployment_target().clone(),
        serving_runtime: *option.serving_runtime(),
        created_at: TimeStamp::parse_string("2026-09-29T12:00:00Z")?,
        updated_at: TimeStamp::parse_string("2026-09-29T11:00:00Z")?,
    });

    assert!(matches!(
        result,
        Err(DeploymentOptionError::DataIntegrityError(_))
    ));

    Ok(())
}
