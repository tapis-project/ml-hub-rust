use crate::application::inputs::model_metadata as inputs;
use crate::domain::entities::model_metadata as domain;
use crate::shared_kernel::enums::Task;

use crate::application::errors::ApplicationError;

impl TryFrom<domain::Locator> for inputs::Locator {
    type Error = ApplicationError;
    
    fn try_from(value: domain::Locator) -> Result<Self, Self::Error> {
        Ok(Self {
            url: value.url  
        })
    }
}

impl TryFrom<domain::Canonical> for inputs::Canonical {
    type Error = ApplicationError;
    
    fn try_from(value: domain::Canonical) -> Result<Self, Self::Error> {
        Ok(Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: inputs::Locator::try_from(value.locator)?,
            author: value.author,
            likes: value.likes,
            downloads: value.downloads,
            gated: value.gated,
            private: value.private,
            sha: value.sha,
        })
    }
}

impl TryFrom<domain::ModelMetadata> for inputs::RegisterModelMetadataInput {
    type Error = ApplicationError;
    
    fn try_from(value: domain::ModelMetadata) -> Result<Self, Self::Error> {
        let mut task_types: Vec<Task> = Vec::new();
        for task_type in value.task_types.unwrap_or(Vec::with_capacity(0)) {
            task_types.push(task_type.clone())
        }

        let canonical = value.canonical
            .map(|v| inputs::Canonical::try_from(v))
            .transpose()?;
        
        Ok(Self {
            name: value.name,
            description: value.description,
            canonical,
            libraries: value.libraries,
            model_type: value.model_type,
            tags: value.tags,
            task_types: Some(task_types),
            regulatory: value.regulatory,
            license: value.license,
        })
    }
}