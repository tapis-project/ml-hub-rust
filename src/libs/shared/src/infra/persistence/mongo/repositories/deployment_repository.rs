use crate::application::{errors::ApplicationError, inputs::deployment::FilterInput};
use crate::infra::persistence::mongo::database::MODEL_DEPLOYMENT_COLLECTION;
use crate::infra::persistence::mongo::documents::deployment::{ModelDeployment, State};
use crate::application;
use crate::domain::entities;
use mongodb::{
    bson::{doc, Uuid},
    Database,
    Collection,
};
use futures::stream::TryStreamExt;

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

#[async_trait::async_trait]
impl application::ports::deployment::ModelDeploymentRepository for ModelDeploymentRepository {
    async fn save(&self, input: &entities::deployment::ModelDeployment) -> Result<(), ApplicationError> {
        let mut document = ModelDeployment::from(input);

        let result = self.write_collection.insert_one(&document, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    // async fn update_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError> {
    //     let filter = doc! {
    //         "id": Uuid::from_bytes(*deployment.id.as_bytes())
    //     };
        
    //     let document = doc! {
    //         "$set": {
    //             "state": deployment.state
    //         }
    //     };

    //     self.write_collection
    //         .update_one(filter, document, None)
    //         .await
    //         .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
    // }

    // async fn update_desired_state(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError> {
    //     let filter = doc! {
    //         "id": Uuid::from_bytes(*deployment.id.as_bytes())
    //     };
        
    //     let document = doc! {
    //         "$set": {
    //             "state": deployment.state
    //         }
    //     };

    //     self.write_collection
    //         .update_one(filter, document, None)
    //         .await
    //         .map_err(|err| ApplicationError::RepoError(err.to_string()))?;
    // }

    async fn find(&self, input: &FilterInput) -> Result<Option<entities::deployment::ModelDeployment>, ApplicationError> {
        let mut filter = doc! {};

        if let Some(id) = input.deployment_id {
            filter.insert("deployment_id", Uuid::from_bytes(*id.clone().as_bytes()));
        }

        if let Some(state) = input.state.clone() {
            match bson::to_bson(&State::from(state)) {
                Ok(state) => {
                    filter.insert("state", state);
                },
                Err(err) => return Err(ApplicationError::RepoError(err.to_string()))
            }
        }
        
        let mut cursor = self.read_collection.find(filter, None)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let maybe_model_deployment = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        match maybe_model_deployment {
            Some(m) => Ok(Some(entities::deployment::ModelDeployment::from(&m))),
            None => Ok(None)
        }
    }
}