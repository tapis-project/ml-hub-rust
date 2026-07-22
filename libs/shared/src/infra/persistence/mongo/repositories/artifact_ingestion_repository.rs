use crate::application::ports::artifacts::ArtifactIngestionRepositoryError;
use crate::application::ports::errors::CommonRepositoryError;
use crate::infra::persistence::mongo::database::ARTIFACT_INGESTION_COLLECTION;
use crate::infra::artifacts::mongo::documents::ArtifactType as ArtifactTypeDoc;
use crate::infra::persistence::mongo::documents::artifact_ingestion::{ArtifactIngestion, UpdateArtifactIngestionRequest, UpdateArtifactIngestionStatusRequest};
use crate::application;
use crate::domain::entities;
use mongodb::{
    bson::{
        doc,
        Uuid
    },
    Client,
    Collection,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;

pub struct ArtifactIngestionRepository {
    read_collection: Collection<ArtifactIngestion>,
    write_collection: Collection<ArtifactIngestion>
}

impl ArtifactIngestionRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let db = client.database(&db_name);
        
        Self {
            write_collection: db.collection(ARTIFACT_INGESTION_COLLECTION),
            read_collection: db.collection(ARTIFACT_INGESTION_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::artifacts::ArtifactIngestionRepository for ArtifactIngestionRepository {
    async fn save(&self, ingestion: &entities::artifact_ingestion::ArtifactIngestion) -> Result<(), ArtifactIngestionRepositoryError> {
        let mut document = ArtifactIngestion::from(ingestion.clone());
        
        let result = self.write_collection.insert_one(&document)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn update(&self, ingestion: &entities::artifact_ingestion::ArtifactIngestion) -> Result<(), ArtifactIngestionRepositoryError>  {
        let update = UpdateArtifactIngestionRequest::from(ingestion.clone());

        let filter = doc! {
            "id": Uuid::from_bytes(*ingestion.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "status": String::from(update.status),
                "last_modified": update.last_modified,
                "last_message": update.last_message,
                "webhook_url": update.webhook_url,
                "artifact_path": update.artifact_path,
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

    async fn update_status(&self, ingestion: &entities::artifact_ingestion::ArtifactIngestion) -> Result<(), ArtifactIngestionRepositoryError> {
        let update = UpdateArtifactIngestionStatusRequest::from(ingestion.clone());

        let filter = doc! {
            "id": Uuid::from_bytes(*ingestion.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "status": String::from(update.status),
                "last_modified": update.last_modified,
                "last_message": update.last_message
            }
        };

        self.write_collection.update_one(filter, document)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;
        
        Ok(())
    }

    async fn find_by_artifact_id(&self, artifact_id: &uuid::Uuid) -> Result<Vec<entities::artifact_ingestion::ArtifactIngestion>, ArtifactIngestionRepositoryError> {
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

        let mut ingestions: Vec<entities::artifact_ingestion::ArtifactIngestion> = Vec::new();
        while let Some(ingestion_doc) = cursor.try_next()
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })? 
        {
            let ingestion = entities::artifact_ingestion::ArtifactIngestion::try_from(ingestion_doc)
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?;

            ingestions.push(ingestion);
        }

        Ok(ingestions)
    }

    async fn find_by_artifact_type(&self, artifact_type: entities::artifact::ArtifactType) -> Result<Vec<entities::artifact_ingestion::ArtifactIngestion>, ArtifactIngestionRepositoryError> {
        let filter = doc! {
            "artifact_type": String::from(ArtifactTypeDoc::from(artifact_type))
        };

        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;
        
        let mut ingestions: Vec<entities::artifact_ingestion::ArtifactIngestion> = Vec::new();
        while let Some(ingestion_doc) = cursor.try_next()
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })? 
        {
            let ingestion = entities::artifact_ingestion::ArtifactIngestion::from(ingestion_doc);

            ingestions.push(ingestion);
        }

        Ok(ingestions)
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<entities::artifact_ingestion::ArtifactIngestion>, ArtifactIngestionRepositoryError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        while let Some(ingestion_doc) = cursor.try_next()
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?
        {
            let ingestion = entities::artifact_ingestion::ArtifactIngestion::try_from(ingestion_doc)
                .map_err(|e| {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?;

            return Ok(Some(ingestion))
        }

        Ok(None)
    }
}