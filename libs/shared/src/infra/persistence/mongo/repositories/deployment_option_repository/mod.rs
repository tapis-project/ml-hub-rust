use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

use crate::{
    application::ports::{
        deployment_option::{
            DeploymentOptionRepository as DeploymentOptionRepositoryPort,
            DeploymentOptionRepositoryError,
        },
        errors::InfrastructureError,
    },
    domain::entities::deployment_option::DeploymentOption as DomainDeploymentOption,
    infra::persistence::mongo::{
        database::DEPLOYMENT_OPTION_COLLECTION, documents::deployment_option::DeploymentOption,
    },
    shared_kernel::identifiers::ExternalModelId,
};

pub struct DeploymentOptionRepository {
    client: Client,
    collection: Collection<DeploymentOption>,
}

impl DeploymentOptionRepository {
    pub fn new(client: &Client, database_name: String) -> Self {
        Self {
            client: client.clone(),
            collection: client
                .database(&database_name)
                .collection(DEPLOYMENT_OPTION_COLLECTION),
        }
    }
}

#[async_trait]
impl DeploymentOptionRepositoryPort for DeploymentOptionRepository {
    async fn find_by_external_model_id(
        &self,
        external_model_id: &ExternalModelId,
    ) -> Result<Vec<DomainDeploymentOption>, DeploymentOptionRepositoryError> {
        let external_model_id =
            mongodb::bson::Uuid::from_bytes(*external_model_id.as_uuid().as_bytes());

        let mut cursor = self
            .collection
            .find(doc! { "external_model_id": external_model_id })
            .await
            .map_err(map_error)?;

        let mut deployment_options = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            deployment_options.push(document.try_into().map_err(map_error)?);
        }

        Ok(deployment_options)
    }

    async fn replace_for_external_model(
        &self,
        external_model_id: &ExternalModelId,
        deployment_options: &[DomainDeploymentOption],
    ) -> Result<(), DeploymentOptionRepositoryError> {
        if deployment_options
            .iter()
            .any(|option| option.external_model_id() != external_model_id)
        {
            return Err(map_error(
                "Cannot persist DeploymentOptions belonging to another ExternalModel",
            ));
        }

        let filter = doc! {
            "external_model_id": mongodb::bson::Uuid::from_bytes(
                *external_model_id.as_uuid().as_bytes(),
            ),
        };
        let documents = deployment_options
            .iter()
            .map(DeploymentOption::from)
            .collect::<Vec<_>>();

        let collection = self.collection.clone();
        let mut session = self.client.start_session().await.map_err(map_error)?;

        session
            .start_transaction()
            .and_run2(async move |session| {
                collection
                    .delete_many(filter.clone())
                    .session(&mut *session)
                    .await?;

                if !documents.is_empty() {
                    collection
                        .insert_many(documents.clone())
                        .session(&mut *session)
                        .await?;
                }

                Ok(())
            })
            .await
            .map_err(map_error)?;

        Ok(())
    }
}

fn map_error(error: impl std::fmt::Display) -> DeploymentOptionRepositoryError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] DeploymentOption persistence error: {}",
        infrastructure_error.error_id(),
        error
    );

    infrastructure_error.into()
}
