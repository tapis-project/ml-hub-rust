use crate::domain::entities::{model_metadata::{Canonical, Locator, ModelMetadata}};
use crate::shared_kernel::enums::Task;


pub fn full_model_metadata() -> ModelMetadata {
    ModelMetadata {
        name: "foo".into(),
        description: Some("foo".into()),
        tenant_id: "foo".into(),
        artifact_id: Some(uuid::Uuid::now_v7()),
        canonical: Some(Canonical {
            model_id: String::from("test/model"),
            platform: platforms::Platform::HuggingFace,
            locator: Locator {
                url: String::from("someurl"),
            },
            author: Some(String::from("test")),
            likes: None,
            downloads: None,
            gated: Some(false),
            private: Some(true),
            sha: None,
        }),
        author: "bar".into(),
        model_type: Some("cnn".into()),
        libraries: Some(vec!["transformers".into(), "diffusers".into()]),
        tags: Some(vec!["text-generation".into(), "transformers".into()]),
        task_types: Some(vec![Task::ImageClassification]),
        regulatory: Some(vec!["HIPPA".into()]),
        license: Some("MIT".into()),
        deployment_strategy_refs: vec![]
    }
}