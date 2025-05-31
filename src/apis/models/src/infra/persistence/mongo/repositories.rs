use crate::{application::errors::ApplicationError, infra::persistence::mongo::database::ARTIFACTS_COLLECTION};
use crate::infra::persistence::mongo::documents::Artifact;
use crate::application;
use crate::domain::entities;
use shared::errors::Error;
use mongodb::{
    bson::doc,
    Database,
    Collection,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use crate::infra::persistence::errors::DatabaseError;

pub struct ArtifactRepository {
    read_collection: Collection<Artifact>,
    write_collection: Collection<Artifact>
}

#[async_trait]
impl application::ports::repositories::ArtifactRepository for ArtifactRepository {
    async fn save(&self, _artifact: entities::Artifact) -> Result<(), ApplicationError> {
        // let mut document = Artifact::try_from(artifact)
        //     .map_err(|err| Error::new(err.to_string()))?;
        
        // let result = self.write_collection.insert_one(&document, None)
        //     .await
        //     .map_err(|err| Error::new(err.to_string()))?;

        // document._id = result.inserted_id.as_object_id();

        // let artifact = entities::Artifact::try_from(document)
        //     .map_err(|err| Error::new(err.to_string()))?;

        Ok(())
    }

    // async fn list_all(&self) -> Result<Vec<entities::InferenceServer>, Error> {
    //     let mut cursor = self.read_collection.find(None, None)
    //         .await
    //         .map_err(|err| Error::new(err.to_string()))?;

    //     let mut inference_servers:Vec<entities::InferenceServer> = Vec::new();
    //     while let Some(server_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
    //         inference_servers.push(
    //             entities::InferenceServer::try_from(server_doc)
    //                 .map_err(|err| Error::new(err.to_string()))?
    //         );
    //     }
        
    //     Ok(inference_servers)
    // }

    // async fn find_by_metadata_name_version(&self, name: String, version: String) -> Result<Option<entities::InferenceServer>, Error> {
    //     let filter = doc! { 
    //         "metadata.name": name,
    //         "metadata.version": version
    //     };
    //     let mut cursor = self.read_collection.find(filter, None)
    //         .await
    //         .map_err(|err| Error::new(err.to_string()))?;

    //     while let Some(server_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
    //         let inference_server = entities::InferenceServer::try_from(server_doc)
    //             .map_err(|err| Error::new(err.to_string()))?;

    //         return Ok(Some(inference_server))
    //     }

    //     Ok(None)
    // }

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

// impl InferenceServerDeploymentRepository {
//     pub fn new(db: Database) -> Self {
//         Self {
//             read_collection: db.collection(INFERENCE_SERVER_DEPLOYMENT_COLLECTION),
//             _write_collection: db.collection(INFERENCE_SERVER_DEPLOYMENT_COLLECTION),
//         }
//     }
// }