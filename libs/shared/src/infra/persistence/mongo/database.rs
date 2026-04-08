use crate::errors::Error;
use mongodb::{Client, options::ClientOptions};
use log::debug;

pub struct ClientParams {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
    pub replica_set: Option<String>
}

pub async fn initialize_client(params: ClientParams) -> Result<Client, Error> {
    let replica_set = params.replica_set
        .map(|rs| format!("&replicaSet={}", rs))
        .unwrap_or("".into());

    let uri = format!(
        "mongodb://{}:{}@{}:{}/{}?authSource=admin{}",
        params.username,
        params.password,
        params.host,
        params.port,
        params.db,
        replica_set,
    );

    debug!("{}", uri.clone());

    let options = ClientOptions::parse(uri)
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    debug!("{:#?}", options);

    let client = Client::with_options(options)
        .map_err(|err| Error::new(err.to_string()))?;
    
    Ok(client)
}

pub const ARTIFACT_COLLECTION: &str = "ARTIFACTS";
pub const ARTIFACT_INGESTION_COLLECTION: &str = "ARTIFACT_INGESTIONS";
pub const MODEL_METADATA_COLLECTION: &str = "MODEL_METADATA";
pub const ARTIFACT_PUBLICATION_COLLECTION: &str = "ARTIFACT_PUBLICATIONS";
pub const MODEL_DEPLOYMENT_COLLECTION: &str = "MODEL_DEPLOYMENTS";