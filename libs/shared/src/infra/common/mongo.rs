pub use crate::domain::entities::timestamp::TimeStamp;

pub use mongodb::bson::DateTime;
use mongodb::{error::{ErrorKind, WriteError, WriteFailure}, IndexModel};
use serde::Serialize;

pub trait Index {
    const INDEX_NAME: &'static str;
    type Collection: Serialize;
    fn index() -> IndexModel;
    fn collection_name() -> &'static str;
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