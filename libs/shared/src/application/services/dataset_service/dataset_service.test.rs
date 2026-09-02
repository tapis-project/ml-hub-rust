use super::*;
use crate::{
    application::{
        inputs::dataset::{DatasetProviderInput, HuggingFaceRepoLocatorInput, VisibilityInput},
        outputs::dataset::DatasetQueryOutput,
        ports::dataset::DatasetRepository,
    },
    shared_kernel::context::RequestContext,
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

struct TestRepository {
    saved: Mutex<Option<Dataset>>,
    huggingface_lookup: Mutex<Option<(String, String, String, String)>>,
    tenant_list: Mutex<Option<String>>,
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
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        Ok(Vec::new())
    }

    async fn list_by_tenant(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        *self
            .tenant_list
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some(tenant_id.into());

        Ok(Vec::new())
    }

    async fn list_shared_with_user(
        &self,
        _tenant_id: &str,
        _owner: &str,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn register_dataset_derives_identity_and_saves() -> Result<(), DatasetServiceError> {
    let repository = Arc::new(TestRepository {
        saved: Mutex::new(None),
        huggingface_lookup: Mutex::new(None),
        tenant_list: Mutex::new(None),
    });

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
    let repository = Arc::new(TestRepository {
        saved: Mutex::new(None),
        huggingface_lookup: Mutex::new(None),
        tenant_list: Mutex::new(None),
    });

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
async fn list_global_uses_the_global_tenant() -> Result<(), DatasetServiceError> {
    let repository = Arc::new(TestRepository {
        saved: Mutex::new(None),
        huggingface_lookup: Mutex::new(None),
        tenant_list: Mutex::new(None),
    });

    let service = DatasetService::new(repository.clone());
    let context = RequestContext::system(None);

    let datasets = service.list_global(&context).await?;

    assert!(datasets.is_empty());
    assert_eq!(
        repository
            .tenant_list
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .as_deref(),
        Some(GLOBAL_TENANT)
    );

    Ok(())
}
