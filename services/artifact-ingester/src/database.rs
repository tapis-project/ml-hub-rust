pub use shared::infra::persistence::mongo::database::{initialize_client, ClientParams};

pub const ARTIFACT_COLLECTION: &str = "ARTIFACTS";
pub const ARTIFACT_INGESTION_COLLECTION: &str = "ARTIFACT_INGESTIONS";