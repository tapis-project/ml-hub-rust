use crate::{
    application::outputs::dataset::DatasetQueryOutput, domain::entities::dataset as entities,
    infra::persistence::mongo::documents::dataset as documents, shared_kernel::value_objects::Tags,
};

impl TryFrom<documents::DatasetQuery> for DatasetQueryOutput {
    type Error = entities::DatasetError;

    fn try_from(value: documents::DatasetQuery) -> Result<Self, Self::Error> {
        if value.name.is_empty() {
            return Err(entities::DatasetError::DataIntegrityError(
                "Dataset contains an empty name".into(),
            ));
        }

        let provider = match (
            value.provider,
            value.huggingface_repo_locator,
            value.tapis_system_locator,
        ) {
            (documents::DatasetProvider::HuggingFace, Some(locator), None) => {
                entities::DatasetProvider::HuggingFace(
                    entities::HuggingFaceRepoLocator::reconstitute(locator.id, locator.sha)
                        .map_err(data_integrity_error)?,
                )
            }
            (documents::DatasetProvider::Tapis, None, Some(locator)) => {
                entities::DatasetProvider::Tapis(
                    entities::TapisSystemLocator::reconstitute(
                        locator.site_id,
                        locator.tenant_id,
                        locator.system_id,
                        locator.path,
                    )
                    .map_err(data_integrity_error)?,
                )
            }
            _ => {
                return Err(entities::DatasetError::DataIntegrityError(
                    "Dataset provider does not match exactly one persisted locator".into(),
                ))
            }
        };

        let items = value
            .items
            .into_iter()
            .map(|item| {
                entities::DatasetItem::reconstitute(item.path, item.size)
                    .map_err(data_integrity_error)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let item_count = u64::try_from(value.item_count).map_err(|error| {
            entities::DatasetError::DataIntegrityError(format!(
                "Dataset contains an invalid item count: {error}"
            ))
        })?;

        if item_count < items.len() as u64 {
            return Err(entities::DatasetError::DataIntegrityError(
                "Dataset item count is smaller than its projected items".into(),
            ));
        }

        let tags = Tags::reconstitute(value.tags).map_err(data_integrity_error)?;

        Ok(Self {
            id: uuid::Uuid::from_bytes(value.id.bytes()),
            tenant_id: value.tenant_id,
            owner: value.owner,
            name: value.name,
            description: value.description,
            tags,
            provider,
            items,
            item_count,
            size: value.size,
            visibility: value.visibility.into(),
        })
    }
}

fn data_integrity_error(error: impl std::fmt::Display) -> entities::DatasetError {
    entities::DatasetError::DataIntegrityError(error.to_string())
}
