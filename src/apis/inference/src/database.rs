use crate::errors::Error;
use mongodb::{bson::doc, Client, Collection};

pub struct ClientParams {
    host: String,
    port: String,
}

pub async fn create_client(params: ClientParams) -> Result<Client, Error> {
    Client::with_uri_str(format!("mongodb://{}:{}", params.host, params.port))
        .await
        .map_err(|err| Error::new(err.to_string()))
}