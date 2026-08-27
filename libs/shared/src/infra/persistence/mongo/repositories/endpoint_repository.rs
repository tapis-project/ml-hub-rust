use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

use crate::application::ports::endpoint::EndpointRepositoryError;
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::endpoint as entities;
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
    async fn list_by_target_urn(
        &self,
        tenant_id: &str,
        target_resource_urn: &str,
    ) -> Result<Vec<entities::Endpoint>, EndpointRepositoryError> {
        self.list(doc! {
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

    async fn save(&self, endpoint: &entities::Endpoint) -> Result<(), EndpointRepositoryError> {
        self.write_collection
            .insert_one(Endpoint::from(endpoint))
            .await
            .map_err(log_persistence_error)?;

        Ok(())
    }
}

impl EndpointRepository {
    async fn list(
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
