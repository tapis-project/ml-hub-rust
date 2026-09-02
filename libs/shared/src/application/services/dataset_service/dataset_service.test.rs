use super::*;
use crate::{
    application::{
        inputs::dataset::{
            DatasetProviderInput, HuggingFaceRepoLocatorInput, ListDatasetsInput, VisibilityInput,
        },
        outputs::dataset::{DatasetListOutput, DatasetQueryOutput},
        ports::dataset::DatasetRepository,
    },
    shared_kernel::context::RequestContext,
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct TestRepository {
    saved: Mutex<Option<Dataset>>,
    huggingface_lookup: Mutex<Option<(String, String, String, String)>>,
    list_requests: Mutex<Vec<(String, u16, Option<String>, bool)>>,
}

#[async_trait]
impl DatasetRepository for TestRepository {
    async fn save(&self, dataset: &Dataset) -> Result<(), DatasetRepositoryError> {
        *self.saved.lock().unwrap_or_else(|e| e.into_inner()) = Some(dataset.clone());

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
        tenant_id: &str,
        owner: &str,
        repo_id: &str,
        sha: &str,
    ) -> Result<Option<Dataset>, DatasetRepositoryError> {
        *self
            .huggingface_lookup
            .lock()
            .unwrap_or_else(|error| error.into_inner()) =
            Some((tenant_id.into(), owner.into(), repo_id.into(), sha.into()));

        Ok(None)
    }

    async fn list_by_owner(
        &self,
        _tenant_id: &str,
        _owner: &str,
        input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetRepositoryError> {
        self.capture_list_request("owner", input);

        Ok(empty_dataset_list())
    }

    async fn list_by_tenant(
        &self,
        tenant_id: &str,
        input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetRepositoryError> {
        self.capture_list_request(tenant_id, input);

        Ok(empty_dataset_list())
    }

    async fn list_shared_with_user(
        &self,
        _tenant_id: &str,
        _owner: &str,
        input: &ListDatasetsInput,
    ) -> Result<DatasetListOutput, DatasetRepositoryError> {
        self.capture_list_request("shared", input);

        Ok(empty_dataset_list())
    }
}

impl TestRepository {
    fn capture_list_request(&self, target: &str, input: &ListDatasetsInput) {
        self.list_requests
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push((
                target.into(),
                input.limit(),
                input.cursor().map(Into::into),
                input.include_count(),
            ));
    }
}

fn empty_dataset_list() -> DatasetListOutput {
    DatasetListOutput {
        datasets: Vec::new(),
        cursor: None,
        count: None,
    }
}

#[tokio::test]
async fn register_dataset_derives_identity_and_saves() -> Result<(), DatasetServiceError> {
    let repository = Arc::new(TestRepository::default());

    let service = DatasetService::new(repository.clone());
    let context = RequestContext::system(None);

    let dataset = service
        .register_dataset(
            &context,
            RegisterDatasetInput {
                tags: Vec::new(),
                provider: DatasetProviderInput::HuggingFace(HuggingFaceRepoLocatorInput {
                    id: "owner/repo".into(),
                    sha: "abc".into(),
                }),
                items: Vec::new(),
                size: 0,
                visibility: VisibilityInput::Private,
            },
        )
        .await?;

    assert_eq!(dataset.tenant_id(), context.actor_tenant_id());
    assert_eq!(dataset.owner(), context.actor_principal_id());

    assert_eq!(
        repository
            .saved
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map(Dataset::id),
        Some(dataset.id())
    );

    Ok(())
}

#[tokio::test]
async fn huggingface_snapshot_lookup_derives_tenant_and_owner_from_context(
) -> Result<(), DatasetServiceError> {
    let repository = Arc::new(TestRepository::default());

    let service = DatasetService::new(repository.clone());
    let context = RequestContext::system(None);

    let dataset = service
        .find_by_huggingface_repo_locator(&context, "owner/repo", "abc")
        .await?;

    assert!(dataset.is_none());
    assert_eq!(
        *repository
            .huggingface_lookup
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        Some((
            context.actor_tenant_id().clone(),
            context.actor_principal_id().clone(),
            "owner/repo".into(),
            "abc".into(),
        ))
    );

    Ok(())
}

#[tokio::test]
async fn dataset_list_methods_forward_pagination_options() -> Result<(), DatasetServiceError> {
    let repository = Arc::new(TestRepository::default());

    let service = DatasetService::new(repository.clone());
    let context = RequestContext::system(None);
    let input = ListDatasetsInput::new(Some(25), Some("cursor".into()), Some(true));

    let owned = service.list_for_user(&context, &input).await?;
    let shared = service.list_shared_with_user(&context, &input).await?;
    let global = service.list_global(&context, &input).await?;

    assert!(owned.datasets.is_empty());
    assert!(shared.datasets.is_empty());
    assert!(global.datasets.is_empty());
    assert_eq!(
        *repository
            .list_requests
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        vec![
            ("owner".into(), 25, Some("cursor".into()), true),
            ("shared".into(), 25, Some("cursor".into()), true),
            (GLOBAL_TENANT.into(), 25, Some("cursor".into()), true),
        ]
    );

    Ok(())
}
