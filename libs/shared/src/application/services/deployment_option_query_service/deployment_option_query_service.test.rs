use async_trait::async_trait;
use nonempty::NonEmpty;

use super::*;
use crate::{
    application::{
        inputs::{
            deployment_option::ListDeploymentOptionsInput,
            discover_models::SearchExternalModelsInput, hpc_cluster::ListHpcClustersInput,
        },
        outputs::hpc_cluster::HpcClusterListOutput,
        ports::{
            deployment_option::{DeploymentOptionPage, DeploymentOptionRepositoryError},
            hpc_cluster::HpcClusterRepositoryError,
            model::{ExternalModelPage, ExternalModelRepositoryError},
        },
    },
    domain::entities::{
        deployment_option::{
            DeploymentOption, DeploymentTarget, HpcClusterQueueReference, NewDeploymentOptionProps,
            ServingRuntime,
        },
        hpc_cluster::{
            BatchSchedulerQueueId, DataCenter, HardwareProfile, NewBatchSchedulerQueueProps,
            NewHpcClusterProps, SchedulerType, SchedulingPolicy,
        },
        model::{
            external_model::{ExternalModel, ModelLocator, ModelProvider},
            fixtures::full_external_model,
        },
    },
    shared_kernel::enums::DeploymentModality,
};

struct TestDeploymentOptionRepository {
    deployment_options: Vec<DeploymentOption>,
}

#[async_trait]
impl DeploymentOptionRepository for TestDeploymentOptionRepository {
    async fn find_by_id(
        &self,
        id: &DeploymentOptionId,
    ) -> Result<Option<DeploymentOption>, DeploymentOptionRepositoryError> {
        Ok(self
            .deployment_options
            .iter()
            .find(|option| option.id() == id)
            .cloned())
    }

    async fn find_by_external_model_id(
        &self,
        _external_model_id: &ExternalModelId,
    ) -> Result<Vec<DeploymentOption>, DeploymentOptionRepositoryError> {
        Ok(self.deployment_options.clone())
    }

    async fn list_by_external_model_id(
        &self,
        _external_model_id: &ExternalModelId,
        _input: &ListDeploymentOptionsInput,
    ) -> Result<DeploymentOptionPage, DeploymentOptionRepositoryError> {
        Ok(DeploymentOptionPage {
            deployment_options: self.deployment_options.clone(),
            count: Some(self.deployment_options.len() as u64),
            cursor: Some("next".into()),
        })
    }

    async fn replace_for_external_model(
        &self,
        _external_model_id: &ExternalModelId,
        _deployment_options: &[DeploymentOption],
    ) -> Result<(), DeploymentOptionRepositoryError> {
        Ok(())
    }
}

struct TestExternalModelRepository {
    external_model: Option<ExternalModel>,
}

#[async_trait]
impl ExternalModelRepository for TestExternalModelRepository {
    async fn save(&self, _model: &ExternalModel) -> Result<(), ExternalModelRepositoryError> {
        Ok(())
    }

    async fn update(&self, _model: &ExternalModel) -> Result<(), ExternalModelRepositoryError> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        _id: &ExternalModelId,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(self.external_model.clone())
    }

    async fn find_by_ids(
        &self,
        _ids: &[ExternalModelId],
    ) -> Result<Vec<ExternalModel>, ExternalModelRepositoryError> {
        Ok(Vec::new())
    }

    async fn find_by_provider_and_locator(
        &self,
        _provider: &ModelProvider,
        _locator: &ModelLocator,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(None)
    }

    async fn search(
        &self,
        _input: &SearchExternalModelsInput,
    ) -> Result<ExternalModelPage, ExternalModelRepositoryError> {
        Ok(ExternalModelPage {
            external_models: Vec::new(),
            count: None,
            cursor: None,
        })
    }
}

struct TestHpcClusterRepository {
    hpc_clusters: Vec<HpcCluster>,
}

#[async_trait]
impl HpcClusterRepository for TestHpcClusterRepository {
    async fn list_all(&self) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError> {
        Ok(self.hpc_clusters.clone())
    }

    async fn find_by_id(
        &self,
        id: &HpcClusterId,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(self
            .hpc_clusters
            .iter()
            .find(|cluster| cluster.id() == id)
            .cloned())
    }

    async fn find_by_id_and_data_center(
        &self,
        _id: &HpcClusterId,
        _data_center: &DataCenter,
    ) -> Result<Option<HpcCluster>, HpcClusterRepositoryError> {
        Ok(None)
    }

    async fn find_by_ids(
        &self,
        ids: &[HpcClusterId],
    ) -> Result<Vec<HpcCluster>, HpcClusterRepositoryError> {
        Ok(self
            .hpc_clusters
            .iter()
            .filter(|cluster| ids.contains(cluster.id()))
            .cloned()
            .collect())
    }

