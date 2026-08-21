use platforms::Platform;

use crate::domain::entities::model_metadata as entities;
use crate::application::outputs::model_metadata as outputs;
use crate::errors::Error;

impl TryFrom<outputs::ModelMetadata> for entities::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: outputs::ModelMetadata) -> Result<Self, Self::Error> {      
        Ok(Self {
            name: value.name.clone(),
            author: value.author.clone(),
            artifact_id: value.artifact_id.clone(),
            description: value.description.clone(),
            canonical: value.canonical.map(|c| { entities::Canonical::from(c) }),
            libraries: value.libraries.clone(),
            model_type: value.model_type.clone(),
            tenant_id: value.tenant_id.clone(),
            tags: value.tags.clone(),
            task_types: value.task_types.clone(),
            regulatory: value.regulatory.clone(),
            license: value.license.clone(),
            deployment_strategy_refs: value.deployment_strategy_refs
                .iter()
                .map(|r| { entities::DeploymentStrategyReference::from(r.clone()) })
                .collect(),
        })
    }
}

impl From<outputs::DeploymentStrategyReference> for entities::DeploymentStrategyReference {
    fn from(value: outputs::DeploymentStrategyReference) -> Self {
        return Self {
            name: value.name,
            platform: value.platform,
        }
    }
}

impl From<outputs::Canonical> for entities::Canonical {
    fn from(value: outputs::Canonical) -> Self {
        Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: entities::Locator::from(value.locator),
            author: value.author,
            likes: value.likes,
            downloads: value.likes,
            gated: value.gated,
            private: value.private,
            sha: value.sha
        }
    }
}

impl From<outputs::Locator> for entities::Locator {
    fn from(value: outputs::Locator) -> Self {
        Self { url: value.url }
    }
}