use std::time::Duration;
use crate::application::errors::ApplicationError;
use crate::application::outputs::discover_models::DiscoverModelsOutput;
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
    options::{AggregateOptions, EstimatedDocumentCountOptions},
    Database,
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
    pub fn new(db: &Database) -> Self {
        Self {
            write_collection: db.collection(MODEL_METADATA_COLLECTION),
            read_collection: db.collection(MODEL_METADATA_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::model_metadata::ModelMetadataRepository for ModelMetadataRepository {
    async fn save(&self, input: &application::inputs::model_metadata::CreateModelMetadata) -> Result<(), ApplicationError> {
        let mut document = ModelMetadata::try_from(&input.metadata)
            .map_err(|err| ApplicationError::ConversionError(format!("Failed to convert from CreateModelInput to document::ModelMetadata: {}", err.to_string())))?;
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn get_by_name_and_author(&self, name: &String, author: &String) -> Result<Option<entities::model_metadata::ModelMetadata>, ApplicationError> {
        let result = self.read_collection
            .find_one(doc!{ name: name, author: author }, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?
            .map(entities::model_metadata::ModelMetadata::try_from)
            .transpose()
            .map_err(|err| ApplicationError::ConversionError(err.to_string()))?;

        Ok(result)
    }

    async fn update_artifact_id(&self, input: &application::inputs::model_metadata::UpdateModelMetadataArtifactId) -> Result<(), ApplicationError> {
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
            .update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        Ok(())
    }

    async fn find_by_artifact_id(&self, artifact_id: &uuid::Uuid) -> Result<Option<entities::model_metadata::ModelMetadata>, ApplicationError> {
        let filter = doc! {
            "artifact_id": Uuid::from_bytes(*artifact_id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let maybe_metadata = match cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))? {
            Some(m) => {
                Some(domain::entities::model_metadata::ModelMetadata::try_from(m)
                    .map_err(|err| ApplicationError::ConversionError(err.to_string()))?)

            },
            None => None
        };

        Ok(maybe_metadata)
    }

    async fn filter_model_metadata_by_criteria(&self, input: &application::inputs::discover_models::DiscoverModelsInput) -> Result<DiscoverModelsOutput, ApplicationError> {
        let mut filters: Vec<Bson> = Vec::new();
        for criterion in input.criteria.clone() {
            let metadata = ModelMetadataFilter::try_from(&criterion)
                .map_err(|err| ApplicationError::ConversionError(err.to_string()))?;

            let serialized_metadata = to_bson(&metadata)
                .map_err(|err| ApplicationError::ConversionError(format!("Failed to serialize ModelMetadata: {}", err.to_string())))?;
            
            filters.push(serialized_metadata);
        }

        let mut aggregate: Vec<Document> = vec![];

        // Return documents the meet the criteria and sort by _id
        let mut match_document_value = doc! { "$or": filters };

        // Find all documents after the cursor
        if let Some(pagination_cursor) = input.options.cursor() {
            let oid = ObjectId::parse_str(pagination_cursor)
                .map_err(|err| ApplicationError::RepoError(format!("Invalid cursor value: {}", err.to_string())))?;

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

        // Limits batch size and max query time to ensure we don't DOS the db
        let options = AggregateOptions::builder()
            .allow_disk_use(true)
            .batch_size(100)
            .comment(Some("Model Discovery Search".into()))
            .max_time(Some(Duration::from_secs(2)))
            .build();

        let mut cursor = self.read_collection.aggregate(aggregate, Some(options))
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let mut models: Vec<entities::model_metadata::ModelMetadata> = Vec::with_capacity(limit as usize);
        let mut pagination_cursor: Option<String> = None;
        let mut returned_model_count = 0;
        while let Some(entry) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            returned_model_count += 1;
            let doc: ModelMetadata = from_document(entry)
                .map_err(|err| ApplicationError::RepoError(format!("Failed to convert Document to ModelMetadata: {}", err.to_string())))?;
            
            if returned_model_count <= limit {
                pagination_cursor = doc._id.and_then(|oid| Some(oid.to_string()));
                let model = entities::model_metadata::ModelMetadata::try_from(doc)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
                
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
            let estimate_count_options = EstimatedDocumentCountOptions::builder()
                .max_time(Duration::from_millis(100))
                .build();

            let returned_count = self.read_collection.estimated_document_count(estimate_count_options).await
                .map_err(|err| ApplicationError::RepoError(format!("Error fetching count: {}", err.to_string())))?;
            
            count = Some(returned_count as i64)
        }

        return Ok(
            DiscoverModelsOutput { models, count, cursor: pagination_cursor }
        )
    }
}