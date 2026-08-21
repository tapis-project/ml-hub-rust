use platforms::Platform;

use crate::domain::entities::deployment_strategy::strategy::Strategy;
use crate::domain::entities::model_metadata as entities;
use crate::application::outputs::model_metadata as outputs;
use crate::errors::Error;

impl TryFrom<(&entities::ModelMetadata, &Vec<Strategy>)> for outputs::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: (&entities::ModelMetadata, &Vec<Strategy>)) -> Result<Self, Self::Error> {      
        let (model, strategies) = value;

        let canonical = model.canonical
            .clone()
            .map(|c| outputs::Canonical::from(c));

        // Hashmap for O(1) lookup
        let strategy_description_map: std::collections::HashMap<String, Option<String>> = strategies
            .iter()
            .map(|s| (format_strategy_description_key(&s.platform, &s.name), s.description.clone()))
            .collect();

        let deployment_strategy_refs: Vec<outputs::DeploymentStrategyReference> = model.deployment_strategy_refs
            .clone()
            .into_iter()
            .map(|r| outputs::DeploymentStrategyReference {
                name: r.name.clone(),
                platform: r.platform.clone(),
                description: strategy_description_map
                    .get(&format_strategy_description_key(&r.platform, &r.name))
                    .and_then(|k| k.clone()),
            })
            .collect();

        Ok(Self {
            name: model.name.clone(),
            author: model.author.clone(),
            artifact_id: model.artifact_id.clone(),
            description: model.description.clone(),
            canonical,
            libraries: model.libraries.clone(),
            model_type: model.model_type.clone(),
            tenant_id: model.tenant_id.clone(),
            tags: model.tags.clone(),
            task_types: model.task_types.clone(),
            regulatory: model.regulatory.clone(),
            license: model.license.clone(),
            deployment_strategy_refs,
        })
    }
}

impl From<entities::Canonical> for outputs::Canonical {
    fn from(value: entities::Canonical) -> Self {
        Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: outputs::Locator::from(value.locator),
            author: value.author,
            likes: value.likes,
            downloads: value.likes,
            gated: value.gated,
            private: value.private,
            sha: value.sha
        }
    }
}

impl From<entities::Locator> for outputs::Locator {
    fn from(value: entities::Locator) -> Self {
        Self { url: value.url }
    }
}

// Helper - Formatter
fn format_strategy_description_key(platform: &Platform, name: &String) -> String {
    format!("{}:{}", platform, name)
}