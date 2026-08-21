use crate::application::ports::artifacts::ArtifactRepositoryError;
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::artifact::ArtifactType as ArtifactTypeEntity;
use crate::infra::artifacts::mongo::ARTIFACT_COLLECTION;
use crate::infra::artifacts::mongo::documents::{Artifact, ArtifactType, UpdateArtifactRequest, UpdateArtifactPathRequest};
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

pub struct ArtifactRepository {
    read_collection: Collection<Artifact>,
    write_collection: Collection<Artifact>
}

impl ArtifactRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let db = client.database(&db_name);
        
        Self {
            write_collection: db.collection(ARTIFACT_COLLECTION),
            read_collection: db.collection(ARTIFACT_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::artifacts::ArtifactRepository for ArtifactRepository {
    async fn save(&self, artifact: &entities::artifact::Artifact) -> Result<(), ArtifactRepositoryError> {
        let mut document = Artifact::from(artifact.clone());
        
        let result = self.write_collection.insert_one(&document)
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn list_by_artifact_type(&self, artifact_type: ArtifactTypeEntity) -> Result<Vec<entities::artifact::Artifact>, ArtifactRepositoryError> {
        let filter = doc! {
            "artifact_type": String::from(ArtifactType::from(artifact_type))
        };
        
        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        let mut artifacts:Vec<entities::artifact::Artifact> = Vec::new();
        while let Some(artifact) = cursor.try_next()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?
        {
            artifacts.push(
                entities::artifact::Artifact::try_from(artifact)
                .map_err(|e| {
                    let error = InfrastructureError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?
            );
        }
        
        Ok(artifacts)
    }

    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<entities::artifact::Artifact>, ArtifactRepositoryError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        while let Some(artifact_doc) = cursor.try_next()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?
        {
            let artifact = entities::artifact::Artifact::try_from(artifact_doc)
                .map_err(|e| {
                    let error = InfrastructureError::new_internal();
                    log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                    error
                })?;

            return Ok(Some(artifact))
        }

        Ok(None)
    }

    async fn update(&self, artifact: &entities::artifact::Artifact) -> Result<(), ArtifactRepositoryError>  {
        let update = UpdateArtifactRequest::try_from(artifact.clone())
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Conversion error: {}", error.error_id(), e.to_string());
                error
            })?;

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
            .update_one(filter, document)
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        Ok(())
    }

    async fn update_path(&self, artifact: &entities::artifact::Artifact) -> Result<(), ArtifactRepositoryError> {
        let update = UpdateArtifactPathRequest::try_from(artifact.clone())
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        let filter = doc! {
            "id": Uuid::from_bytes(*artifact.id.as_bytes()),
        };
        
        let document = doc! {
            "$set": {
                "path": update.path,
                "last_modified": update.last_modified,
            }
        };

        self.write_collection.update_one(filter, document)
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;
        
        Ok(())
    }
}