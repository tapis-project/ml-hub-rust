use std::time::Duration;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client, Collection,
};

use crate::{
    application::{
        inputs::deployment_option::ListDeploymentOptionsInput,
        ports::{
            deployment_option::{
                DeploymentOptionPage, DeploymentOptionRepository as DeploymentOptionRepositoryPort,
                DeploymentOptionRepositoryError,
            },
            errors::InfrastructureError,
        },
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

    async fn list_by_external_model_id(
        &self,
        external_model_id: &ExternalModelId,
        input: &ListDeploymentOptionsInput,
    ) -> Result<DeploymentOptionPage, DeploymentOptionRepositoryError> {
        let filter = list_filter(external_model_id, input)?;
        let count_filter = external_model_filter(external_model_id);
        let pipeline = list_pipeline(filter, input);

        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .map_err(map_error)?;

        let mut documents = Vec::with_capacity(usize::from(input.limit()) + 1);

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            let document =
                mongodb::bson::from_document::<DeploymentOption>(document).map_err(map_error)?;

            documents.push(document);
        }

        let (deployment_options, cursor) = documents_to_page(documents, input.limit())?;

        let count = if input.include_count() {
            Some(
                self.collection
                    .count_documents(count_filter)
                    .max_time(Duration::from_secs(2))
                    .await
                    .map_err(map_error)?,
            )
        } else {
            None
        };

        Ok(DeploymentOptionPage {
            deployment_options,
            count,
            cursor,
        })
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

fn external_model_filter(external_model_id: &ExternalModelId) -> Document {
    doc! {
        "external_model_id": mongodb::bson::Uuid::from_bytes(
            *external_model_id.as_uuid().as_bytes(),
        ),
    }
}

fn list_filter(
    external_model_id: &ExternalModelId,
    input: &ListDeploymentOptionsInput,
) -> Result<Document, DeploymentOptionRepositoryError> {
    let mut filter = external_model_filter(external_model_id);

    if let Some(cursor) = input.cursor() {
        let id = ObjectId::parse_str(cursor)
            .map_err(|_| DeploymentOptionRepositoryError::InvalidCursor)?;

        filter.insert("_id", doc! { "$gt": id });
    }

    Ok(filter)
}

fn list_pipeline(filter: Document, input: &ListDeploymentOptionsInput) -> Vec<Document> {
    vec![
        doc! { "$match": filter },
        doc! { "$sort": { "_id": 1 } },
        doc! { "$limit": i64::from(input.limit()) + 1 },
    ]
}

fn documents_to_page(
    documents: Vec<DeploymentOption>,
    limit: u16,
) -> Result<(Vec<DomainDeploymentOption>, Option<String>), DeploymentOptionRepositoryError> {
    let limit = usize::from(limit);
    let has_next_page = documents.len() > limit;
    let mut last_id = None;
    let mut deployment_options = Vec::with_capacity(documents.len().min(limit));

    for document in documents.into_iter().take(limit) {
        let document_id = document
            ._id
            .ok_or_else(|| map_error("DeploymentOption document is missing _id"))?;

        last_id = Some(document_id);

        deployment_options.push(document.try_into().map_err(map_error)?);
    }

    let cursor = if has_next_page {
        last_id.map(|id| id.to_hex())
    } else {
        None
    };

    Ok((deployment_options, cursor))
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

#[cfg(test)]
#[path = "deployment_option_repository.test.rs"]
mod deployment_option_repository_test;
