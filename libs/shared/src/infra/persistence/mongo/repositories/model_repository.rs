use std::time::Duration;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, Document, Uuid},
    Client, Collection,
};

use crate::{
    application::{
        inputs::model::ListModelsInput,
        ports::{
            errors::InfrastructureError,
            model::{ModelPage, ModelRepository as ModelRepositoryPort, ModelRepositoryError},
        },
    },
    domain::entities::model::Model as DomainModel,
    infra::{
        _common::mongo::is_duplicate_key_error,
        persistence::mongo::{database::MODEL_COLLECTION, documents::model::Model},
    },
    shared_kernel::identifiers::ExternalModelId,
};

pub struct ModelRepository {
    collection: Collection<Model>,
}

impl ModelRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        Self {
            collection: client.database(&db_name).collection(MODEL_COLLECTION),
        }
    }

    async fn find_one(
        &self,
        filter: Document,
    ) -> Result<Option<DomainModel>, ModelRepositoryError> {
        self.collection
            .find_one(filter)
            .await
            .map_err(map_error)?
            .map(TryInto::try_into)
            .transpose()
            .map_err(map_conversion_error)
    }

    async fn list(
        &self,
        mut filter: Document,
        input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError> {
        if let Some(cursor) = input.cursor() {
            filter
                .extend(doc! { "_id": { "$gt": ObjectId::parse_str(cursor).map_err(map_error)? } });
        }

        let count_filter = filter.clone();

        let pipeline = vec![
            doc! { "$match": filter },
            doc! { "$sort": { "_id": 1 } },
            doc! { "$limit": i64::from(input.limit()) + 1 },
        ];
        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .map_err(map_error)?;

        let mut documents = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            documents.push(from_document::<Model>(document).map_err(map_error)?);
        }

        let next_cursor = if documents.len() > usize::from(input.limit()) {
            documents.pop();
            documents
                .last()
                .and_then(|document| document._id.map(|id| id.to_hex()))
        } else {
            None
        };

        let models = documents
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(map_conversion_error)?;

        let count = if input.include_count() {
            Some(
                self.collection
                    .count_documents(count_filter)
                    .max_time(Duration::from_secs(2))
                    .await
                    .map_err(map_error)?,
            )
        } else {
            None
        };

        Ok(ModelPage {
            models,
            count,
            cursor: next_cursor,
        })
    }
}

#[async_trait]
impl ModelRepositoryPort for ModelRepository {
    async fn save(&self, model: &DomainModel) -> Result<(), ModelRepositoryError> {
        let document = Model::from(model);

        self.collection
            .insert_one(document)
            .await
            .map_err(|error| {
                if is_duplicate_key_error(&error) {
                    ModelRepositoryError::ModelAlreadyInCollection
                } else {
                    map_error(error)
                }
            })?;

        Ok(())
    }

    async fn update(&self, model: &DomainModel) -> Result<(), ModelRepositoryError> {
        let document = Model::from(model);

        self.collection
            .replace_one(doc! { "id": &document.id }, document)
            .await
            .map_err(|error| {
                if is_duplicate_key_error(&error) {
                    if model.artifact_id().is_some() {
                        ModelRepositoryError::ArtifactAlreadyAssociated
                    } else {
                        ModelRepositoryError::ModelAlreadyInCollection
                    }
                } else {
                    map_error(error)
                }
            })?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        tenant_id: &str,
        id: uuid::Uuid,
    ) -> Result<Option<DomainModel>, ModelRepositoryError> {
        self.find_one(doc! { "tenant_id": tenant_id, "id": Uuid::from_bytes(*id.as_bytes()) })
            .await
    }

    async fn find_by_external_model_id(
        &self,
        tenant_id: &str,
        owner: &str,
        id: &ExternalModelId,
    ) -> Result<Option<DomainModel>, ModelRepositoryError> {
        self.find_one(doc! { "tenant_id": tenant_id, "owner": owner, "external_model_id": Uuid::from_bytes(*id.as_uuid().as_bytes()) }).await
    }

    async fn find_by_artifact_id(
        &self,
        artifact_id: &uuid::Uuid,
    ) -> Result<Option<DomainModel>, ModelRepositoryError> {
        self.find_one(doc! { "artifact_id": Uuid::from_bytes(*artifact_id.as_bytes()) })
            .await
    }

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
        input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError> {
        self.list(doc! { "tenant_id": tenant_id, "owner": owner }, input)
            .await
    }

    async fn list_shared(
        &self,
        tenant_id: &str,
        owner: &str,
        input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError> {
        self.list(
            doc! { "tenant_id": tenant_id, "owner": { "$ne": owner }, "visibility": "Public" },
            input,
        )
        .await
    }
}

fn map_error(error: impl std::fmt::Display) -> ModelRepositoryError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] Model persistence error: {}",
        infrastructure_error.error_id(),
        error
    );

    infrastructure_error.into()
}

fn map_conversion_error(error: impl std::fmt::Display) -> ModelRepositoryError {
    map_error(error)
}
