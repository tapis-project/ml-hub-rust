use crate::application::errors::ApplicationError;
use crate::domain::entities::artifact::ArtifactType as ArtifactTypeEntity;
use crate::infra::persistence::mongo::database::ARTIFACT_COLLECTION;
use crate::infra::persistence::mongo::documents::artifact::{Artifact, ArtifactType, UpdateArtifactRequest, UpdateArtifactPathRequest};
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

    async fn list_all_by_artifact_type(&self, artifact_type: ArtifactTypeEntity) -> Result<Vec<entities::artifact::Artifact>, ApplicationError> {
        let filter = doc! {
            "artifact_type": String::from(ArtifactType::from(artifact_type))
        };
        
        let mut cursor = self.read_collection.find(filter, None)
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

    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<entities::artifact::Artifact>, ApplicationError> {
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