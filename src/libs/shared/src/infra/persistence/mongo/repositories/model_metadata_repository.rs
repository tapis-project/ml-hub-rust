use crate::application::errors::ApplicationError;
use crate::{application, domain};
use crate::domain::entities;
use mongodb::{
    bson::{
        doc,
        Uuid,
        to_bson,
        Bson,
    },
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

    async fn filter_model_metadata_by_criteria(&self, input: &application::inputs::discover_models::DiscoverModelsInput) -> Result<Vec<entities::model_metadata::ModelMetadata>, ApplicationError> {
        let mut filters: Vec<Bson> = Vec::new();
        for criterion in input.criteria.clone() {
            let metadata = ModelMetadataFilter::try_from(&criterion)
                .map_err(|err| ApplicationError::ConversionError(err.to_string()))?;

            let serialized_metadata = to_bson(&metadata)
                .map_err(|err| ApplicationError::ConversionError(format!("Failed to serialize ModelMetadata: {}", err.to_string())))?;
            
            filters.push(serialized_metadata);
        }
        
        let filter = doc! {
            "$or": filters
        };

        println!("{}", filter);

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let mut metadata_entries: Vec<entities::model_metadata::ModelMetadata> = Vec::new();
        while let Some(entry) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            let metadata = entities::model_metadata::ModelMetadata::try_from(entry)
                .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            metadata_entries.push(metadata)
        }

        return Ok(metadata_entries)
    }
}