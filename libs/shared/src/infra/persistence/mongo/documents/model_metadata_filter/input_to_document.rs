use crate::infra::persistence::mongo::documents::model_metadata_filter;
use crate::application::inputs::discover_models as inputs;
use crate::infra::persistence::mongo::documents::task as document_task;
use crate::errors::Error;

use mongodb::bson::doc;

impl TryFrom<(&inputs::SearchCriterion, &Vec<String>)> for model_metadata_filter::ModelMetadataFilter {
    type Error = Error;
    fn try_from(value: (&inputs::SearchCriterion, &Vec<String>)) -> Result<Self, Self::Error> {
        let search_criteria = value.0;
        let mut task_types: Vec<document_task::Task> = Vec::new();
        for task_type in search_criteria.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(document_task::Task::from(task_type))
        }

        let tenant_ids = value.1;
        let mut tenancy_selector = Some(
            doc! {
                "$in": tenant_ids
            }
        );

        // If no tenants are provided
        if tenant_ids.len() == 0 {
            tenancy_selector = None;
        }

        Ok(Self {
            name: search_criteria.name.clone(),
            author: search_criteria.author.clone(),
            tenant_id: tenancy_selector,
            libraries: search_criteria.libraries.clone(),
            model_type: search_criteria.model_type.clone(),
            tags: search_criteria.tags.clone(),
            task_types: Some(task_types),
            regulatory: search_criteria.regulatory.clone(),
            license: search_criteria.license.clone(),
        })
    }
}