use std::{
    collections::HashMap,
    fs::write,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use shared::{
    application::{
        inputs::dataset::ListDatasetsInput,
        outputs::dataset::{DatasetListOutput, DatasetQueryOutput},
        ports::dataset::{DatasetRepository, DatasetRepositoryError},
    },
    domain::entities::dataset::{Dataset, DatasetProvider},
};
use tempfile::tempdir;
use uuid::Uuid;

use super::*;

#[derive(Default)]
struct TestDatasetRepository {
    snapshots: Mutex<HashMap<(String, String), Dataset>>,
}

#[async_trait]
impl DatasetRepository for TestDatasetRepository {
    async fn save(&self, dataset: &Dataset) -> Result<(), DatasetRepositoryError> {
        if let DatasetProvider::HuggingFace(locator) = dataset.provider() {
            self.snapshots
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .insert((locator.id().into(), locator.sha().into()), dataset.clone());
        }

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
        repo_id: &str,
        sha: &str,
    ) -> Result<Option<Dataset>, DatasetRepositoryError> {
        let dataset = self
            .snapshots
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(&(repo_id.into(), sha.into()))
            .cloned();

        Ok(dataset)
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

fn empty_dataset_list() -> DatasetListOutput {
    DatasetListOutput {
        datasets: Vec::new(),
        cursor: None,
        count: None,
    }
}

#[tokio::test]
async fn skips_an_existing_sha_and_registers_a_changed_sha(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempdir()?;
    let records = [
        record_json("abc123"),
        record_json("abc123"),
        record_json("def456"),
    ]
    .join("\n");

    write(directory.path().join("datasets_00000.jsonl"), records)?;

    let repository = Arc::new(TestDatasetRepository::default());
    let service = DatasetService::new(repository.clone());
    let context = RequestContext::system(None);

    let summary = process_inbox(directory.path(), None, &service, &context).await?;

    assert_eq!(summary.processed, 3);
    assert_eq!(summary.registered, 2);
    assert_eq!(summary.skipped_existing, 1);
    assert_eq!(summary.rejected, 0);
    assert_eq!(
        repository
            .snapshots
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .len(),
        2
    );

    Ok(())
}

fn record_json(sha: &str) -> String {
    format!(
        r#"{{"id":"owner/dataset","sha":"{sha}","tags":["text"],"private":false,"gated":false,"siblings":[{{"rfilename":"data.parquet","size":10}}]}}"#
    )
}
