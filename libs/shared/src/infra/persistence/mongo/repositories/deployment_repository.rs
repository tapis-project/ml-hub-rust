// Application
use crate::application::ports::deployment::ModelDeploymentRepositoryError;
use crate::application::ports::errors::CommonRepositoryError;
use crate::application::inputs::deployment::FilterInput;
use crate::application;

// Domain
use crate::domain::entities;

// Infra
use crate::infra::persistence::mongo::database::MODEL_DEPLOYMENT_COLLECTION;
use crate::infra::persistence::mongo::documents::deployment::{ModelDeployment, State};

use mongodb::{
    bson::{doc, Uuid, to_bson},
    Client,
    Collection,
};
use futures::stream::TryStreamExt;

pub struct ModelDeploymentRepository {
    read_collection: Collection<ModelDeployment>,
    write_collection: Collection<ModelDeployment>
}

impl ModelDeploymentRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let db = client.database(&db_name);
        
        Self {
            write_collection: db.collection(MODEL_DEPLOYMENT_COLLECTION),
            read_collection: db.collection(MODEL_DEPLOYMENT_COLLECTION)
        }
    }
}

#[async_trait::async_trait]
impl application::ports::deployment::ModelDeploymentRepository for ModelDeploymentRepository {
    async fn save(&self, input: &entities::deployment::ModelDeployment) -> Result<(), ModelDeploymentRepositoryError> {
        let mut document = ModelDeployment::from(input);

        let result = self.write_collection.insert_one(&document)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn find_by_owner(&self, tenant_id: &str, owner: &str) -> Result<Vec<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let filter = doc! {
            "tenant_id": tenant_id,
            "owner": owner,
        };

        let mut results: Vec<entities::deployment::ModelDeployment> = vec![];

        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

            while let Some(entry) = cursor.try_next().await.map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })? {
                results.push(entities::deployment::ModelDeployment::from(&entry));
            }

        Ok(results)
    }

    async fn update(&self, deployment: &entities::deployment::ModelDeployment) -> Result<(), ModelDeploymentRepositoryError>  {
        let filter = doc! {
            "id": Uuid::from_bytes(*deployment.id.as_bytes())
        };
        
        let update = ModelDeployment::from(&deployment.clone());

        let document = doc! {
            "$set": {
                "platform": update.platform.to_string(),
                "state": String::from(update.state),
                "desired_state": String::from(update.desired_state),
                "last_message": update.last_message,
                "visibility": to_bson(&update.visibility)
                    .map_err(|e| {
                        let error = CommonRepositoryError::new_internal();
                        log::error!("[{}] Conversion Error: {}", error.error_id(), e.to_string());
                        error
                    })?,
                "last_modified": update.last_modified,
                "last_state_change": update.last_state_change,
                "last_desired_state_change": update.last_desired_state_change,
                "deployment_interface": to_bson(&update.deployment_interface)
                    .map_err(|e| {
                        let error = CommonRepositoryError::new_internal();
                        log::error!("[{}] Conversion Error: {}", error.error_id(), e.to_string());
                        error
                    })?,
                "replicas": to_bson(&update.replicas)
                    .map_err(|e| {
                        let error = CommonRepositoryError::new_internal();
                        log::error!("[{}] Conversion Error: {}", error.error_id(), e.to_string());
                        error
                    })?,
                "metadata": to_bson(&update.metadata)
                    .map_err(|e| {
                        let error = CommonRepositoryError::new_internal();
                        log::error!("[{}] Conversion Error: {}", error.error_id(), e.to_string());
                        error
                    })?,
                "revision": update.revision,
            }
        };

        self.write_collection
            .update_one(filter, document)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        Ok(())
    }

    async fn find(&self, input: &FilterInput) -> Result<Option<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let mut filter = doc! {};

        if let Some(id) = input.deployment_id {
            filter.insert("id", Uuid::from_bytes(*id.clone().as_bytes()));
        }

        if let Some(state) = input.state.clone() {
            match bson::to_bson(&State::from(state)) {
                Ok(state) => {
                    filter.insert("state", state);
                },
                Err(e) => {
                    let error = CommonRepositoryError::new_internal();
                    log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                    return Err(ModelDeploymentRepositoryError::from(error))
                }
            }
        }
        
        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

        let maybe_model_deployment = cursor.try_next()
            .await
            .map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;
        
        match maybe_model_deployment {
            Some(m) => Ok(Some(entities::deployment::ModelDeployment::from(&m))),
            None => Ok(None)
        }
    }
}