    async fn list(
        &self,
        _input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError> {
        Ok(HpcClusterListOutput {
            hpc_clusters: Vec::new(),
            cursor: None,
            count: None,
        })
    }
}

fn hpc_cluster(
    enabled: bool,
    queue_enabled: bool,
) -> Result<HpcCluster, Box<dyn std::error::Error>> {
    Ok(HpcCluster::new(NewHpcClusterProps {
        enabled,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        container_runtimes: Vec::new(),
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: vec![NewBatchSchedulerQueueProps {
            enabled: queue_enabled,
            name: "gh".into(),
            scheduler_type: SchedulerType::Slurm,
            hardware_profile: HardwareProfile::new(
                1,
                1,
                "x86_64".into(),
                "cpu".into(),
                "vendor".into(),
                1,
                None,
            ),
            scheduling_policy: SchedulingPolicy::new(1, 1, 1, 1, 1, 1),
            billing_policy: None,
        }],
    })?)
}

fn deployment_option(
    external_model_id: ExternalModelId,
    hpc_cluster: &HpcCluster,
) -> Result<DeploymentOption, Box<dyn std::error::Error>> {
    Ok(DeploymentOption::new(NewDeploymentOptionProps {
        external_model_id,
        supported_deployment_modalities: NonEmpty::new(DeploymentModality::Batch),
        deployment_target: DeploymentTarget::HpcClusterQueue(HpcClusterQueueReference::new(
            *hpc_cluster.id(),
            *hpc_cluster.queues()[0].id(),
        )),
        serving_runtime: ServingRuntime::FlexServ,
    })?)
}

fn service(
    external_model: Option<ExternalModel>,
    deployment_options: Vec<DeploymentOption>,
    hpc_clusters: Vec<HpcCluster>,
) -> DeploymentOptionQueryService {
    DeploymentOptionQueryService::new(
        Arc::new(TestDeploymentOptionRepository { deployment_options }),
        Arc::new(TestExternalModelRepository { external_model }),
        Arc::new(TestHpcClusterRepository { hpc_clusters }),
    )
}

#[tokio::test]
async fn reports_missing_external_model() {
    let service = service(None, Vec::new(), Vec::new());

    let input = ListDeploymentOptionsInput::new(None, None, None);

    let result = service
        .list_external_model_deployment_options(
            &RequestContext::system(None),
            &ExternalModelId::new(),
            &input,
        )
        .await;

    assert!(matches!(
        result,
        Err(DeploymentOptionQueryServiceError::ExternalModelNotFound)
    ));
}

#[tokio::test]
async fn hydrates_display_ready_target_and_pagination() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let hpc_cluster = hpc_cluster(true, true)?;

    let option = deployment_option(*external_model.id(), &hpc_cluster)?;

    let service = service(Some(external_model), vec![option], vec![hpc_cluster]);

    let input = ListDeploymentOptionsInput::new(None, None, Some(true));

    let output = service
        .list_external_model_deployment_options(
            &RequestContext::system(None),
            &ExternalModelId::new(),
            &input,
        )
        .await?;

    assert_eq!(output.deployment_options.len(), 1);
    assert_eq!(output.count, Some(1));
    assert_eq!(output.cursor.as_deref(), Some("next"));
    assert_eq!(
        output.deployment_options[0].target.hpc_cluster_name,
        "Vista"
    );
    assert_eq!(
        output.deployment_options[0]
            .target
            .batch_scheduler_queue_name,
        "gh"
    );
    assert!(output.deployment_options[0].available);

    Ok(())
}

#[tokio::test]
async fn returns_disabled_target_as_unavailable() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let hpc_cluster = hpc_cluster(true, false)?;

    let option = deployment_option(*external_model.id(), &hpc_cluster)?;

    let service = service(Some(external_model), vec![option], vec![hpc_cluster]);

    let input = ListDeploymentOptionsInput::new(None, None, None);

    let output = service
        .list_external_model_deployment_options(
            &RequestContext::system(None),
            &ExternalModelId::new(),
            &input,
        )
        .await?;

    assert!(!output.deployment_options[0].available);
    assert!(
        !output.deployment_options[0]
            .target
            .batch_scheduler_queue_enabled
    );

    Ok(())
}

#[tokio::test]
async fn returns_option_on_disabled_cluster_as_unavailable(
) -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let hpc_cluster = hpc_cluster(false, true)?;

    let option = deployment_option(*external_model.id(), &hpc_cluster)?;

    let service = service(Some(external_model), vec![option], vec![hpc_cluster]);

    let input = ListDeploymentOptionsInput::new(None, None, None);

    let output = service
        .list_external_model_deployment_options(
            &RequestContext::system(None),
            &ExternalModelId::new(),
            &input,
        )
        .await?;

    assert!(!output.deployment_options[0].available);
    assert!(!output.deployment_options[0].target.hpc_cluster_enabled);
    assert!(
        output.deployment_options[0]
            .target
            .batch_scheduler_queue_enabled
    );

    Ok(())
}

#[tokio::test]
async fn reports_dangling_hpc_cluster_reference() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let hpc_cluster = hpc_cluster(true, true)?;

    let option = deployment_option(*external_model.id(), &hpc_cluster)?;

    let service = service(Some(external_model), vec![option], Vec::new());

    let input = ListDeploymentOptionsInput::new(None, None, None);

    let result = service
        .list_external_model_deployment_options(
            &RequestContext::system(None),
            &ExternalModelId::new(),
            &input,
        )
        .await;

    assert!(matches!(
        result,
        Err(DeploymentOptionQueryServiceError::DataIntegrity(_))
    ));

    Ok(())
}

#[tokio::test]
async fn reports_dangling_queue_reference() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let hpc_cluster = hpc_cluster(true, true)?;

    let option = DeploymentOption::new(NewDeploymentOptionProps {
        external_model_id: *external_model.id(),
        supported_deployment_modalities: NonEmpty::new(DeploymentModality::Batch),
        deployment_target: DeploymentTarget::HpcClusterQueue(HpcClusterQueueReference::new(
            *hpc_cluster.id(),
            BatchSchedulerQueueId::new(),
        )),
        serving_runtime: ServingRuntime::FlexServ,
    })?;

    let service = service(Some(external_model), vec![option], vec![hpc_cluster]);

    let input = ListDeploymentOptionsInput::new(None, None, None);

    let result = service
        .list_external_model_deployment_options(
            &RequestContext::system(None),
            &ExternalModelId::new(),
            &input,
        )
        .await;

    assert!(matches!(
        result,
        Err(DeploymentOptionQueryServiceError::DataIntegrity(_))
    ));

    Ok(())
}
