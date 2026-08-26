use std::time::Duration;

use chrono::{DateTime, ParseError, Utc};
use semver::Version;
use serde::Serialize;
use thiserror::Error;
use base64::prelude::*;

pub const MAX_TAGS: usize = 16;
pub const MAX_TAG_LENGTH_BYTES: usize = 64;

#[derive(Debug, Clone, Error)]
pub enum TagError {
    #[error("Tag MUST not be empty")]
    Empty,

    #[error("Tag MUST not exceed {MAX_TAG_LENGTH_BYTES} bytes")]
    TooLong,
}

#[derive(Clone, Debug)]
pub struct Tag(String);

impl Tag {
    pub fn new(value: String) -> Result<Self, TagError> {
        if value.is_empty() {
            return Err(TagError::Empty);
        }

        if value.len() > MAX_TAG_LENGTH_BYTES {
            return Err(TagError::TooLong);
        }

        Ok(Self(value))
    }

    pub fn reconstitute(value: String) -> Result<Self, TagError> {
        Self::new(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Error)]
pub enum TagsError {
    #[error("A resource MUST not have more than {MAX_TAGS} tags")]
    TooMany,

    #[error(transparent)]
    Tag(#[from] TagError),
}

#[derive(Clone, Debug, Default)]
pub struct Tags(Vec<Tag>);

impl Tags {
    pub fn new(values: Vec<String>) -> Result<Self, TagsError> {
        if values.len() > MAX_TAGS {
            return Err(TagsError::TooMany);
        }

        values
            .into_iter()
            .map(Tag::new)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
            .map_err(Into::into)
    }

    pub fn reconstitute(values: Vec<String>) -> Result<Self, TagsError> {
        Self::new(values)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tag> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Error)]
pub enum SemanticVersionError {
    #[error("Value MUST be a valid semantic version")]
    Invalid,
}

#[derive(Clone, Debug)]
pub struct SemanticVersion(String);

impl SemanticVersion {
    pub fn new(value: String) -> Result<Self, SemanticVersionError> {
        Version::parse(&value).map_err(|_| SemanticVersionError::Invalid)?;

        Ok(Self(value))
    }

    pub fn reconstitute(value: String) -> Result<Self, SemanticVersionError> {
        Self::new(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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

#[cfg(test)]
#[path = "value_objects.test.rs"]
mod value_objects_test;
