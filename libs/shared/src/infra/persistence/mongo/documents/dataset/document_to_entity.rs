use crate::{
    domain::entities::dataset as entities,
    infra::persistence::mongo::documents::dataset as documents,
};

impl TryFrom<documents::Dataset> for entities::Dataset {
    type Error = entities::DatasetError;

    fn try_from(value: documents::Dataset) -> Result<Self, Self::Error> {
        let provider = match (
            value.provider,
            value.huggingface_repo_locator,
            value.tapis_system_locator,
        ) {
            (documents::DatasetProvider::HuggingFace, Some(v), None) => {
                entities::DatasetProvider::HuggingFace(
                    entities::HuggingFaceRepoLocator::reconstitute(v.id, v.sha)
                        .map_err(|e| entities::DatasetError::DataIntegrityError(e.to_string()))?,
                )
            }
            (documents::DatasetProvider::Tapis, None, Some(v)) => entities::DatasetProvider::Tapis(
                entities::TapisSystemLocator::reconstitute(
                    v.site_id,
                    v.tenant_id,
                    v.system_id,
                    v.path,
                )
                .map_err(|e| entities::DatasetError::DataIntegrityError(e.to_string()))?,
            ),
            _ => {
                return Err(entities::DatasetError::DataIntegrityError(
                    "Dataset provider does not match exactly one persisted locator".into(),
                ))
            }
        };

        let items = value
            .items
            .into_iter()
            .map(|v| {
                entities::DatasetItem::reconstitute(v.path, v.size)
                    .map_err(|e| entities::DatasetError::DataIntegrityError(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        entities::Dataset::reconstitute(entities::ReconstituteDatasetProps {
            id: uuid::Uuid::from_bytes(value.id.bytes()),
            tenant_id: value.tenant_id,
            owner: value.owner,
            tags: value.tags,
            provider,
            items,
            size: value.size,
            visibility: value.visibility.into(),
        })
    }
}
