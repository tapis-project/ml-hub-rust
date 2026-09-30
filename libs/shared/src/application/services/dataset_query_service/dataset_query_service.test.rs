use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use super::*;
use crate::{
    domain::entities::dataset::{Dataset, DatasetProvider, HuggingFaceRepoLocator},
    shared_kernel::value_objects::Tags,
};

#[derive(Default)]
struct TestRepository {
    found: Mutex<Option<DatasetQueryOutput>>,
    huggingface_lookup: Mutex<Option<(String, String, String, String)>>,
    list_requests: Mutex<Vec<(String, u16, Option<String>, bool)>>,
}

#[async_trait]
impl DatasetRepository for TestRepository {
    async fn save(&self, _dataset: &Dataset) -> Result<(), DatasetRepositoryError> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        _tenant_id: &str,
        _id: Uuid,
    ) -> Result<Option<DatasetQueryOutput>, DatasetRepositoryError> {
        Ok(self
            .found
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .clone())
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

#[tokio::test]
async fn missing_dataset_returns_not_found() {
    let repository = Arc::new(TestRepository::default());
    let service = DatasetQueryService::new(repository);
    let context = RequestContext::system(None);

    let result = service.get_dataset(&context, Uuid::now_v7()).await;

    assert!(matches!(result, Err(DatasetQueryServiceError::NotFound)));
}

#[tokio::test]
async fn inaccessible_private_dataset_is_concealed_as_not_found(
) -> Result<(), Box<dyn std::error::Error>> {
    let repository = Arc::new(TestRepository {
        found: Mutex::new(Some(dataset_output("another-owner", Visibility::Private)?)),
        ..Default::default()
    });
    let service = DatasetQueryService::new(repository);
    let context = RequestContext::system(None);

    let result = service.get_dataset(&context, Uuid::now_v7()).await;

    assert!(matches!(result, Err(DatasetQueryServiceError::NotFound)));

    Ok(())
}

#[tokio::test]
async fn public_dataset_is_visible_to_a_non_owner() -> Result<(), Box<dyn std::error::Error>> {
    let repository = Arc::new(TestRepository {
        found: Mutex::new(Some(dataset_output("another-owner", Visibility::Public)?)),
        ..Default::default()
    });
    let service = DatasetQueryService::new(repository);
    let context = RequestContext::system(None);

    let dataset = service.get_dataset(&context, Uuid::now_v7()).await?;

    assert_eq!(dataset.owner, "another-owner");

    Ok(())
}

#[tokio::test]
async fn huggingface_snapshot_lookup_derives_tenant_and_owner_from_context(
) -> Result<(), DatasetQueryServiceError> {
    let repository = Arc::new(TestRepository::default());
    let service = DatasetQueryService::new(repository.clone());
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
async fn list_methods_forward_pagination_options() -> Result<(), DatasetQueryServiceError> {
    let repository = Arc::new(TestRepository::default());
    let service = DatasetQueryService::new(repository.clone());
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

fn empty_dataset_list() -> DatasetListOutput {
    DatasetListOutput {
        datasets: Vec::new(),
        cursor: None,
        count: None,
    }
}

fn dataset_output(
    owner: &str,
    visibility: Visibility,
) -> Result<DatasetQueryOutput, Box<dyn std::error::Error>> {
    Ok(DatasetQueryOutput {
        id: Uuid::now_v7(),
        tenant_id: GLOBAL_TENANT.into(),
        owner: owner.into(),
        name: "dataset".into(),
        description: None,
        tags: Tags::new(Vec::new())?,
        provider: DatasetProvider::HuggingFace(HuggingFaceRepoLocator::new(
            "owner/repo".into(),
            "abc".into(),
        )?),
        items: Vec::new(),
        item_count: 0,
        size: 0,
        visibility,
    })
}
