use crate::{
    application::inputs::dataset as inputs, domain::entities::dataset as entities,
    presentation::http::v1::requests::datasets as requests, shared_kernel::enums::Visibility,
};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum RegisterDatasetInputError {
    #[error("Dataset provider does not match exactly one supplied locator")]
    ProviderLocatorMismatch,
}

impl TryFrom<requests::RegisterDatasetBody> for inputs::RegisterDatasetInput {
    type Error = RegisterDatasetInputError;

    fn try_from(value: requests::RegisterDatasetBody) -> Result<Self, Self::Error> {
        let provider = match (
            value.provider,
            value.huggingface_repo_locator,
            value.tapis_system_locator,
        ) {
            (requests::DatasetProvider::HuggingFace, Some(v), None) => {
                inputs::DatasetProviderInput::HuggingFace(inputs::HuggingFaceRepoLocatorInput {
                    id: v.id,
                    sha: v.sha,
                })
            }
            (requests::DatasetProvider::Tapis, None, Some(v)) => {
                inputs::DatasetProviderInput::Tapis(inputs::TapisSystemLocatorInput {
                    site_id: v.site_id,
                    tenant_id: v.tenant_id,
                    system_id: v.system_id,
                    path: v.path,
                })
            }
            _ => return Err(RegisterDatasetInputError::ProviderLocatorMismatch),
        };

        Ok(Self {
            tags: value.tags,
            provider,
            items: value
                .items
                .into_iter()
                .map(|v| inputs::DatasetItemInput {
                    path: v.path,
                    size: v.size,
                })
                .collect(),
            size: value.size,
            visibility: value.visibility.into(),
        })
    }
}

impl From<requests::Visibility> for inputs::VisibilityInput {
    fn from(v: requests::Visibility) -> Self {
        match v {
            requests::Visibility::Public => Self::Public,
            requests::Visibility::Private => Self::Private,
        }
    }
}

impl From<inputs::VisibilityInput> for Visibility {
    fn from(v: inputs::VisibilityInput) -> Self {
        match v {
            inputs::VisibilityInput::Public => Self::Public,
            inputs::VisibilityInput::Private => Self::Private,
        }
    }
}

impl TryFrom<inputs::DatasetItemInput> for entities::DatasetItem {
    type Error = entities::DatasetItemError;

    fn try_from(v: inputs::DatasetItemInput) -> Result<Self, Self::Error> {
        Self::new(v.path, v.size)
    }
}

impl TryFrom<inputs::DatasetProviderInput> for entities::DatasetProvider {
    type Error = entities::DatasetLocatorError;

    fn try_from(v: inputs::DatasetProviderInput) -> Result<Self, Self::Error> {
        Ok(match v {
            inputs::DatasetProviderInput::HuggingFace(v) => {
                Self::HuggingFace(entities::HuggingFaceRepoLocator::new(v.id, v.sha)?)
            }
            inputs::DatasetProviderInput::Tapis(v) => Self::Tapis(
                entities::TapisSystemLocator::new(v.site_id, v.tenant_id, v.system_id, v.path)?,
            ),
        })
    }
}
