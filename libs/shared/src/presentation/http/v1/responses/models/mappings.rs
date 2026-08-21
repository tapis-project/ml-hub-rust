use crate::application::outputs::model_metadata as output;
use crate::presentation::http::v1::responses::models as responses;
use crate::presentation::http::v1::responses::tasks::Task;
use crate::errors::Error;

impl TryFrom<&output::ModelMetadata> for responses::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: &output::ModelMetadata) -> Result<Self, Self::Error> {
        let mut task_types: Vec<Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(Task::from(task_type))
        }

        let canonical = value.canonical
            .clone()
            .map(|c| responses::Canonical::from(c));

        let deployment_strategy_refs: Vec<responses::DeploymentStrategyReference> = value.deployment_strategy_refs
            .clone()
            .into_iter()
            .map(|r| responses::DeploymentStrategyReference {
                name: r.name.clone(),
                platform: r.platform.clone(),
                description: r.description.clone(),
            })
            .collect();

        Ok(Self {
            name: value.name.clone(),
            author: value.author.clone(),
            description: value.description.clone(),
            canonical,
            libraries: value.libraries.clone(),
            model_type: value.model_type.clone(),
            tenant_id: value.tenant_id.clone(),
            tags: value.tags.clone(),
            task_types: Some(task_types),
            regulatory: value.regulatory.clone(),
            license: value.license.clone(),
            deployment_strategy_refs,
        })
    }
}

impl From<output::Canonical> for responses::Canonical {
    fn from(value: output::Canonical) -> Self {
        Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: responses::Locator::from(value.locator),
            author: value.author,
            likes: value.likes,
            downloads: value.likes,
            gated: value.gated,
            private: value.private,
            sha: value.sha
        }
    }
}

impl From<output::Locator> for responses::Locator {
    fn from(value: output::Locator) -> Self {
        Self { url: value.url }
    }
}