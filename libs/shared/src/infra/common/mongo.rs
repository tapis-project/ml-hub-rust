pub use crate::domain::entities::timestamp::TimeStamp;

pub use mongodb::bson::DateTime;

pub trait ToBsonDateTime {
    fn to_bson(&self) -> DateTime;
}

impl ToBsonDateTime for TimeStamp {
    fn to_bson(&self) -> DateTime {
        DateTime::from_chrono(self.into_inner())
    }
}

impl From<TimeStamp> for DateTime {
    fn from(value: TimeStamp) -> Self {
        DateTime::from_chrono(value.into_inner())
    }
}