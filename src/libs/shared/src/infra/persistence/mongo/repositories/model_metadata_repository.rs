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
    options::AggregateOptions,
    Database,
    Collection,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use serde::Deserialize;

use super::super::database::MODEL_METADATA_COLLECTION;
use super::super::documents::model_metadata_filter::ModelMetadataFilter;
use super::super::documents::model_metadata::ModelMetadata;

pub struct ModelMetadataRepository {
    read_collection: Collection<ModelMetadata>,
    write_collection: Collection<ModelMetadata>
}

#[derive(Deserialize)]
pub struct Count {
    total: i64
}

#[derive(Deserialize)]
pub struct SearchResponseDocument {
    results: Vec<ModelMetadata>,
    count: Option<Vec<Count>>
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
impl application::ports::repositories::ModelMetadataRepository for ModelMetadataRepository {
    async fn save(&self, input: &application::inputs::model_metadata::CreateModelMetadata) -> Result<(), ApplicationError> {
        let mut document = ModelMetadata::try_from(&input.metadata)
            .map_err(|err| ApplicationError::ConversionError(format!("Failed to convert from CreateModelInput to document::ModelMetadata: {}", err.to_string())))?;
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
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

        // Return documents the meet the criteria and sort by _id
        let mut results_pipeline = vec![
            doc! {
                "$match": { "$or": filters },
            }
        ];
        
        // Sort by _id by default
        results_pipeline.push(
            doc! {
                "$sort": { "_id": 1 }
            }
        );
        
        // Find all documents after the cursor
        if let Some(pagination_cursor) = input.options.cursor() {
            let oid = ObjectId::parse_str(pagination_cursor)
                .map_err(|err| ApplicationError::RepoError(format!("Invalid cursor value: {}", err.to_string())))?;

            results_pipeline.push(
                doc! {
                    "$match": {
                        "_id": { "$gt": oid }
                    }
                }
            )
        }

        // Return a limited number of docuemtns
        if let Some(limit) = input.options.limit() {
            results_pipeline.push(
                doc! {
                    "$limit": limit as i64
                }
            )
        }
        
        // Add results pipeline to the facet statemnt
        let mut facet_doc = Document::new();
        facet_doc.insert("results", results_pipeline);

        // Return the count if requested
        if input.options.include_count().unwrap_or_else(|| false) {
            facet_doc.insert("count", vec![doc! { "$count": "total" }]);
        }

        // Construct the facet document
        let pipeline = vec![
            doc! { "$facet": facet_doc }
        ];

        // Aggregate options
        let options = AggregateOptions::builder()
            .allow_disk_use(true)
            .batch_size(100)
            .comment(Some("Model Discovery Search".into()))
            .max_time(Some(Duration::from_secs(2)))
            .build();

        let mut cursor = self.read_collection.aggregate(pipeline, Some(options))
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;


        let mut models: Vec<entities::model_metadata::ModelMetadata> = Vec::new();
        let mut count: Option<i64> = None;
        let mut pagination_cursor: Option<String> = None;
        if let Some(doc) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            let response: SearchResponseDocument = from_document(doc)
                .map_err(|err| ApplicationError::RepoError(format!("Failed to convert to SearchResponseDocument: {}", err.to_string())))?;

            if let Some(last) = response.results.last() {
                pagination_cursor = last._id.clone().and_then(|oid| Some(oid.to_string()));
            };

            for entry in response.results.into_iter() {
                let model = entities::model_metadata::ModelMetadata::try_from(entry)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
                
                models.push(model);
            };

            if let Some(c) = response.count {
                count = Some(c[0].total);
            }
        }

        return Ok(
            DiscoverModelsOutput { models, count, cursor: pagination_cursor }
        )
    }
}