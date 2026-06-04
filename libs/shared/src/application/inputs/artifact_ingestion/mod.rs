use uuid::Uuid;

pub struct ListModelIngestionsInput {}

pub struct GetModelIngestionInput {
    pub ingestion_id: Uuid
}