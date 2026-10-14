use std::fmt;

use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModelDeploymentId(Uuid);

impl ModelDeploymentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn reconstitute(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for ModelDeploymentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ModelDeploymentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
#[path = "model_deployment_id.test.rs"]
mod model_deployment_id_test;
