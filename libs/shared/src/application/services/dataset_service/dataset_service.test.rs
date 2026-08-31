use super::*;
use crate::{
    application::{
        inputs::dataset::{DatasetProviderInput, HuggingFaceRepoLocatorInput, VisibilityInput},
        ports::dataset::DatasetRepository,
    },
    shared_kernel::context::RequestContext,
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

struct TestRepository {
    saved: Mutex<Option<Dataset>>,
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
    ) -> Result<Option<Dataset>, DatasetRepositoryError> {
        Ok(None)
    }

    async fn list_by_owner(
        &self,
        _tenant_id: &str,
        _owner: &str,
    ) -> Result<Vec<Dataset>, DatasetRepositoryError> {
        Ok(Vec::new())
    }

    async fn list_shared_with_user(
        &self,
        _tenant_id: &str,
        _owner: &str,
    ) -> Result<Vec<Dataset>, DatasetRepositoryError> {
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn register_dataset_derives_identity_and_saves() -> Result<(), DatasetServiceError> {
    let repository = Arc::new(TestRepository {
        saved: Mutex::new(None),
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
