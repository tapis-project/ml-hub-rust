use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

use crate::application;
use crate::application::ports::agent_record::AgentRecordRepositoryError;
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities;
use crate::infra::persistence::mongo::database::AGENT_RECORD_COLLECTION;
use crate::infra::persistence::mongo::documents::agent_record::AgentRecord;

pub struct AgentRecordRepository {
    read_collection: Collection<AgentRecord>,
    write_collection: Collection<AgentRecord>,
}

impl AgentRecordRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let database = client.database(&db_name);

        Self {
            read_collection: database.collection(AGENT_RECORD_COLLECTION),
            write_collection: database.collection(AGENT_RECORD_COLLECTION),
        }
    }
}

#[async_trait]
impl application::ports::agent_record::AgentRecordRepository for AgentRecordRepository {
    async fn save(
        &self,
        agent_record: &entities::agent_record::AgentRecord,
    ) -> Result<(), AgentRecordRepositoryError> {
        let document = AgentRecord::from(agent_record);

        self.write_collection
            .insert_one(document)
            .await
            .map_err(|error| {
                let infrastructure_error = InfrastructureError::new_internal();
                log::error!(
                    "[{}] Persistence error: {}",
                    infrastructure_error.error_id(),
                    error
                );
                infrastructure_error
            })?;

        Ok(())
    }

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<entities::agent_record::AgentRecord>, AgentRecordRepositoryError> {
        self.list(doc! { "tenant_id": tenant_id, "owner": owner })
            .await
    }

    async fn list_by_tenant(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<entities::agent_record::AgentRecord>, AgentRecordRepositoryError> {
        self.list(doc! { "tenant_id": tenant_id }).await
    }
}

impl AgentRecordRepository {
    async fn list(
        &self,
        filter: mongodb::bson::Document,
    ) -> Result<Vec<entities::agent_record::AgentRecord>, AgentRecordRepositoryError> {
        let mut cursor = self.read_collection.find(filter).await.map_err(|error| {
            let infrastructure_error = InfrastructureError::new_internal();
            log::error!(
                "[{}] Persistence error: {}",
                infrastructure_error.error_id(),
                error
            );
            infrastructure_error
        })?;

        let mut agent_records = Vec::new();
        while let Some(document) = cursor.try_next().await.map_err(|error| {
            let infrastructure_error = InfrastructureError::new_internal();
            log::error!(
                "[{}] Persistence error: {}",
                infrastructure_error.error_id(),
                error
            );
            infrastructure_error
        })? {
            let agent_record =
                entities::agent_record::AgentRecord::try_from(document).map_err(|error| {
                    let infrastructure_error = InfrastructureError::new_internal();
                    log::error!(
                        "[{}] Conversion error: {}",
                        infrastructure_error.error_id(),
                        error
                    );
                    infrastructure_error
                })?;

            agent_records.push(agent_record);
        }

        Ok(agent_records)
    }
}
