//! This module contains mappings of this request's structs to application-layer input structs

use crate::presentation::http::v1::requests::discover_models;
use crate::application::inputs::discover_models as inputs;
use crate::shared_kernel::enums::Task;
use crate::errors::Error;

impl TryFrom<&discover_models::DiscoveryCriterion> for inputs::SearchCriterion {
    type Error = Error;
    
    fn try_from(value: &discover_models::DiscoveryCriterion) -> Result<Self, Self::Error> {
        let mut task_types: Vec<Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(Task::from(task_type))
        }

        Ok(Self {
            name: value.name.clone(),
            author: value.author.clone(),
            libraries: value.libraries.clone(),
            model_type: value.model_type.clone(),
            tags: value.tags.clone(),
            task_types: Some(task_types),
            regulatory: value.regulatory.clone(),
            license: value.license.clone(),
        })
    }
}