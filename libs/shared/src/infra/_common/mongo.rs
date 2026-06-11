pub use crate::domain::entities::timestamp::TimeStamp;
use crate::errors::Error as GenericError;
pub use mongodb::bson::DateTime;
use mongodb::{error::{Error, ErrorKind, WriteError, WriteFailure}, Database, IndexModel};
use serde::Serialize;
use mongodb::{Client, options::ClientOptions};

pub struct ClientParams {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
    pub replica_set: Option<String>
}

pub async fn initialize_client(params: ClientParams) -> Result<Client, GenericError> {
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

    let options = ClientOptions::parse(uri)
        .await
        .map_err(|err| GenericError::new(err.to_string()))?;

    let client = Client::with_options(options)
        .map_err(|err| GenericError::new(err.to_string()))?;
    
    Ok(client)
}

#[async_trait::async_trait]
pub trait Index {
    const INDEX_NAME: &'static str;
    type Collection: Serialize;
    fn index() -> IndexModel;
    fn collection_name() -> &'static str;

    // Creates a collection and returns a reference to the database. If the collection already
    // exists, igrnore the error. All other errors will be returned
    async fn ensure_collection(db: &Database) -> Result<(), Error> {
        let error = match db.create_collection(Self::collection_name()).await {
            Ok(_) => return Ok(()),
            Err(err) => err,
        };

        let result = match error.kind.as_ref().clone() {
            ErrorKind::Command(cmd_err) => {
                match cmd_err.code {
                    48 => Ok(()),
                    _ => Err(error)
                }
            },
            _ => Err(error)
        };

        result
    }
}

pub trait ToBsonDateTime {
    fn to_bson(&self) -> DateTime;
}

impl ToBsonDateTime for TimeStamp {
    fn to_bson(&self) -> DateTime {
        DateTime::from_chrono(self.into_inner())
    }
}

pub trait ToTimeStamp {
    fn to_timestamp(&self) -> TimeStamp;
}

impl ToTimeStamp for DateTime {
    fn to_timestamp(&self) -> TimeStamp {
        TimeStamp::from(self.to_chrono())
    }
}

impl From<TimeStamp> for DateTime {
    fn from(value: TimeStamp) -> Self {
        DateTime::from_chrono(value.into_inner())
    }
}

pub fn is_duplicate_key_error(error: &mongodb::error::Error) -> bool {
    matches!(
        *error.kind,
        ErrorKind::Write(WriteFailure::WriteError(WriteError { code: 11000, .. }))
    )
}