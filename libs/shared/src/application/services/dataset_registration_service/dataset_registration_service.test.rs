use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use super::*;
use crate::{
    application::{
        inputs::dataset::{
            DatasetProviderInput, HuggingFaceRepoLocatorInput, ListDatasetsInput,
            TapisSystemLocatorInput, VisibilityInput,
        },
        outputs::dataset::{DatasetListOutput, DatasetQueryOutput},
        ports::errors::InfrastructureError,
    },
    domain::entities::dataset::DatasetProvider,
};

#[derive(Default)]
struct TestRepository {
    saved: Mutex<Option<Dataset>>,
    save_attempts: Mutex<usize>,
    fail_save: bool,
}

#[async_trait]
impl DatasetRepository for TestRepository {
    async fn save(&self, dataset: &Dataset) -> Result<(), DatasetRepositoryError> {
        *self
            .save_attempts
            .lock()
            .unwrap_or_else(|error| error.into_inner()) += 1;

        if self.fail_save {
            return Err(InfrastructureError::new_internal().into());
        }

        *self.saved.lock().unwrap_or_else(|error| error.into_inner()) = Some(dataset.clone());

        Ok(())
    }

    async fn find_by_id(
        &self,
        _tenant_id: &str,
        _id: Uuid,
    ) -> Result<Option<DatasetQueryOutput>, DatasetRepositoryError> {
        Ok(None)
    }

    async fn find_by_huggingface_repo_locator(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _repo_id: &str,
        _sha: &str,
    ) -> Result<Option<Dataset>, DatasetRepositoryError> {
        Ok(None)
    }

    async fn list_by_owner(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetRepositoryError> {
        Ok(empty_dataset_list())
    }

    async fn list_by_tenant(
        &self,
        _tenant_id: &str,
        _input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetRepositoryError> {
        Ok(empty_dataset_list())
    }

    async fn list_shared_with_user(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetRepositoryError> {
        Ok(empty_dataset_list())
    }
}

#[tokio::test]
async fn derives_identity_and_saves_huggingface_dataset(
) -> Result<(), DatasetRegistrationServiceError> {
    let repository = Arc::new(TestRepository::default());
    let service = DatasetRegistrationService::new(repository.clone());
    let context = RequestContext::system(None);

    let dataset = service
        .register_dataset(&context, huggingface_input())
        .await?;

    assert_eq!(dataset.tenant_id(), context.actor_tenant_id());
    assert_eq!(dataset.owner(), context.actor_principal_id());
    assert_eq!(
        repository
            .saved
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .as_ref()
            .map(Dataset::id),
        Some(dataset.id())
    );

    Ok(())
}

#[tokio::test]
async fn accepts_tapis_locator_outside_request_site_and_tenant(
) -> Result<(), DatasetRegistrationServiceError> {
    let repository = Arc::new(TestRepository::default());
    let service = DatasetRegistrationService::new(repository.clone());
    let context = RequestContext::system(None);

    let dataset = service
        .register_dataset(&context, tapis_input("other-site", "other-tenant"))
        .await?;

    assert!(matches!(dataset.provider(), DatasetProvider::Tapis(_)));
    assert!(repository
        .saved
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .is_some());

    Ok(())
}

#[tokio::test]
async fn invalid_domain_input_is_rejected_before_persistence() {
    let repository = Arc::new(TestRepository::default());
    let service = DatasetRegistrationService::new(repository.clone());
    let context = RequestContext::system(None);
    let mut input = huggingface_input();
    input.provider = DatasetProviderInput::HuggingFace(HuggingFaceRepoLocatorInput {
        id: String::new(),
        sha: "abc".into(),
    });

    let result = service.register_dataset(&context, input).await;

    assert!(matches!(
        result,
        Err(DatasetRegistrationServiceError::Locator(_))
    ));
    assert_eq!(
        *repository
            .save_attempts
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        0
    );
}

#[tokio::test]
async fn repository_failures_are_retried_and_returned() {
    let repository = Arc::new(TestRepository {
        fail_save: true,
        ..Default::default()
    });
    let service = DatasetRegistrationService::new(repository.clone());
    let context = RequestContext::system(None);

    let result = service
        .register_dataset(&context, huggingface_input())
        .await;

    assert!(matches!(
        result,
        Err(DatasetRegistrationServiceError::Repository(_))
    ));
    assert_eq!(
        *repository
            .save_attempts
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        4
    );
    assert!(repository
        .saved
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .is_none());
}

fn huggingface_input() -> RegisterDatasetInput {
    RegisterDatasetInput {
        name: "dataset".into(),
        description: None,
        tags: Vec::new(),
        provider: DatasetProviderInput::HuggingFace(HuggingFaceRepoLocatorInput {
            id: "owner/repo".into(),
            sha: "abc".into(),
        }),
        items: Vec::new(),
        size: 0,
        visibility: VisibilityInput::Private,
    }
}

fn tapis_input(site_id: &str, tenant_id: &str) -> RegisterDatasetInput {
    RegisterDatasetInput {
        name: "dataset".into(),
        description: None,
        tags: Vec::new(),
        provider: DatasetProviderInput::Tapis(TapisSystemLocatorInput {
            site_id: site_id.into(),
            tenant_id: tenant_id.into(),
            system_id: "system".into(),
            path: "path".into(),
        }),
        items: Vec::new(),
        size: 0,
        visibility: VisibilityInput::Private,
    }
}

fn empty_dataset_list() -> DatasetListOutput {
    DatasetListOutput {
        datasets: Vec::new(),
        cursor: None,
        count: None,
    }
}
