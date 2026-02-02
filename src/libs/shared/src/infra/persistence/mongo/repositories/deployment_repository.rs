use crate::application::errors::ApplicationError;
use crate::infra::persistence::mongo::database::MODEL_DEPLOYMENT_COLLECTION;
use crate::infra::persistence::mongo::documents::deployment::ModelDeployment;
use crate::application;
use crate::domain::entities;
use mongodb::{
    Database,
    Collection,
};
use async_trait::async_trait;
// use futures::stream::TryStreamExt;

pub struct ModelDeploymentRepository {
    read_collection: Collection<ModelDeployment>,
    write_collection: Collection<ModelDeployment>
}

impl ModelDeploymentRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            write_collection: db.collection(MODEL_DEPLOYMENT_COLLECTION),
            read_collection: db.collection(MODEL_DEPLOYMENT_COLLECTION)
        }
    }
}


impl application::ports::deployment::ModelDeploymentRepository for ModelDeploymentRepository {
    async fn save(&self, input: &entities::deployment::ModelDeployment) -> Result<(), ApplicationError> {
        let mut document = ModelDeployment::from(input);

        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn update_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*deployment.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "state": deployment.state
            }
        };

        self.write_collection
            .update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
    }

    async fn update_desired_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*deployment.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "state": deployment.state
            }
        };

        self.write_collection
            .update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
    }

    async fn find_for_reconciliation(self, input: FindForReconciliationInput) -> Result<ModelDeployment, ApplicationError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*deployment.id.as_bytes())
        };
        
        let document = doc! {
            "$set": {
                "state": deployment.state
            }
        };

        self.write_collection
            .update_one(filter, document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
    }
}