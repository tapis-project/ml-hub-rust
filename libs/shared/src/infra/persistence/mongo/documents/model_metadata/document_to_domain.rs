use crate::infra::persistence::mongo::documents::model_metadata as infra;
use crate::domain::entities::model_metadata as domain;
use crate::shared_kernel::enums::Task;
use crate::errors::Error;
use platforms::Platform;
use uuid::Uuid;

impl TryFrom<infra::Locator> for domain::Locator {
    type Error = Error;
    
    fn try_from(value: infra::Locator) -> Result<Self, Self::Error> {
        Ok(Self {
            url: value.url  
        })
    }
}

impl TryFrom<infra::Canonical> for domain::Canonical {
    type Error = Error;
    
    fn try_from(value: infra::Canonical) -> Result<Self, Self::Error> {
        Ok(Self {
            platform: Platform::try_from(value.platform.to_string().as_str())
                .map_err(|err| Error::new(err.to_string()))?,
            model_id: value.model_id,
            locator: domain::Locator::try_from(value.locator)?,
            author: value.author,
            likes: value.likes.map(|v| v as u128),
            downloads: value.downloads.map(|v| v as u128),
            gated: value.gated,
            private: value.private,
            sha: value.sha,
        })
    }
}

impl From<infra::DeploymentStrategyReference> for domain::DeploymentStrategyReference {
    fn from(value: infra::DeploymentStrategyReference) -> Self {
        domain::DeploymentStrategyReference {
            name: value.name,
            platform: value.platform,
        }
    }
}

impl TryFrom<infra::ModelMetadata> for domain::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: infra::ModelMetadata) -> Result<Self, Self::Error> {
        let mut task_types: Vec<Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(Task::from(task_type))
        }

        let canonical = value.canonical
            .map(|v| domain::Canonical::try_from(v))
            .transpose()?;

        let deployment_strategy_refs = value.deployment_strategy_refs
            .clone()
            .into_iter()
            .map(domain::DeploymentStrategyReference::from)
            .collect::<Vec<domain::DeploymentStrategyReference>>();

        Ok(Self {
            name: value.name,
            description: value.description,
            tenant_id: value.tenant_id,
            canonical,
            artifact_id: value.artifact_id.and_then(|id| Some(Uuid::from_bytes(id.bytes()))),
            author: value.author,
            libraries: value.libraries,
            model_type: value.model_type,
            tags: value.tags,
            task_types: Some(task_types),
            regulatory: value.regulatory,
            license: value.license,
            deployment_strategy_refs,
        })
    }
}