use std::time::Duration;

use serde::Serialize;

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