pub const ARTIFACT_CACHE_DIR_NAME: &'static str = "cache";
pub const ARTIFACT_INGEST_DIR_NAME: &'static str = "ingest";
pub const ARTIFACT_PUBLICATION_DIR_NAME: &'static str = "publication";

pub const ARTIFACT_INGESTION_QUEUE: &'static str = "queue.artifact.ingest";
pub const ARTIFACT_INGESTION_EXCHANGE: &'static str = "exchange.artifact.ingest";
pub const ARTIFACT_INGESTION_ROUTING_KEY: &'static str = "artifact.ingest.queue";

pub const ARTIFACT_PUBLICATION_QUEUE: &'static str = "queue.artifact.publish";
pub const ARTIFACT_PUBLICATION_EXCHANGE: &'static str = "exchange.artifact.publish";
pub const ARTIFACT_PUBLICATION_ROUTING_KEY: &'static str = "artifact.publish.queue";