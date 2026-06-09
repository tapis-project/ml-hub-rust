pub use crate::domain::entities::timestamp::TimeStamp;
pub use mongodb::bson::DateTime;
use mongodb::{error::{Error, ErrorKind, WriteError, WriteFailure}, Database, IndexModel};
use serde::Serialize;

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