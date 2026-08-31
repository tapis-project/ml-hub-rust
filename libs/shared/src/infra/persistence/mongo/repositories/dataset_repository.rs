use crate::{
    application::ports::{
        dataset::{DatasetRepository as DatasetRepositoryPort, DatasetRepositoryError},
        errors::InfrastructureError,
    },
    domain::entities::dataset as entities,
    infra::persistence::mongo::{
        database::DATASET_COLLECTION,
        documents::{
            dataset::Dataset as DatasetDocument, visibility::Visibility as DocumentVisibility,
        },
    },
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson},
    Client, Collection,
};

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
    ) -> Result<Option<entities::Dataset>, DatasetRepositoryError> {
        let id = mongodb::bson::Uuid::from_bytes(*id.as_bytes());

        let document = self
            .read_collection
            .find_one(doc! { "tenant_id": tenant_id, "id": id })
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
    ) -> Result<Vec<entities::Dataset>, DatasetRepositoryError> {
        self.list(doc! { "tenant_id": tenant_id, "owner": owner })
            .await
    }

    async fn list_shared_with_user(
        &self,
        tenant_id: &str,
        _owner: &str,
    ) -> Result<Vec<entities::Dataset>, DatasetRepositoryError> {
        let visibility = to_bson(&DocumentVisibility::Public).map_err(map_error)?;

        self.list(doc! { "tenant_id": tenant_id, "visibility": visibility })
            .await
    }
}

impl DatasetRepository {
    async fn list(
        &self,
        filter: mongodb::bson::Document,
    ) -> Result<Vec<entities::Dataset>, DatasetRepositoryError> {
        let mut cursor = self.read_collection.find(filter).await.map_err(map_error)?;

        let mut datasets = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            datasets.push(entities::Dataset::try_from(document).map_err(map_conversion_error)?);
        }

        Ok(datasets)
    }
}

fn map_error(error: impl std::fmt::Display) -> InfrastructureError {
    let e = InfrastructureError::new_internal();

    log::error!("[{}] Dataset persistence error: {}", e.error_id(), error);

    e
}

fn map_conversion_error(error: impl std::fmt::Display) -> DatasetRepositoryError {
    map_error(error).into()
}
