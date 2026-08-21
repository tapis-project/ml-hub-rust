// use crate::application::outputs::model_metadata as outputs;
// use crate::domain::entities::model_metadata as domain;
// use crate::application::errors::ApplicationError;
// use crate::shared_kernel::enums::Task;

// impl TryFrom<outputs::ModelMetadata> for domain::ModelMetadata {
//     type Error = ApplicationError;
    
//     fn try_from(value: domain::ModelMetadata) -> Result<Self, Self::Error> {
//         let mut task_types: Vec<Task> = Vec::new();
//         for task_type in value.task_types.unwrap_or(Vec::with_capacity(0)) {
//             task_types.push(task_type)
//         }
        
//         let canonical = value.canonical
//             .map(|v| { domain::Canonical::from(v) });
        
//         Ok(Self {
//             name: value.name,
//             author: value.author,
//             tenant_id: value.tenant_id,
//             artifact_id: value.artifact_id,
//             description: value.description,
//             canonical,
//             libraries: value.libraries,
//             model_type: value.model_type,
//             tags: value.tags,
//             task_types: Some(task_types),
//             regulatory: value.regulatory,
//             license: value.license,
//         })
//     }
// }