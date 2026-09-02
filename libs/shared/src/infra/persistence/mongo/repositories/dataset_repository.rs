use crate::{
    application::{
        outputs::dataset::DatasetQueryOutput,
        ports::{
            dataset::{DatasetRepository as DatasetRepositoryPort, DatasetRepositoryError},
            errors::InfrastructureError,
        },
    },
    domain::entities::dataset as entities,
    infra::persistence::mongo::{
        database::DATASET_COLLECTION,
        documents::{
            dataset::{
                Dataset as DatasetDocument, DatasetProvider as DatasetDocumentProvider,
                DatasetQuery as DatasetQueryDocument,
            },
            visibility::Visibility as DocumentVisibility,
        },
    },
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, from_document, to_bson, Document},
    Client, Collection,
};

const DATASET_QUERY_ITEM_LIMIT: i32 = 50;

pub struct DatasetRepository {
    read_collection: Collection<DatasetDocument>,
    write_collection: Collection<DatasetDocument>,
}

impl DatasetRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let database = client.database(&db_name);

        Self {
            read_collection: database.collection(DATASET_COLLECTION),
            write_collection: database.collection(DATASET_COLLECTION),
        }
    }
}

#[async_trait]
impl DatasetRepositoryPort for DatasetRepository {
    async fn save(&self, dataset: &entities::Dataset) -> Result<(), DatasetRepositoryError> {
        self.write_collection
            .insert_one(DatasetDocument::from(dataset))
            .await
            .map_err(map_error)?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        tenant_id: &str,
        id: uuid::Uuid,
    ) -> Result<Option<DatasetQueryOutput>, DatasetRepositoryError> {
        let id = mongodb::bson::Uuid::from_bytes(*id.as_bytes());

        let pipeline = dataset_query_pipeline(dataset_id_filter(tenant_id, id), true);
        let mut cursor = self
            .read_collection
            .aggregate(pipeline)
            .await
            .map_err(map_error)?;

        let document = cursor.try_next().await.map_err(map_error)?;

        document
            .map(|document| from_document::<DatasetQueryDocument>(document).map_err(map_error))
            .transpose()
            .map_err(DatasetRepositoryError::from)?
            .map(DatasetQueryOutput::try_from)
            .transpose()
            .map_err(map_conversion_error)
    }

    async fn find_by_huggingface_repo_locator(
        &self,
        tenant_id: &str,
        owner: &str,
        repo_id: &str,
        sha: &str,
    ) -> Result<Option<entities::Dataset>, DatasetRepositoryError> {
        let provider = to_bson(&DatasetDocumentProvider::HuggingFace).map_err(map_error)?;

        let document = self
            .read_collection
            .find_one(doc! {
                "tenant_id": tenant_id,
                "owner": owner,
                "provider": provider,
                "huggingface_repo_locator.id": repo_id,
                "huggingface_repo_locator.sha": sha,
            })
            .await
            .map_err(map_error)?;

        document
            .map(entities::Dataset::try_from)
            .transpose()
            .map_err(map_conversion_error)
    }

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        self.list(owner_filter(tenant_id, owner)).await
    }

    async fn list_by_tenant(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        self.list(tenant_filter(tenant_id)).await
    }

    async fn list_shared_with_user(
        &self,
        tenant_id: &str,
        _owner: &str,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        let visibility = to_bson(&DocumentVisibility::Public).map_err(map_error)?;

        self.list(shared_filter(tenant_id, visibility)).await
    }
}

impl DatasetRepository {
    async fn list(
        &self,
        filter: Document,
    ) -> Result<Vec<DatasetQueryOutput>, DatasetRepositoryError> {
        let pipeline = dataset_query_pipeline(filter, false);
        let mut cursor = self
            .read_collection
            .aggregate(pipeline)
            .await
            .map_err(map_error)?;

        let mut datasets = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            let document = from_document::<DatasetQueryDocument>(document).map_err(map_error)?;

            datasets.push(DatasetQueryOutput::try_from(document).map_err(map_conversion_error)?);
        }

        Ok(datasets)
    }
}

fn dataset_id_filter(tenant_id: &str, id: mongodb::bson::Uuid) -> Document {
    doc! { "tenant_id": tenant_id, "id": id }
}

fn owner_filter(tenant_id: &str, owner: &str) -> Document {
    doc! { "tenant_id": tenant_id, "owner": owner }
}

fn tenant_filter(tenant_id: &str) -> Document {
    doc! { "tenant_id": tenant_id }
}

fn shared_filter(tenant_id: &str, visibility: mongodb::bson::Bson) -> Document {
    doc! { "tenant_id": tenant_id, "visibility": visibility }
}

fn dataset_query_pipeline(filter: Document, single_result: bool) -> Vec<Document> {
    let mut pipeline = vec![
        doc! { "$match": filter },
        doc! {
            "$project": {
                "id": 1,
                "tenant_id": 1,
                "owner": 1,
                "tags": 1,
                "provider": 1,
                "huggingface_repo_locator": 1,
                "tapis_system_locator": 1,
                "items": { "$slice": ["$items", DATASET_QUERY_ITEM_LIMIT] },
                "item_count": { "$toLong": { "$size": "$items" } },
                "size": 1,
                "visibility": 1,
            }
        },
    ];

    if single_result {
        pipeline.push(doc! { "$limit": 1 });
    }

    pipeline
}

fn map_error(error: impl std::fmt::Display) -> InfrastructureError {
    let e = InfrastructureError::new_internal();

    log::error!("[{}] Dataset persistence error: {}", e.error_id(), error);

    e
}

fn map_conversion_error(error: impl std::fmt::Display) -> DatasetRepositoryError {
    map_error(error).into()
}

#[cfg(test)]
#[path = "dataset_repository.test.rs"]
mod dataset_repository_test;
