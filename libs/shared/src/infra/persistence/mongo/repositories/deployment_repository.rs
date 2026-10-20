// Application
use crate::application;
use crate::application::inputs::deployment::FilterInput;
use crate::application::ports::deployment::ModelDeploymentRepositoryError;
use crate::application::ports::errors::InfrastructureError;

// Domain
use crate::domain::entities;
use crate::domain::entities::deployment::argument::Argument;
use crate::infra::_common::mongo::ToBsonDateTime;
use crate::infra::argument::mongo::{
    DeploymentArgumentsDocument, MongoDeploymentArgumentRepository,
};

// Infra
use crate::infra::persistence::mongo::database::{
    DEPLOYMENT_ARGUMENT_COLLECTION, MODEL_DEPLOYMENT_COLLECTION,
};
use crate::infra::persistence::mongo::documents::deployment::{ModelDeployment, State};
use crate::shared_kernel::value_objects::TimeStamp;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson, Uuid},
    Client, Collection,
};

pub struct ModelDeploymentRepository {
    client: Client,
    read_collection: Collection<ModelDeployment>,
    write_collection: Collection<ModelDeployment>,
    argument_collection: Collection<DeploymentArgumentsDocument>,
}

impl ModelDeploymentRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let db = client.database(&db_name);

        Self {
            client: client.clone(),
            write_collection: db.collection(MODEL_DEPLOYMENT_COLLECTION),
            read_collection: db.collection(MODEL_DEPLOYMENT_COLLECTION),
            argument_collection: db.collection(DEPLOYMENT_ARGUMENT_COLLECTION),
        }
    }

    async fn update_with_filter(
        &self,
        deployment: &entities::deployment::ModelDeployment,
        filter: mongodb::bson::Document,
    ) -> Result<(), ModelDeploymentRepositoryError> {
        let update = ModelDeployment::from(deployment);

        let document = doc! {
            "$set": {
                "state": String::from(update.state),
                "batch_execution_outcome": to_bson(&update.batch_execution_outcome)
                    .map_err(|error| map_error("Could not serialize batch_execution_outcome", error))?,
                "desired_state": String::from(update.desired_state),
                "last_message": update.last_message,
                "visibility": to_bson(&update.visibility)
                    .map_err(|error| map_error("Could not serialize visibility", error))?,
                "last_modified": update.last_modified,
                "last_state_change": update.last_state_change,
                "last_observed": to_bson(&update.last_observed)
                    .map_err(|error| map_error("Could not serialize last_observed", error))?,
                "maximum_runtime_seconds": to_bson(&update.maximum_runtime_seconds)
                    .map_err(|error| map_error("Could not serialize maximum_runtime_seconds", error))?,
                "expires_at": to_bson(&update.expires_at)
                    .map_err(|error| map_error("Could not serialize expires_at", error))?,
                "last_desired_state_change": update.last_desired_state_change,
                "target_endpoint": to_bson(&update.target_endpoint)
                    .map_err(|error| map_error("Could not serialize target_endpoint", error))?,
                "replicas": to_bson(&update.replicas)
                    .map_err(|error| map_error("Could not serialize replicas", error))?,
                "metadata": to_bson(&update.metadata)
                    .map_err(|error| map_error("Could not serialize metadata", error))?,
                "reconciliation_status": to_bson(&update.reconciliation_status)
                    .map_err(|error| map_error("Could not serialize reconciliation_status", error))?,
                "retries": update.retries,
                "retry_after": to_bson(&update.retry_after)
                    .map_err(|error| map_error("Could not serialize retry_after", error))?,
                "revision": update.revision,
            }
        };

        let result = self
            .write_collection
            .update_one(filter, document)
            .await
            .map_err(|error| map_error("Could not update model deployment", error))?;

        if result.matched_count == 0 {
            return Err(ModelDeploymentRepositoryError::RevisionConflict);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl application::ports::deployment::ModelDeploymentRepository for ModelDeploymentRepository {
    async fn find_by_id_and_tenant(
        &self,
        tenant_id: &str,
        deployment_id: &entities::deployment::ModelDeploymentId,
    ) -> Result<Option<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let document = self
            .read_collection
            .find_one(doc! {
                "tenant_id": tenant_id,
                "id": Uuid::from_bytes(*deployment_id.as_uuid().as_bytes()),
            })
            .await
            .map_err(|error| map_error("Could not find model deployment", error))?;

        document.map(map_document).transpose()
    }

    async fn find_by_ids_and_tenant(
        &self,
        tenant_id: &str,
        deployment_ids: &[entities::deployment::ModelDeploymentId],
    ) -> Result<Vec<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let deployment_ids = deployment_ids
            .iter()
            .map(|id| Uuid::from_bytes(*id.as_uuid().as_bytes()))
            .collect::<Vec<_>>();

        let mut cursor = self
            .read_collection
            .find(doc! {
                "tenant_id": tenant_id,
                "id": { "$in": deployment_ids },
            })
            .await
            .map_err(|error| map_error("Could not find model deployments", error))?;

        let mut deployments = Vec::new();

        while let Some(document) = cursor
            .try_next()
            .await
            .map_err(|error| map_error("Could not read model deployments", error))?
        {
            deployments.push(map_document(document)?);
        }

        Ok(deployments)
    }

    async fn save(
        &self,
        input: &entities::deployment::ModelDeployment,
    ) -> Result<(), ModelDeploymentRepositoryError> {
        let mut document = ModelDeployment::from(input);

        let result = self
            .write_collection
            .insert_one(&document)
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Persistence error: {}",
                    error.error_id(),
                    e.to_string()
                );
                error
            })?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn save_with_arguments(
        &self,
        deployment: &entities::deployment::ModelDeployment,
        arguments: &[Argument],
    ) -> Result<(), ModelDeploymentRepositoryError> {
        let deployment_document = ModelDeployment::from(deployment);

        let argument_document =
            MongoDeploymentArgumentRepository::document_from_domain(deployment.id(), arguments)
                .map_err(|error| map_error("Could not prepare deployment arguments", error))?;

        let argument_id = argument_document.deployment_id.clone();

        let deployment_collection = self.write_collection.clone();

        let argument_collection = self.argument_collection.clone();

        let mut session = self
            .client
            .start_session()
            .await
            .map_err(|error| map_error("Could not start deployment transaction", error))?;

        session
            .start_transaction()
            .and_run2(async move |session| {
                deployment_collection
                    .insert_one(deployment_document.clone())
                    .session(&mut *session)
                    .await?;

                argument_collection
                    .replace_one(doc! { "_id": &argument_id }, argument_document.clone())
                    .upsert(true)
                    .session(&mut *session)
                    .await?;

                Ok(())
            })
            .await
            .map_err(|error| map_error("Could not save deployment and arguments", error))?;

        Ok(())
    }

    async fn find_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let filter = doc! {
            "tenant_id": tenant_id,
            "owner": owner,
        };

        let mut results: Vec<entities::deployment::ModelDeployment> = vec![];

        let mut cursor = self.read_collection.find(filter).await.map_err(|e| {
            let error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Persistence error: {}",
                error.error_id(),
                e.to_string()
            );
            error
        })?;

        while let Some(entry) = cursor.try_next().await.map_err(|e| {
            let error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Persistence error: {}",
                error.error_id(),
                e.to_string()
            );
            error
        })? {
            results.push(map_document(entry)?);
        }

        Ok(results)
    }

    async fn update(
        &self,
        deployment: &entities::deployment::ModelDeployment,
    ) -> Result<(), ModelDeploymentRepositoryError> {
        let filter = doc! { "id": Uuid::from_bytes(*deployment.id().as_uuid().as_bytes()) };

        self.update_with_filter(deployment, filter).await
    }

    async fn update_at_revision(
        &self,
        deployment: &entities::deployment::ModelDeployment,
        expected_revision: u32,
    ) -> Result<(), ModelDeploymentRepositoryError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*deployment.id().as_uuid().as_bytes()),
            "revision": expected_revision,
        };

        self.update_with_filter(deployment, filter).await
    }

    async fn record_observation(
        &self,
        deployment_id: &entities::deployment::ModelDeploymentId,
        observed_at: &TimeStamp,
    ) -> Result<(), ModelDeploymentRepositoryError> {
        let filter = doc! {
            "id": Uuid::from_bytes(*deployment_id.as_uuid().as_bytes()),
        };

        let update = doc! {
            "$max": {
                "last_observed": observed_at.to_bson(),
            },
        };

        self.write_collection
            .update_one(filter, update)
            .await
            .map_err(|error| map_error("Could not record model deployment observation", error))?;

        Ok(())
    }

    async fn find(
        &self,
        input: &FilterInput,
    ) -> Result<Option<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let mut filter = doc! {};

        if let Some(id) = input.deployment_id {
            filter.insert("id", Uuid::from_bytes(*id.as_uuid().as_bytes()));
        }

        if let Some(state) = input.state.clone() {
            match bson::to_bson(&State::from(state)) {
                Ok(state) => {
                    filter.insert("state", state);
                }
                Err(e) => {
                    let error = InfrastructureError::new_internal();

                    log::error!(
                        "[{}] Persistence error: {}",
                        error.error_id(),
                        e.to_string()
                    );
                    return Err(ModelDeploymentRepositoryError::from(error));
                }
            }
        }

        let mut cursor = self.read_collection.find(filter).await.map_err(|e| {
            let error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Persistence error: {}",
                error.error_id(),
                e.to_string()
            );
            error
        })?;

        let maybe_model_deployment = cursor.try_next().await.map_err(|e| {
            let error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Persistence error: {}",
                error.error_id(),
                e.to_string()
            );
            error
        })?;

        match maybe_model_deployment {
            Some(document) => Ok(Some(map_document(document)?)),
            None => Ok(None),
        }
    }

    async fn find_reconcilable(
        &self,
    ) -> Result<Vec<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let now = mongodb::bson::DateTime::now();

        let filter = doc! {
            "$and": [
                { "$expr": { "$ne": ["$state", "$desired_state"] } },
                { "reconciliation_status": { "$ne": "Failed" } },
                {
                    "$nor": [
                        {
                            "deployment_modality": "batch",
                            "state": "Stopped",
                            "desired_state": "Running",
                            "batch_execution_outcome": { "$in": ["Succeeded", "TimedOut"] },
                        },
                    ],
                },
                {
                    "$or": [
                        { "expires_at": { "$exists": false } },
                        { "expires_at": null },
                        { "expires_at": { "$gt": now } },
                    ],
                },
            ],
        };

        let mut cursor =
            self.read_collection.find(filter).await.map_err(|error| {
                map_error("Could not find reconcilable model deployments", error)
            })?;

        let mut deployments = Vec::new();

        while let Some(document) = cursor
            .try_next()
            .await
            .map_err(|error| map_error("Could not read a reconcilable model deployment", error))?
        {
            deployments.push(map_document(document)?);
        }

        Ok(deployments)
    }

    async fn find_observable(
        &self,
    ) -> Result<Vec<entities::deployment::ModelDeployment>, ModelDeploymentRepositoryError> {
        let now = mongodb::bson::DateTime::now();

        let filter = doc! {
            "$and": [
                { "reconciliation_status": { "$ne": "Failed" } },
                {
                    "$or": [
                        { "expires_at": { "$exists": false } },
                        { "expires_at": null },
                        { "expires_at": { "$gt": now } },
                    ],
                },
                {
                    "$or": [
                        { "state": { "$in": ["Running", "Unknown", "Blocked"] } },
                        {
                            "reconciliation_status": {
                                "$in": ["Pending", "Starting", "Stopping", "Undeploying", "Retrying"],
                            },
                        },
                        { "$expr": { "$ne": ["$state", "$desired_state"] } },
                    ],
                },
            ],
        };

        let mut cursor = self
            .read_collection
            .find(filter)
            .await
            .map_err(|error| map_error("Could not find observable model deployments", error))?;

        let mut deployments = Vec::new();

        while let Some(document) = cursor
            .try_next()
            .await
            .map_err(|error| map_error("Could not read an observable model deployment", error))?
        {
            deployments.push(map_document(document)?);
        }

        Ok(deployments)
    }
}

fn map_error(context: &str, error: impl std::fmt::Display) -> ModelDeploymentRepositoryError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] {}: {}",
        infrastructure_error.error_id(),
        context,
        error
    );

    infrastructure_error.into()
}

fn map_document(
    document: ModelDeployment,
) -> Result<entities::deployment::ModelDeployment, ModelDeploymentRepositoryError> {
    entities::deployment::ModelDeployment::try_from(&document).map_err(|error| {
        let infrastructure_error = InfrastructureError::new_internal();

        log::error!(
            "[{}] ModelDeployment data integrity error: {}",
            infrastructure_error.error_id(),
            error
        );

        infrastructure_error.into()
    })
}
