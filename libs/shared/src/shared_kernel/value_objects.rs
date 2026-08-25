use std::time::Duration;

use chrono::{DateTime, ParseError, Utc};
use serde::Serialize;
use thiserror::Error;
use base64::prelude::*;

#[derive(Error, Debug)]
pub enum TimeStampError {
    #[error("{0}")]
    ParseError(#[from] ParseError),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TimeStamp {
    inner: DateTime<Utc>,
}

impl TimeStamp {
    pub fn now() -> Self {
        Self { inner: Utc::now() }
    }

    pub fn parse_string(s: &str) -> Result<Self, TimeStampError> {
        let inner = DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc);

        Ok(Self { inner })
    }

    pub fn into_inner(&self) -> DateTime<Utc> {
        self.inner
    }
}

impl From<DateTime<Utc>> for TimeStamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self { inner: value }
    }
}

impl From<TimeStamp> for String {
    fn from(value: TimeStamp) -> Self {
        value.inner.to_rfc3339()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Ttl(Duration);

impl Ttl {
    pub fn from_minutes(minutes: u64) -> Self {
        Self(Duration::from_secs(minutes * 60))
    }

    pub fn as_minutes(&self) -> u64 {
        self.0.as_secs() * 60
    }
}

#[derive(Debug, Clone, Error)]
pub enum Base64EncodedStringError {
    #[error("{0}")]
    InvalidBase64Encoding(String)
}

#[derive(Debug, Clone)]
pub struct Base64EncodedString(String);

impl Base64EncodedString {
    pub fn encode(payload: &Vec<u8>) -> Self {
        Self(BASE64_STANDARD.encode(&payload))
    }

    pub fn decode(&self) -> Result<Vec<u8>, Base64EncodedStringError> {
        BASE64_STANDARD.decode(&self.0)
            .map_err(|e| Base64EncodedStringError::InvalidBase64Encoding(e.to_string()))
    }

    pub fn new_from_base64(payload: String) -> Result<Self, Base64EncodedStringError> {
        if BASE64_STANDARD.decode(&payload).is_err() {
            return Err(Base64EncodedStringError::InvalidBase64Encoding("Expected base64".into()));
        }

        Ok(Self(payload))
    }

    pub fn into_inner(&self) -> &str {
        &self.0
    }
}
