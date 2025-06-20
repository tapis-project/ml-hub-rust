use crate::common::application::errors::ApplicationError;
use crate::common::infra::persistence::mongo::database::{
    ARTIFACT_COLLECTION,
    ARTIFACT_INGESTION_COLLECTION,
};
use crate::common::infra::persistence::mongo::documents::{Artifact, ArtifactIngestion, ArtifactIngestionStatus};
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

    // async fn delete_by_metadata_name_version(&self, name: String, version: String) -> Result<(), Error> {
    //     let filter = doc! { 
    //         "metadata.name": name,
    //         "metadata.version": version
    //     };
    //     let result = self.read_collection.delete_one(filter, None)
    //         .await
    //         .map_err(|err| Error::new(err.to_string()))?;

    //     if result.deleted_count < 1 {
    //         return Err(Error::new(String::from("No document delete with specified metadata name and/or version")))
    //     }

    //     Ok(())
    // }

    // async fn exists_by_metadata_name_version(&self, name: String, version: String) -> Result<bool, Error> {
    //     let filter = doc! { 
    //         "metadata.name": name,
    //         "metadata.version": version
    //     };
    //     let mut cursor = self.read_collection.find(filter, None)
    //         .await
    //         .map_err(|err| Error::new(err.to_string()))?;

    //     while let Some(_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
    //         return Ok(true)
    //     }
        
    //     return Ok(false)
    // }
}

// impl InferenceServerRepository {
//     pub fn new(db: Database) -> Self {
//         Self {
//             write_collection: db.collection(INFERENCE_SERVER_COLLECTION),
//             read_collection: db.collection(INFERENCE_SERVER_COLLECTION)
//         }
//     }
// }

// pub struct InferenceServerDeploymentRepository {
//     read_collection: Collection<InferenceServerDeployment>,
//     _write_collection: Collection<InferenceServerDeployment>
// }

// #[async_trait]
// impl application::repositories::InferenceServerDeploymentRepository for InferenceServerDeploymentRepository {
//     // async fn save(&self, _server: entities::InferenceServerDeployment) -> Result<entities::InferenceServerDeployment, Error> {
//     //     // let _server_entity = InferenceServerDeployment::try_from(server)
//     //     //     .map_err(|err| Error::new(err.to_string()))?;
//     //     // let _ = self.collection.find(None, None).await.unwrap();
//     //     Err(Error::from_str("Unimplemented"))
//     // }

//     // async fn find_by_inference_server(&self, _server: entities::InferenceServer) -> Result<Option<shared::domain::inference::InferenceServer>, Error> {
//     //     Err(Error::from_str("Unimplemented"))
//     // }

//     async fn find_by_labels(&self, key: String, value: String) -> Result<Option<entities::InferenceServerDeployment>, Error> {
//         let filter = doc! { 
//             format!("metadata.selectors.match_metadata.{}", key).as_str(): value,
//         };
        
//         let mut cursor = self.read_collection.find(filter, None)
//             .await
//             .map_err(|err| Error::new(err.to_string()))?;

//         while let Some(deployment_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
//             let deployment = entities::InferenceServerDeployment::try_from(deployment_doc)
//                 .map_err(|err| Error::new(err.to_string()))?;

//             return Ok(Some(deployment))
//         }

//         Ok(None)
//     }
// }

pub struct ArtifactIngestionRepository {
    read_collection: Collection<Artifact>,
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

    async fn update_status(&self, id: &uuid::Uuid, status: &entities::ArtifactIngestionStatus) -> Result<(), ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(id.as_bytes().clone())
        };
        
        let document = doc! {
            "$set": {
                "status": String::from(ArtifactIngestionStatus::from(status.clone()))
            }
        };

        self.write_collection.update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        Ok(())
    }
}