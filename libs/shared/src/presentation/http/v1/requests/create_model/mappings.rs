use crate::application::inputs::model as inputs;
use crate::errors::Error;
use crate::presentation::http::v1::requests::create_model::body as requests;
use crate::shared_kernel::enums::Task;

impl TryFrom<requests::CreateModelBody> for inputs::RegisterModelInput {
    type Error = Error;

    fn try_from(value: requests::CreateModelBody) -> Result<Self, Self::Error> {
        let mut task_types: Vec<Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(Task::from(task_type))
        }

        Ok(Self {
            name: value.name,
            description: value.description,
            canonical: None,
            libraries: value.libraries,
            model_type: value.model_type,
            tags: value.tags,
            task_types: Some(task_types),
            regulatory: value.regulatory,
            license: value.license,
        })
    }
}
