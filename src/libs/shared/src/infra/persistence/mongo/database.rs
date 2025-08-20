use crate::errors::Error;
use mongodb::{Client, options::ClientOptions};
use mongodb::Database;

pub struct ClientParams {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
}

pub async fn get_db(params: ClientParams) -> Result<Database, Error> {
    let uri = format!(
        "mongodb://{}:{}@{}:{}/{}?authSource=admin",
        params.username,
        params.password,
        params.host,
        params.port,
        params.db,
    );

    let options = ClientOptions::parse(uri)
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    let client = Client::with_options(options)
        .map_err(|err| Error::new(err.to_string()))?;
    
    Ok(client.database(&params.db))
}

pub const ARTIFACT_COLLECTION: &str = "ARTIFACTS";
pub const ARTIFACT_INGESTION_COLLECTION: &str = "ARTIFACT_INGESTIONS";
pub const MODEL_METADATA_COLLECTION: &str = "MODEL_METADATA";
pub const ARTIFACT_PUBLICATION_COLLECTION: &str = "ARTIFACT_PUBLICATIONS";