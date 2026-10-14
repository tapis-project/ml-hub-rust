use std::fmt;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Urn(String);

impl Urn {
    pub fn new(urn: String) -> Self {
        Self(urn)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse_network_resource(&self) -> Result<NetworkResourceReference, UrnParseError> {
        let segments: Vec<&str> = self.0.split(':').collect();

        if segments.len() != 6 || segments[..3] != ["urn", "mlhub", "v1"] {
            return Err(UrnParseError::Malformed(self.0.clone()));
        }

        let tenant_id = segments[3];
        if tenant_id.is_empty() {
            return Err(UrnParseError::Malformed(self.0.clone()));
        }

        let id = Uuid::parse_str(segments[5])
            .map_err(|_| UrnParseError::InvalidResourceId(segments[5].into()))?;

        match segments[4] {
            "agent" => Ok(NetworkResourceReference::Agent {
                tenant_id: tenant_id.into(),
                id,
            }),
            "deployment" => Ok(NetworkResourceReference::ModelDeployment {
                tenant_id: tenant_id.into(),
                id,
            }),
            resource_type => Err(UrnParseError::UnsupportedResourceType(resource_type.into())),
        }
    }
}

impl fmt::Display for Urn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkResourceReference {
    Agent { tenant_id: String, id: Uuid },
    ModelDeployment { tenant_id: String, id: Uuid },
}

impl NetworkResourceReference {
    pub fn tenant_id(&self) -> &str {
        match self {
            Self::Agent { tenant_id, .. } | Self::ModelDeployment { tenant_id, .. } => tenant_id,
        }
    }

    pub fn id(&self) -> &Uuid {
        match self {
            Self::Agent { id, .. } | Self::ModelDeployment { id, .. } => id,
        }
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum UrnParseError {
    #[error("Malformed MLHub URN: {0}")]
    Malformed(String),

    #[error("Unsupported network resource type: {0}")]
    UnsupportedResourceType(String),

    #[error("Invalid network resource id: {0}")]
    InvalidResourceId(String),
}

#[cfg(test)]
#[path = "urn.test.rs"]
mod urn_test;
