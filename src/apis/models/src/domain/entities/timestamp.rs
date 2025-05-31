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
}

impl From<TimeStamp> for String {
    fn from(value: TimeStamp) -> Self {
        value.inner.to_rfc3339()
    }
}