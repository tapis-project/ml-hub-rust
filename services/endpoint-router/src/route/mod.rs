use thiserror::Error;
use url::Url;
use uuid::Uuid;

use shared::domain::entities::endpoint::NetworkProtocol;

const ENDPOINT_PATH_PREFIX: &str = "/v3/mlhub/endpoints/";

#[derive(Clone, Debug)]
pub struct CachedRoute {
    pub endpoint_id: Uuid,
    pub owner: String,
    pub protocol: NetworkProtocol,
    pub target_url: Url,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndpointInvocation {
    pub slug: String,
    pub remaining_path_and_query: String,
}

impl EndpointInvocation {
    pub fn parse(path_and_query: &str) -> Result<Self, EndpointInvocationError> {
        let path_end = path_and_query.find('?').unwrap_or(path_and_query.len());
        let path = &path_and_query[..path_end];
        let query = &path_and_query[path_end..];
        let suffix = path
            .strip_prefix(ENDPOINT_PATH_PREFIX)
            .ok_or(EndpointInvocationError::InvalidPath)?;

        let mut segments = suffix.splitn(2, '/');
        let slug = segments.next().unwrap_or_default();

        if slug.is_empty() {
            return Err(EndpointInvocationError::InvalidPath);
        }

        let remaining = segments.next().unwrap_or_default();
        let remaining_path_and_query = format!("/{remaining}{query}");

        Ok(Self {
            slug: slug.into(),
            remaining_path_and_query,
        })
    }

    pub fn upstream_path(&self, base_url: &Url) -> String {
        let base_path = base_url.path().trim_end_matches('/');
        let request_path = self.remaining_path_and_query.as_str();

        if base_path.is_empty() {
            return request_path.into();
        }

        format!("{base_path}{request_path}")
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum EndpointInvocationError {
    #[error("Invalid endpoint proxy path")]
    InvalidPath,
}

#[cfg(test)]
#[path = "route.test.rs"]
mod route_test;
