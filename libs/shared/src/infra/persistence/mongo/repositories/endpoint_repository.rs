use async_trait::async_trait;
use futures::stream::TryStreamExt;
use std::time::Duration;

use mongodb::{
    bson::{doc, oid::ObjectId, Uuid},
    Client, Collection,
};
use uuid::Uuid as DomainUuid;

use crate::application::inputs::endpoint::ListEndpointsInput;
use crate::application::outputs::endpoint::EndpointListOutput;
use crate::application::ports::endpoint::EndpointRepositoryError;
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::endpoint as entities;
use crate::infra::_common::mongo::is_duplicate_key_error;
use crate::infra::persistence::mongo::database::ENDPOINT_COLLECTION;
use crate::infra::persistence::mongo::documents::endpoint::Endpoint;

pub struct EndpointRepository {
    read_collection: Collection<Endpoint>,
    write_collection: Collection<Endpoint>,
}

impl EndpointRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let database = client.database(&db_name);

        Self {
            read_collection: database.collection(ENDPOINT_COLLECTION),
            write_collection: database.collection(ENDPOINT_COLLECTION),
        }
    }
}

#[async_trait]
impl crate::application::ports::endpoint::EndpointRepository for EndpointRepository {
    async fn list(
        &self,
        input: &ListEndpointsInput,
    ) -> Result<EndpointListOutput, EndpointRepositoryError> {
        let mut filter = doc! {
            "tenant_id": input.tenant_id(),
            "owner": input.owner(),
        };

        if let Some(target_resource_urn) = input.target_resource_urn() {
            filter.insert("target_resource_urn", target_resource_urn.as_str());
        }

        let count_filter = filter.clone();

        if let Some(cursor) = input.cursor() {
            let id =
                ObjectId::parse_str(cursor).map_err(|_| EndpointRepositoryError::InvalidCursor)?;

            filter.insert("_id", doc! { "$gt": id });
        }

        let mut cursor = self
            .read_collection
            .find(filter)
            .sort(doc! { "_id": 1 })
            .limit(i64::from(input.limit()) + 1)
            .await
            .map_err(log_persistence_error)?;

        let mut documents = Vec::with_capacity(usize::from(input.limit()) + 1);

        while let Some(document) = cursor.try_next().await.map_err(log_persistence_error)? {
            documents.push(document);
        }

        let has_next_page = documents.len() > usize::from(input.limit());
        documents.truncate(usize::from(input.limit()));

        let next_cursor = if has_next_page {
            documents
                .last()
                .and_then(|document| document._id)
                .map(|id| id.to_hex())
        } else {
            None
        };

        let count = if input.include_count() {
            Some(
                self.read_collection
                    .count_documents(count_filter)
                    .max_time(Duration::from_secs(2))
                    .await
                    .map_err(log_persistence_error)?,
            )
        } else {
            None
        };

        Ok(EndpointListOutput {
            endpoints: documents
                .into_iter()
                .map(entities::Endpoint::from)
                .collect(),
            cursor: next_cursor,
            count,
        })
    }

    async fn get_by_id(
        &self,
        tenant_id: &str,
        owner: &str,
        id: &DomainUuid,
    ) -> Result<Option<entities::Endpoint>, EndpointRepositoryError> {
        let id = Uuid::from_bytes(*id.as_bytes());

        self.read_collection
            .find_one(doc! { "tenant_id": tenant_id, "owner": owner, "id": id })
            .await
            .map_err(log_persistence_error)
            .map_err(EndpointRepositoryError::from)
            .map(|endpoint| endpoint.map(entities::Endpoint::from))
    }

    async fn list_by_target_urn(
        &self,
        tenant_id: &str,
        target_resource_urn: &str,
    ) -> Result<Vec<entities::Endpoint>, EndpointRepositoryError> {
        self.list_documents(doc! {
            "tenant_id": tenant_id,
            "target_resource_urn": target_resource_urn,
        })
        .await
    }

    async fn get_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<entities::Endpoint>, EndpointRepositoryError> {
        self.read_collection
            .find_one(doc! { "slug": slug })
            .await
            .map_err(log_persistence_error)
            .map_err(EndpointRepositoryError::from)
            .map(|endpoint| endpoint.map(entities::Endpoint::from))
    }

    async fn get_by_tenant_and_slug(
        &self,
        tenant_id: &str,
        slug: &str,
    ) -> Result<Option<entities::Endpoint>, EndpointRepositoryError> {
        self.read_collection
            .find_one(doc! { "tenant_id": tenant_id, "slug": slug })
            .await
            .map_err(log_persistence_error)
            .map_err(EndpointRepositoryError::from)
            .map(|endpoint| endpoint.map(entities::Endpoint::from))
    }

    async fn get_by_target(
        &self,
        tenant_id: &str,
        target_resource_urn: &str,
        target_name: &str,
    ) -> Result<Option<entities::Endpoint>, EndpointRepositoryError> {
        self.read_collection
            .find_one(doc! {
                "tenant_id": tenant_id,
                "target_resource_urn": target_resource_urn,
                "target_name": target_name,
            })
            .await
            .map_err(log_persistence_error)
            .map_err(EndpointRepositoryError::from)
            .map(|endpoint| endpoint.map(entities::Endpoint::from))
    }

    async fn save(&self, endpoint: &entities::Endpoint) -> Result<(), EndpointRepositoryError> {
        let result = self
            .write_collection
            .insert_one(Endpoint::from(endpoint))
            .await;

        if let Err(error) = result {
            if is_duplicate_key_error(&error) {
                return Err(EndpointRepositoryError::AlreadyExists);
            }

            return Err(log_persistence_error(error).into());
        }

        Ok(())
    }
}

impl EndpointRepository {
    async fn list_documents(
        &self,
        filter: mongodb::bson::Document,
    ) -> Result<Vec<entities::Endpoint>, EndpointRepositoryError> {
        let mut cursor = self
            .read_collection
            .find(filter)
            .await
            .map_err(log_persistence_error)?;

        let mut endpoints = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(log_persistence_error)? {
            endpoints.push(entities::Endpoint::from(document));
        }

        Ok(endpoints)
    }
}

fn log_persistence_error(error: mongodb::error::Error) -> InfrastructureError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] Persistence error: {}",
        infrastructure_error.error_id(),
        error
    );

    infrastructure_error
}
