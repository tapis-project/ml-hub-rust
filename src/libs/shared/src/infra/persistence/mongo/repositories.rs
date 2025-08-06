use crate::application::errors::ApplicationError;
use crate::infra::persistence::mongo::database::{
    ARTIFACT_COLLECTION,
    ARTIFACT_INGESTION_COLLECTION,
};
use crate::infra::persistence::mongo::documents::artifact::{Artifact, UpdateArtifactRequest, UpdateArtifactPathRequest};
use crate::infra::persistence::mongo::documents::artifact_ingestion::{ArtifactIngestion, UpdateArtifactIngestionRequest, UpdateArtifactIngestionStatusRequest};
use crate::application;
use crate::domain::entities;
use mongodb::{
    bson::{
        doc,
        Uuid
    },
    Database,
    Collection,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
// use crate::infra::persistence::errors::DatabaseError;

pub struct ArtifactRepository {
    read_collection: Collection<Artifact>,
    write_collection: Collection<Artifact>
}

impl ArtifactRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            write_collection: db.collection(ARTIFACT_COLLECTION),
            read_collection: db.collection(ARTIFACT_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::repositories::ArtifactRepository for ArtifactRepository {
    async fn save(&self, artifact: &entities::artifact::Artifact) -> Result<(), ApplicationError> {
        let mut document = Artifact::from(artifact.clone());
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<entities::artifact::Artifact>, ApplicationError> {
        let mut cursor = self.read_collection.find(None, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let mut artifacts:Vec<entities::artifact::Artifact> = Vec::new();
        while let Some(artifact) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            artifacts.push(
                entities::artifact::Artifact::try_from(artifact)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?
            );
        }
        
        Ok(artifacts)
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<entities::artifact::Artifact>, ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        while let Some(artifact_doc) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            let artifact = entities::artifact::Artifact::try_from(artifact_doc)
                .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            return Ok(Some(artifact))
        }

        Ok(None)
    }

    async fn update(&self, artifact: &entities::artifact::Artifact) -> Result<(), ApplicationError>  {
        let update = UpdateArtifactRequest::try_from(artifact.clone())?;

        let filter = doc! {
            "id": Uuid::from_bytes(*artifact.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "last_modified": update.last_modified,
                "path": update.path,
            }
        };

        self.write_collection
            .update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        Ok(())
    }

    async fn update_path(&self, artifact: &entities::artifact::Artifact) -> Result<(), ApplicationError> {
        let update = UpdateArtifactPathRequest::try_from(artifact.clone())?;

        let filter = doc! {
            "id": Uuid::from_bytes(*artifact.id.as_bytes()),
        };
        
        let document = doc! {
            "$set": {
                "path": update.path,
                "last_modified": update.last_modified,
            }
        };

        self.write_collection.update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        Ok(())
    }
}

pub struct ArtifactIngestionRepository {
    read_collection: Collection<ArtifactIngestion>,
    write_collection: Collection<ArtifactIngestion>
}

impl ArtifactIngestionRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            write_collection: db.collection(ARTIFACT_INGESTION_COLLECTION),
            read_collection: db.collection(ARTIFACT_INGESTION_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::repositories::ArtifactIngestionRepository for ArtifactIngestionRepository {
    async fn save(&self, ingestion: &entities::artifact_ingestion::ArtifactIngestion) -> Result<(), ApplicationError> {
        let mut document = ArtifactIngestion::from(ingestion.clone());
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn update(&self, ingestion: &entities::artifact_ingestion::ArtifactIngestion) -> Result<(), ApplicationError>  {
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
            .update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        Ok(())
    }

    async fn update_status(&self, ingestion: &entities::artifact_ingestion::ArtifactIngestion) -> Result<(), ApplicationError> {
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

        self.write_collection.update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        Ok(())
    }

    async fn find_by_artifact_id(&self, artifact_id: uuid::Uuid) -> Result<Vec<entities::artifact_ingestion::ArtifactIngestion>, ApplicationError> {
        let filter = doc! {
            "artifact_id": Uuid::from_bytes(*artifact_id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let mut ingestions: Vec<entities::artifact_ingestion::ArtifactIngestion> = Vec::new();
        while let Some(ingestion_doc) = cursor.try_next()
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))? 
        {
            let ingestion = entities::artifact_ingestion::ArtifactIngestion::try_from(ingestion_doc)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            ingestions.push(ingestion);
        }

        Ok(ingestions)
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<entities::artifact_ingestion::ArtifactIngestion>, ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        while let Some(ingestion_doc) = cursor.try_next()
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))? 
        {
            let ingestion = entities::artifact_ingestion::ArtifactIngestion::try_from(ingestion_doc)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            return Ok(Some(ingestion))
        }

        Ok(None)
    }
}