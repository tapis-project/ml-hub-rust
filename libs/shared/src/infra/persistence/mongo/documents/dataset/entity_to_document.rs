use crate::{
    domain::entities::dataset as entities,
    infra::persistence::mongo::documents::dataset as documents,
};

impl From<&entities::Dataset> for documents::Dataset {
    fn from(value: &entities::Dataset) -> Self {
        let (provider, huggingface_repo_locator, tapis_system_locator) = match value.provider() {
            entities::DatasetProvider::HuggingFace(v) => (
                documents::DatasetProvider::HuggingFace,
                Some(documents::HuggingFaceRepoLocator {
                    id: v.id().into(),
                    sha: v.sha().into(),
                }),
                None,
            ),
            entities::DatasetProvider::Tapis(v) => (
                documents::DatasetProvider::Tapis,
                None,
                Some(documents::TapisSystemLocator {
                    site_id: v.site_id().into(),
                    tenant_id: v.tenant_id().into(),
                    system_id: v.system_id().into(),
                    path: v.path().into(),
                }),
            ),
        };

        Self {
            _id: None,
            id: mongodb::bson::Uuid::from_bytes(*value.id().as_bytes()),
            tenant_id: value.tenant_id().into(),
            owner: value.owner().into(),
            name: value.name().into(),
            description: value.description().map(Into::into),
            tags: value.tags().iter().map(|v| v.as_str().to_owned()).collect(),
            provider,
            huggingface_repo_locator,
            tapis_system_locator,
            items: value
                .items()
                .iter()
                .map(|v| documents::DatasetItem {
                    path: v.path().into(),
                    size: v.size(),
                })
                .collect(),
            size: value.size(),
            visibility: value.visibility().clone().into(),
        }
    }
}
