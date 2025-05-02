use crate::infra::mongo::database::{
    INFERENCE_SERVER_COLLECTION,
    INFERENCE_SERVER_DEPLOYMENT_COLLECTION
};
use crate::application;
use crate::domain;
use crate::infra::mongo::entities::{InferenceServer, InferenceServerDeployment};
use shared::errors::Error;
use mongodb::{
    bson::doc,
    Cursor,
    Database,
    Collection,
};
use futures::stream::TryStreamExt;
use async_trait::async_trait;

pub struct InferenceServerRepository {
    collection: Collection<InferenceServer>
}

#[async_trait]
impl application::repositories::InferenceServerRepository for InferenceServerRepository {
    async fn save(&self, server: domain::entities::InferenceServer) -> Result<domain::entities::InferenceServer, Error> {
        let _server_entity = InferenceServer::try_from(server)
            .map_err(|err| Error::new(err.to_string()))?;
        let _ = self.collection.find(None, None).await.unwrap();
        Err(Error::from_str("Unimplemented"))
    }

    async fn list_all(&self) -> Result<Vec<domain::entities::InferenceServer>, Error> {
        let mut cursor: Cursor<InferenceServer> = self.collection.find(None, None)
            .await
            .map_err(|err| Error::new(err.to_string()))?;

        while let Some(_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
            // do something
        }
        Err(Error::from_str("Unimplemented"))
    }

    // async fn find_by_metadata_name_version(&self, name: String, version: String) -> Result<Option<domain::entities::InferenceServer>, Error> {
    //     let filter = doc! { 
    //         "metadata.name": name,
    //         "metadata.version": version
    //     };
    //     let mut cursor: Cursor<InferenceServer> = self.collection.find(filter, None)
    //         .await
    //         .map_err(|err| Error::new(err.to_string()))?;

    //     while let Some(_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
    //         // do something
    //     }
    //     Err(Error::from_str("Unimplemented"))
    // }
}

impl InferenceServerRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection(INFERENCE_SERVER_COLLECTION),
        }
    }
}

pub struct InferenceServerDeploymentRepository {
    collection: Collection<InferenceServerDeployment>
}

#[async_trait]
impl application::repositories::InferenceServerDeploymentRepository for InferenceServerDeploymentRepository {
    // async fn save(&self, _server: domain::entities::InferenceServerDeployment) -> Result<domain::entities::InferenceServerDeployment, Error> {
    //     // let _server_entity = InferenceServerDeployment::try_from(server)
    //     //     .map_err(|err| Error::new(err.to_string()))?;
    //     // let _ = self.collection.find(None, None).await.unwrap();
    //     Err(Error::from_str("Unimplemented"))
    // }

    // async fn find_by_inference_server(&self, _server: domain::entities::InferenceServer) -> Result<Option<shared::domain::inference::InferenceServer>, Error> {
    //     Err(Error::from_str("Unimplemented"))
    // }

    async fn find_by_metadata_name_version(&self, name: String, version: String) -> Result<Option<domain::entities::InferenceServerDeployment>, Error> {
        let filter = doc! { 
            "metadata.name": name,
            "metadata.version": version
        };
        let mut cursor: Cursor<InferenceServerDeployment> = self.collection.find(filter, None)
            .await
            .map_err(|err| Error::new(err.to_string()))?;

        while let Some(_doc) = cursor.try_next().await.map_err(|err| Error::new(err.to_string()))?  {
            // do something
        }
        Err(Error::from_str("Unimplemented"))
    }
}

impl InferenceServerDeploymentRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection(INFERENCE_SERVER_DEPLOYMENT_COLLECTION),
        }
    }
}