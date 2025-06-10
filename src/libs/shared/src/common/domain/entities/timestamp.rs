use chrono::{DateTime, Utc};

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