use std::time::Duration;
use crate::application::ports::errors::CommonRepositoryError;
use crate::application::ports::model_metadata::ModelMetadataRepositoryError;
use crate::shared_kernel::context::RequestContext;
use crate::{application, domain};
use crate::domain::entities;
use bson::oid::ObjectId;
use mongodb::{
    bson::{
        doc,
        from_document,
        Uuid,
        to_bson,
        Bson,
        Document,
    },
    Client,
    Collection,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;

use super::super::database::MODEL_METADATA_COLLECTION;
use super::super::documents::model_metadata_filter::ModelMetadataFilter;
use super::super::documents::model_metadata::ModelMetadata;

pub struct ModelMetadataRepository {
    read_collection: Collection<ModelMetadata>,
    write_collection: Collection<ModelMetadata>
}

impl ModelMetadataRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let db = client.database(&db_name);
        
        Self {
            write_collection: db.collection(MODEL_METADATA_COLLECTION),
            read_collection: db.collection(MODEL_METADATA_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::model_metadata::ModelMetadataRepository for ModelMetadataRepository {
    async fn upsert(
        &self,
        metadata: &entities::model_metadata::ModelMetadata,
        ctx: &RequestContext,
    ) -> Result<(), ModelMetadataRepositoryError> {
        let document = ModelMetadata::try_from((metadata, ctx))
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                error
            })?;

        let filter = doc! {  
            "name": &document.name,
            "author": &document.author,
        };

        self.write_collection.replace_one(filter, &document)
            .upsert(true)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        Ok(())
    }

    async fn find_by_author_and_name(&self, author: &String, name: &String, tenant_id: &String) -> Result<Option<entities::model_metadata::ModelMetadata>, ModelMetadataRepositoryError> {
        let result = self.read_collection
            .find_one(doc!{ "tenant_id": tenant_id, "author": author, "name": name })
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?
            .map(entities::model_metadata::ModelMetadata::try_from)
            .transpose()
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                error
            })?;

        Ok(result)
    }

    async fn find_all_by_author(&self, author: &String, tenant_id: &String) -> Result<Vec<entities::model_metadata::ModelMetadata>, ModelMetadataRepositoryError> {
        let filter = doc!{ "tenant_id": tenant_id, "author": author };

        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;


        let mut results: Vec<entities::model_metadata::ModelMetadata> = vec![];
        while let Some(entry) = cursor.try_next().await.map_err(|e| {
            let error = CommonRepositoryError::new_internal();
            log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
            error
        })? {
            let entity = domain::entities::model_metadata::ModelMetadata::try_from(entry)
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?;

            results.push(entity);
        }

        Ok(results)
    }

    async fn update_artifact_id(&self, input: &application::inputs::model_metadata::UpdateModelMetadataArtifactId) -> Result<(), ModelMetadataRepositoryError> {
        let filter = doc! {
            "name": input.name.clone(),
            "author": input.author.clone(),
        };
        
        let document = doc! {
            "$set": {
                "artifact_id": Uuid::from_bytes(*input.artifact_id.as_bytes())
            }
        };

        self.write_collection
            .update_one(filter, document)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        Ok(())
    }

    async fn find_by_artifact_id(&self, artifact_id: &uuid::Uuid) -> Result<Option<entities::model_metadata::ModelMetadata>, ModelMetadataRepositoryError> {
        let filter = doc! {
            "artifact_id": Uuid::from_bytes(*artifact_id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        let maybe_metadata = match cursor
                .try_next()
                .await
                .map_err(|e|
            {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?
        {
            Some(m) => {
                Some(
                    domain::entities::model_metadata::ModelMetadata::try_from(m)
                        .map_err(|e| {
                            let error = CommonRepositoryError::new_internal();
                            log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                            error
                        })?
                )

            },
            None => None
        };

        Ok(maybe_metadata)
    }

    async fn search(
        &self,
        input: &application::inputs::discover_models::SearchModelsInput,
        tenant_ids: &Vec<String>
    ) -> Result<application::ports::model_metadata::ModelSearchResult, ModelMetadataRepositoryError> {
        let mut filters: Vec<Bson> = Vec::new();
        for criterion in input.criteria.clone() {
            let filter = ModelMetadataFilter::try_from((&criterion, tenant_ids))
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?;

            let serialized_filter = to_bson(&filter)
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Serialization error: {}", error.error_id(), e.to_string());
                    error
                })?;

            filters.push(serialized_filter);
        }

        let mut aggregate: Vec<Document> = vec![];

        // Return documents the meet the criteria and sort by _id
        let mut match_document_value = doc! { "$or": filters };

        // Find all documents after the cursor
        if let Some(pagination_cursor) = input.options.cursor() {
            let oid = ObjectId::parse_str(pagination_cursor)
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                    error
                })?;

            match_document_value.extend(doc! { "_id": { "$gt": oid }});
        }

        let match_document = doc! {
            "$match": match_document_value,
        };

        aggregate.push(match_document);
        
        // Sort by _id by default
        aggregate.push(
            doc! {
                "$sort": { "_id": 1 }
            }
        );

        // Return a limited number of documents + 1 to help use determine
        // if we should return a pagination cursor
        let limit = input.options.limit().unwrap_or_else(|| 0);
        aggregate.push(
            doc! {
                "$limit": (limit + 1) as i64
            }
        );

        let mut cursor = self.read_collection
            .aggregate(aggregate)
            .allow_disk_use(true)
            .batch_size(100)
            .comment(String::from("Model Discovery Search"))
            .max_time(Duration::from_secs(2))
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        let mut models: Vec<entities::model_metadata::ModelMetadata> = Vec::with_capacity(limit as usize);
        let mut pagination_cursor: Option<String> = None;
        let mut returned_model_count = 0;
        while let Some(entry) = cursor.try_next().await.map_err(|e| {
            let error = CommonRepositoryError::new_internal();
            log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
            error
        })? {
            returned_model_count += 1;
            let doc: ModelMetadata = from_document(entry)
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?;
            
            if returned_model_count <= limit {
                pagination_cursor = doc._id.and_then(|oid| Some(oid.to_string()));
                let model = entities::model_metadata::ModelMetadata::try_from(doc)
                    .map_err(|e| {
                        let error = CommonRepositoryError::new_internal();
                        log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                        error
                    })?;
                
                models.push(model);
            }
        }

        // Determine whether a pagination cursor should be sent back
        if returned_model_count <= limit {
            pagination_cursor = None;
        }

        // Return the count if requested
        let mut count: Option<i64> = None;
        if input.options.include_count().unwrap_or_else(|| false) {
            let returned_count = self.read_collection
                .estimated_document_count()
                .max_time(Duration::from_millis(100))
                .await
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                    error
                })?;
            
            count = Some(returned_count as i64)
        }

        return Ok(
            application::ports::model_metadata::ModelSearchResult { models, count, cursor: pagination_cursor }
        )
    }
}