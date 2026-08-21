use crate::application::inputs::model_metadata as inputs;
use crate::domain::entities::model_metadata as domain;

use crate::application::errors::ApplicationError;
use crate::shared_kernel::context::RequestContext;

impl TryFrom<inputs::Locator> for domain::Locator {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::Locator) -> Result<Self, Self::Error> {
        Ok(Self {
            url: value.url  
        })
    }
}


impl TryFrom<inputs::Canonical> for domain::Canonical {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::Canonical) -> Result<Self, Self::Error> {
        Ok(Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: domain::Locator::try_from(value.locator)?,
            author: value.author,
            likes: value.likes,
            downloads: value.downloads,
            gated: value.gated,
            private: value.private,
            sha: value.sha,
        })
    }
}

impl TryFrom<(inputs::RegisterModelMetadataInput, &RequestContext)> for domain::ModelMetadata {
    type Error = ApplicationError;
    
    fn try_from(value: (inputs::RegisterModelMetadataInput, &RequestContext)) -> Result<Self, Self::Error> {        
        let canonical = value.0.canonical
            .map(|v| domain::Canonical::try_from(v))
            .transpose()?;
        
        Ok(Self {
            name: value.0.name,
            author: value.1.actor_principal_id().clone(),
            description: value.0.description,
            tenant_id: value.1.actor_tenant_id().clone(),
            artifact_id: None,
            canonical,
            libraries: value.0.libraries,
            model_type: value.0.model_type,
            tags: value.0.tags,
            task_types: value.0.task_types.clone(),
            regulatory: value.0.regulatory,
            license: value.0.license,
            deployment_strategy_refs: vec![] // Deploment strategies cannot be known a regsitration time
        })
    }
}