use chrono::{DateTime, Utc, ParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeStampError {
    #[error("{0}")]
    ParseError(#[from] ParseError)
}


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TimeStamp {
    inner: DateTime<Utc>
}

impl TimeStamp {
    pub fn now() -> Self {
        Self {
            inner: Utc::now()
        }
    }

    pub fn parse_string(s: &str) -> Result<Self, TimeStampError> {
        let inner = DateTime::parse_from_rfc3339(s)?
            .with_timezone(&Utc);

        Ok(Self {
            inner
        })
    }

    pub fn into_inner(&self) -> DateTime<Utc>{
        return self.inner
    }
}

impl From<DateTime<Utc>> for TimeStamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self {
            inner: value
        }
    }
}

impl From<TimeStamp> for String {
    fn from(value: TimeStamp) -> Self {
        value.inner.to_rfc3339()
    }
}