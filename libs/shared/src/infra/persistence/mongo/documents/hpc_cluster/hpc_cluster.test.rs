use super::*;

fn domain_cluster() -> Result<domain::HpcCluster, domain::HpcClusterError> {
    domain::HpcCluster::new(domain::NewHpcClusterProps {
        enabled: false,
        name: "Vista".into(),
        description: Some("GPU cluster".into()),
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        documentation_url: None,
        data_center: domain::DataCenter::Tacc,
        queues: vec![domain::NewBatchSchedulerQueueProps {
            enabled: false,
            name: "gh".into(),
            scheduler_type: domain::SchedulerType::Slurm,
            hardware_profile: domain::HardwareProfile::new(
                100,
                128,
                "x86_64".into(),
                "EPYC".into(),
                "AMD".into(),
                512,
                Some(domain::Accelerator::new(
                    4,
                    domain::AcceleratorProfile::Gpu(domain::GpuProfile::new(
                        "H100".into(),
                        "NVIDIA".into(),
                        80,
                        false,
                    )),
                )),
            ),
            scheduling_policy: domain::SchedulingPolicy::new(1, 16, 10, 32, 48, 100),
            billing_policy: Some(domain::BillingPolicy::new(
                domain::BillingMetric::PerGpuHour,
                1.5,
            )),
        }],
    })
}

#[test]
fn document_round_trip_preserves_cluster_definition() -> Result<(), Box<dyn std::error::Error>> {
    let original = domain_cluster()?;
    let document = HpcCluster::from(&original);

    let gpu = document.queues[0]
        .hardware_profile
        .gpu
        .as_ref()
        .ok_or_else(|| std::io::Error::other("GPU definition should be persisted"))?;

    assert!(matches!(
        document.queues[0]
            .hardware_profile
            .accelerator_type
            .as_ref(),
        Some(AcceleratorType::Gpu)
    ));
    assert_eq!(gpu.count_per_node, 4);
    assert_eq!(gpu.gpu_model, "H100");

    let restored = domain::HpcCluster::try_from(document)?;

    assert_eq!(restored.id(), original.id());
    assert!(!restored.enabled());
    assert_eq!(restored.name(), original.name());
    assert_eq!(restored.queues().len(), 1);
    assert_eq!(restored.queues()[0].name(), "gh");
    assert!(!restored.queues()[0].enabled());
    assert!(matches!(
        restored.queues()[0].hardware_profile().accelerator(),
        Some(accelerator)
            if matches!(accelerator.profile(), domain::AcceleratorProfile::Gpu(_))
    ));

    Ok(())
}

#[test]
fn missing_enabled_fields_default_to_enabled() -> Result<(), Box<dyn std::error::Error>> {
    let original = domain_cluster()?;
    let document = HpcCluster::from(&original);
    let mut bson = mongodb::bson::to_document(&document)?;

    bson.remove("enabled");

    let queues = bson
        .get_array_mut("queues")?
        .first_mut()
        .and_then(mongodb::bson::Bson::as_document_mut)
        .ok_or_else(|| std::io::Error::other("queue document should exist"))?;

    queues.remove("enabled");

    let document = mongodb::bson::from_document::<HpcCluster>(bson)?;
    let restored = domain::HpcCluster::try_from(document)?;

    assert!(restored.enabled());
    assert!(restored.queues()[0].enabled());

    Ok(())
}

#[test]
fn discriminator_and_accelerator_definition_must_agree() -> Result<(), Box<dyn std::error::Error>> {
    let original = domain_cluster()?;
    let mut missing_definition = HpcCluster::from(&original);

    missing_definition.queues[0].hardware_profile.gpu = None;

    let result = domain::HpcCluster::try_from(missing_definition);

    assert!(matches!(
        result,
        Err(domain::HpcClusterError::DataIntegrityError(_))
    ));

    let mut missing_discriminator = HpcCluster::from(&original);

    missing_discriminator.queues[0]
        .hardware_profile
        .accelerator_type = None;

    let result = domain::HpcCluster::try_from(missing_discriminator);

    assert!(matches!(
        result,
        Err(domain::HpcClusterError::DataIntegrityError(_))
    ));

    Ok(())
}

#[test]
fn invalid_persisted_queue_reference_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let original = domain_cluster()?;
    let mut document = HpcCluster::from(&original);

    document.queues[0].cluster_id = Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes());

    let result = domain::HpcCluster::try_from(document);

    assert!(matches!(
        result,
        Err(domain::HpcClusterError::DataIntegrityError(_))
    ));

    Ok(())
}
