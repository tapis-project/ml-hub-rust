use crate::{
    application::outputs::dataset::DatasetQueryOutput, domain::entities::dataset as entities,
    presentation::http::v1::responses::datasets as responses,
};

impl From<entities::Dataset> for responses::Dataset {
    fn from(value: entities::Dataset) -> Self {
        let (provider, huggingface_repo_locator, tapis_system_locator) = match value.provider() {
            entities::DatasetProvider::HuggingFace(v) => (
                responses::DatasetProvider::HuggingFace,
                Some(responses::HuggingFaceRepoLocator {
                    id: v.id().into(),
                    sha: v.sha().into(),
                }),
                None,
            ),
            entities::DatasetProvider::Tapis(v) => (
                responses::DatasetProvider::Tapis,
                None,
                Some(responses::TapisSystemLocator {
                    site_id: v.site_id().into(),
                    tenant_id: v.tenant_id().into(),
                    system_id: v.system_id().into(),
                    path: v.path().into(),
                }),
            ),
        };

        Self {
            id: *value.id(),
            tenant_id: value.tenant_id().into(),
            owner: value.owner().into(),
            tags: value.tags().iter().map(|v| v.as_str().to_owned()).collect(),
            provider,
            huggingface_repo_locator,
            tapis_system_locator,
            items: value
                .items()
                .iter()
                .map(|v| responses::DatasetItem {
                    path: v.path().into(),
                    size: v.size(),
                })
                .collect(),
            item_count: value.items().len() as u64,
            size: value.size(),
            visibility: value.visibility().clone().into(),
        }
    }
}

impl From<DatasetQueryOutput> for responses::Dataset {
    fn from(value: DatasetQueryOutput) -> Self {
        let (provider, huggingface_repo_locator, tapis_system_locator) = match value.provider {
            entities::DatasetProvider::HuggingFace(v) => (
                responses::DatasetProvider::HuggingFace,
                Some(responses::HuggingFaceRepoLocator {
                    id: v.id().into(),
                    sha: v.sha().into(),
                }),
                None,
            ),
            entities::DatasetProvider::Tapis(v) => (
                responses::DatasetProvider::Tapis,
                None,
                Some(responses::TapisSystemLocator {
                    site_id: v.site_id().into(),
                    tenant_id: v.tenant_id().into(),
                    system_id: v.system_id().into(),
                    path: v.path().into(),
                }),
            ),
        };

        Self {
            id: value.id,
            tenant_id: value.tenant_id,
            owner: value.owner,
            tags: value
                .tags
                .iter()
                .map(|tag| tag.as_str().to_owned())
                .collect(),
            provider,
            huggingface_repo_locator,
            tapis_system_locator,
            items: value
                .items
                .into_iter()
                .map(|item| responses::DatasetItem {
                    path: item.path().into(),
                    size: item.size(),
                })
                .collect(),
            item_count: value.item_count,
            size: value.size,
            visibility: value.visibility.into(),
        }
    }
}
