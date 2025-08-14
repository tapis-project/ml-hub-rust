use crate::application::errors::ApplicationError;
use crate::infra::persistence::mongo::database::ARTIFACT_PUBLICATION_COLLECTION;
use crate::infra::persistence::mongo::documents::artifact_publication::{ArtifactPublication, UpdateArtifactPublicationStatusRequest};
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

pub struct ArtifactPublicationRepository {
    read_collection: Collection<ArtifactPublication>,
    write_collection: Collection<ArtifactPublication>
}

impl ArtifactPublicationRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            write_collection: db.collection(ARTIFACT_PUBLICATION_COLLECTION),
            read_collection: db.collection(ARTIFACT_PUBLICATION_COLLECTION)
        }
    }
}

#[async_trait]
impl application::ports::repositories::ArtifactPublicationRepository for ArtifactPublicationRepository {
    async fn save(&self, publication: &entities::artifact_publication::ArtifactPublication) -> Result<(), ApplicationError> {
        let mut document = ArtifactPublication::from(publication);
        
        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    // async fn update(&self, publication: &entities::artifact_publication::ArtifactPublication) -> Result<(), ApplicationError>  {
    //     let update = UpdateArtifactPublicationRequest::from(publication.clone());

    //     let filter = doc! {
    //         "id": Uuid::from_bytes(*publication.id.as_bytes())
    //     };
        
    //     let document = doc! {
    //         "$set": {
    //             "status": String::from(update.status),
    //             "last_modified": update.last_modified,
    //             "last_message": update.last_message,
    //             "webhook_url": update.webhook_url,
    //             "artifact_path": update.artifact_path,
    //         }
    //     };

    //     self.write_collection
    //         .update_one(filter, document, None)
    //         .await
    //         .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

    //     Ok(())
    // }

    async fn update_status(&self, publication: &entities::artifact_publication::ArtifactPublication) -> Result<(), ApplicationError> {
        let update = UpdateArtifactPublicationStatusRequest::from(publication);

        let filter = doc! {
            "id": Uuid::from_bytes(*publication.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "status": update.status.to_string(),
                "last_modified": update.last_modified,
                "last_message": update.last_message
            }
        };

        self.write_collection.update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        Ok(())
    }

    // async fn find_by_artifact_id(&self, artifact_id: uuid::Uuid) -> Result<Vec<entities::artifact_publication::ArtifactPublication>, ApplicationError> {
    //     let filter = doc! {
    //         "artifact_id": Uuid::from_bytes(*artifact_id.as_bytes()),
    //     };

    //     let mut cursor = self.read_collection.find(filter, None)
    //         .await
    //         .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

    //     let mut publications: Vec<entities::artifact_publication::ArtifactPublication> = Vec::new();
    //     while let Some(publication_doc) = cursor.try_next()
    //         .await
    //         .map_err(|err| ApplicationError::RepoError(err.to_string()))? 
    //     {
    //         let publication = entities::artifact_publication::ArtifactPublication::try_from(publication_doc)
    //                 .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

    //         publications.push(publication);
    //     }

    //     Ok(publications)
    // }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<entities::artifact_publication::ArtifactPublication>, ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*id.as_bytes()),
        };

        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        while let Some(publication_doc) = cursor.try_next()
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))? 
        {
            let publication = entities::artifact_publication::ArtifactPublication::try_from(&publication_doc)
                    .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

            return Ok(Some(publication))
        }

        Ok(None)
    }
}