use crate::infra::persistence::mongo::documents::model_metadata;
use crate::infra::persistence::mongo::documents::task as document_task;
use crate::domain::entities::model_metadata as entities;
use crate::errors::Error;
use crate::shared_kernel::context::RequestContext;

impl TryFrom<entities::Locator> for model_metadata::Locator {
    type Error = Error;
    
    fn try_from(value: entities::Locator) -> Result<Self, Self::Error> {
        Ok(Self {
            url: value.url  
        })
    }
}


impl TryFrom<entities::Canonical> for model_metadata::Canonical {
    type Error = Error;
    
    fn try_from(value: entities::Canonical) -> Result<Self, Self::Error> {
        Ok(Self {
            platform: value.platform.to_string(),
            model_id: value.model_id,
            locator: model_metadata::Locator::try_from(value.locator)?,
            author: value.author,
            likes: value.likes.map(|v| v as u64),
            downloads: value.downloads.map(|v| v as u64),
            gated: value.gated,
            private: value.private,
            sha: value.sha,
        })
    }
}

impl From<entities::DeploymentStrategyReference> for model_metadata::DeploymentStrategyReference {
    fn from(value: entities::DeploymentStrategyReference) -> Self {
        model_metadata::DeploymentStrategyReference {
            name: value.name,
            platform: value.platform,
        }
    }
}

impl TryFrom<(&entities::ModelMetadata, &RequestContext)> for model_metadata::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: (&entities::ModelMetadata, &RequestContext)) -> Result<Self, Self::Error> {

        let mut task_types: Vec<document_task::Task> = Vec::new();
        for task_type in value.0.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(document_task::Task::from(task_type))
        }

        let canonical = value.0.canonical
            .clone()
            .map(|v| model_metadata::Canonical::try_from(v))
            .transpose()?;

        let deployment_strategy_refs = value.0.deployment_strategy_refs
            .clone()
            .into_iter()
            .map(model_metadata::DeploymentStrategyReference::from)
            .collect::<Vec<model_metadata::DeploymentStrategyReference>>();

        let tenant_id = value.1.actor_tenant_id().clone();

        let author = value.1.actor_principal_id().clone();

        Ok(Self {
            _id: None,
            artifact_id: None,
            description: value.0.description.clone(),
            author,
            tenant_id,
            canonical,
            name: value.0.name.clone(),
            libraries: value.0.libraries.clone(),
            model_type: value.0.model_type.clone(),
            tags: value.0.tags.clone(),
            task_types: Some(task_types),
            regulatory: value.0.regulatory.clone(),
            license: value.0.license.clone(),
            deployment_strategy_refs,
        })
    }
}