use async_trait::async_trait;
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
    async fn get_by_target_url(
        &self,
        tenant_id: &str,
        target_url: &str,
    ) -> Result<Option<entities::Endpoint>, EndpointRepositoryError> {
        self.read_collection
            .find_one(doc! { "tenant_id": tenant_id, "target_url": target_url })
            .await
            .map_err(log_persistence_error)
            .map_err(EndpointRepositoryError::from)
            .map(|endpoint| endpoint.map(entities::Endpoint::from))
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

fn log_persistence_error(error: mongodb::error::Error) -> InfrastructureError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] Persistence error: {}",
        infrastructure_error.error_id(),
        error
    );

    infrastructure_error
}
