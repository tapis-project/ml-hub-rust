use std::collections::HashSet;

use serde::Deserialize;
use shared::{
    application::inputs::dataset::{
        DatasetItemInput, DatasetProviderInput, HuggingFaceRepoLocatorInput, RegisterDatasetInput,
        VisibilityInput,
    },
    shared_kernel::value_objects::{MAX_TAGS, MAX_TAG_LENGTH_BYTES},
};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct HuggingFaceDatasetRecord {
    pub id: String,
    pub sha: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub gated: bool,
    #[serde(default)]
    pub siblings: Vec<HuggingFaceRepoSibling>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HuggingFaceRepoSibling {
    pub rfilename: String,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum TransformDatasetError {
    #[error("Hugging Face dataset ID does not include a repository name: {0}")]
    MalformedDatasetId(String),

    #[error("Private Hugging Face datasets are not eligible for the global catalog")]
    Private,

    #[error("Gated Hugging Face datasets are not eligible for the global catalog")]
    Gated,

    #[error("Hugging Face dataset item {0} does not include its size")]
    MissingItemSize(String),

    #[error("Hugging Face dataset item sizes exceed the supported u64 total")]
    ItemSizeOverflow,
}

impl TryFrom<HuggingFaceDatasetRecord> for RegisterDatasetInput {
    type Error = TransformDatasetError;

    fn try_from(value: HuggingFaceDatasetRecord) -> Result<Self, Self::Error> {
        if value.private {
            return Err(TransformDatasetError::Private);
        }

        if value.gated {
            return Err(TransformDatasetError::Gated);
        }

        let name = value
            .id
            .rsplit_once('/')
            .map(|(_, name)| name)
            .filter(|name| !name.is_empty())
            .ok_or_else(|| TransformDatasetError::MalformedDatasetId(value.id.clone()))?
            .to_owned();

        let mut size = 0_u64;
        let mut items = Vec::with_capacity(value.siblings.len());

        for sibling in value.siblings {
            let item_size = sibling
                .size
                .ok_or_else(|| TransformDatasetError::MissingItemSize(sibling.rfilename.clone()))?;

            size = size
                .checked_add(item_size)
                .ok_or(TransformDatasetError::ItemSizeOverflow)?;

            items.push(DatasetItemInput {
                path: sibling.rfilename,
                size: item_size,
            });
        }

        Ok(Self {
            name,
            description: None,
            tags: sanitize_tags(value.tags),
            provider: DatasetProviderInput::HuggingFace(HuggingFaceRepoLocatorInput {
                id: value.id,
                sha: value.sha,
            }),
            items,
            size,
            visibility: VisibilityInput::Public,
        })
    }
}

fn sanitize_tags(tags: Vec<String>) -> Vec<String> {
    let mut unique = HashSet::new();

    tags.into_iter()
        .filter(|tag| !tag.is_empty() && tag.len() <= MAX_TAG_LENGTH_BYTES)
        .filter(|tag| unique.insert(tag.clone()))
        .take(MAX_TAGS)
        .collect()
}

#[cfg(test)]
#[path = "transform.test.rs"]
mod transform_test;
