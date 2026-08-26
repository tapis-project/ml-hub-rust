use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson},
    Client, Collection,
};

use crate::application::ports::agent::AgentRepositoryError;
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities;
use crate::infra::persistence::mongo::database::AGENT_COLLECTION;
use crate::infra::persistence::mongo::documents::agent::Agent;
use crate::infra::persistence::mongo::documents::visibility::Visibility as DocumentVisibility;

pub struct AgentRepository {
    read_collection: Collection<Agent>,
    write_collection: Collection<Agent>,
}

impl AgentRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let database = client.database(&db_name);
        Self {
            read_collection: database.collection(AGENT_COLLECTION),
            write_collection: database.collection(AGENT_COLLECTION),
        }
    }

    async fn list(
        &self,
        filter: mongodb::bson::Document,
    ) -> Result<Vec<entities::agent::Agent>, AgentRepositoryError> {
        let mut cursor = self
            .read_collection
            .find(filter)
            .await
            .map_err(log_persistence_error)?;

        let mut agents = Vec::new();
        while let Some(document) = cursor.try_next().await.map_err(log_persistence_error)? {
            agents.push(entities::agent::Agent::try_from(document).map_err(|error| {
                let infrastructure_error = InfrastructureError::new_internal();
                log::error!(
                    "[{}] Conversion error: {}",
                    infrastructure_error.error_id(),
                    error
                );
                infrastructure_error
            })?);
        }

        Ok(agents)
    }
}

#[async_trait]
impl crate::application::ports::agent::AgentRepository for AgentRepository {
    async fn save(&self, agent: &entities::agent::Agent) -> Result<(), AgentRepositoryError> {
        self.write_collection
            .insert_one(Agent::from(agent))
            .await
            .map_err(log_persistence_error)?;
        Ok(())
    }

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<entities::agent::Agent>, AgentRepositoryError> {
        self.list(doc! { "tenant_id": tenant_id, "owner": owner })
            .await
    }

    async fn list_shared_with_user(
        &self,
        tenant_id: &str,
        _owner: &str,
    ) -> Result<Vec<entities::agent::Agent>, AgentRepositoryError> {
        let visibility = to_bson(&DocumentVisibility::Public).map_err(|error| {
            let infrastructure_error = InfrastructureError::new_internal();
            log::error!(
                "[{}] Visibility serialization error: {}",
                infrastructure_error.error_id(),
                error
            );
            infrastructure_error
        })?;
        
        self.list(doc! { "tenant_id": tenant_id, "visibility": visibility })
            .await
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
