use crate::common::application::errors::ApplicationError;
use crate::common::infra::persistence::mongo::database::{
    ARTIFACT_COLLECTION,
    ARTIFACT_INGESTION_COLLECTION,
};
use crate::common::infra::persistence::mongo::documents::{Artifact, ArtifactIngestion, UpdateArtifactIngestionStatusRequest};
use crate::common::application;
use crate::common::domain::entities;
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
    async fn save(&self, artifact: &entities::Artifact) -> Result<(), ApplicationError> {
        let mut document = Artifact::from(artifact.clone());
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<entities::Artifact>, ApplicationError> {
        let mut cursor = self.read_collection.find(None, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let mut artifacts:Vec<entities::Artifact> = Vec::new();
        while let Some(artifact) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            artifacts.push(
                entities::Artifact::try_from(artifact)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?
            );
        }
        
        Ok(artifacts)
    }

    async fn get_by_id(&self, id: uuid::Uuid) -> Result<Option<entities::Artifact>, ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        while let Some(artifact_doc) = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?  {
            let artifact = entities::Artifact::try_from(artifact_doc)
                .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            return Ok(Some(artifact))
        }

        Ok(None)
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
    async fn save(&self, ingestion: &entities::ArtifactIngestion) -> Result<(), ApplicationError> {
        let mut document = ArtifactIngestion::from(ingestion.clone());
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn update_status(&self, ingestion: &entities::ArtifactIngestion) -> Result<(), ApplicationError> {
        
        let update = UpdateArtifactIngestionStatusRequest::from(ingestion.clone());

        let filter = doc! {
            "id": update.id
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

    async fn find_by_artifact_id(&self, artifact_id: uuid::Uuid) -> Result<Vec<entities::ArtifactIngestion>, ApplicationError> {
        let filter = doc! {
            "artifact_id": Uuid::from_bytes(*artifact_id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let mut ingestions: Vec<entities::ArtifactIngestion> = Vec::new();
        while let Some(ingestion_doc) = cursor.try_next()
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))? 
        {
            let ingestion = entities::ArtifactIngestion::try_from(ingestion_doc)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            ingestions.push(ingestion);
        }

        Ok(ingestions)
    }
}