use std::collections::HashSet;

use thiserror::Error;
use uuid::Uuid;

use crate::impl_urn_generator;
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::{Tags, TagsError};

#[derive(Clone, Debug)]
pub struct Dataset {
    id: Uuid,
    tenant_id: String,
    owner: String,
    tags: Tags,
    provider: DatasetProvider,
    items: Vec<DatasetItem>,
    size: u64,
    visibility: Visibility,
}

impl_urn_generator!(Dataset, tenant_id, "dataset", id);

impl Dataset {
    pub fn register(
        tenant_id: String,
        owner: String,
        tags: Vec<String>,
        provider: DatasetProvider,
        items: Vec<DatasetItem>,
        size: u64,
        visibility: Visibility,
    ) -> Result<Self, DatasetError> {
        Self::validate_items(&items, size)?;

        let tags = Tags::new(tags).map_err(DatasetError::InvalidTags)?;

        Ok(Self {
            id: Uuid::now_v7(),
            tenant_id,
            owner,
            tags,
            provider,
            items,
            size,
            visibility,
        })
    }

    pub fn reconstitute(props: ReconstituteDatasetProps) -> Result<Self, DatasetError> {
        Self::validate_items(&props.items, props.size).map_err(|error| {
            DatasetError::DataIntegrityError(format!("Dataset contains invalid items: {error}"))
        })?;

        let tags = Tags::reconstitute(props.tags).map_err(|error| {
            DatasetError::DataIntegrityError(format!("Dataset contains invalid tags: {error}"))
        })?;

        Ok(Self {
            id: props.id,
            tenant_id: props.tenant_id,
            owner: props.owner,
            tags,
            provider: props.provider,
            items: props.items,
            size: props.size,
            visibility: props.visibility,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn provider(&self) -> &DatasetProvider {
        &self.provider
    }

    pub fn items(&self) -> &[DatasetItem] {
        &self.items
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    fn validate_items(items: &[DatasetItem], size: u64) -> Result<(), DatasetError> {
        let mut paths = HashSet::new();
        let mut calculated_size = 0_u64;

        for item in items {
            if !paths.insert(item.path()) {
                return Err(DatasetError::DuplicateItemPath(item.path().to_owned()));
            }

            calculated_size = calculated_size
                .checked_add(item.size())
                .ok_or(DatasetError::ItemSizeOverflow)?;
        }

        if calculated_size != size {
            return Err(DatasetError::SizeMismatch {
                declared: size,
                calculated: calculated_size,
            });
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteDatasetProps {
    pub id: Uuid,
    pub tenant_id: String,
    pub owner: String,
    pub tags: Vec<String>,
    pub provider: DatasetProvider,
    pub items: Vec<DatasetItem>,
    pub size: u64,
    pub visibility: Visibility,
}

#[derive(Clone, Debug)]
pub struct DatasetItem {
    path: String,
    size: u64,
}

impl DatasetItem {
    pub fn new(path: String, size: u64) -> Result<Self, DatasetItemError> {
        if path.is_empty() {
            return Err(DatasetItemError::EmptyPath);
        }

        Ok(Self { path, size })
    }

    pub fn reconstitute(path: String, size: u64) -> Result<Self, DatasetItemError> {
        Self::new(path, size)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Clone, Debug)]
pub enum DatasetProvider {
    HuggingFace(HuggingFaceRepoLocator),
    Tapis(TapisSystemLocator),
}

#[derive(Clone, Debug)]
pub struct HuggingFaceRepoLocator {
    id: String,
    sha: String,
}

impl HuggingFaceRepoLocator {
    pub fn new(id: String, sha: String) -> Result<Self, DatasetLocatorError> {
        ensure_not_empty("id", &id)?;

        ensure_not_empty("sha", &sha)?;

        Ok(Self { id, sha })
    }

    pub fn reconstitute(id: String, sha: String) -> Result<Self, DatasetLocatorError> {
        Self::new(id, sha)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn sha(&self) -> &str {
        &self.sha
    }
}

#[derive(Clone, Debug)]
pub struct TapisSystemLocator {
    site_id: String,
    tenant_id: String,
    system_id: String,
    path: String,
}

impl TapisSystemLocator {
    pub fn new(
        site_id: String,
        tenant_id: String,
        system_id: String,
        path: String,
    ) -> Result<Self, DatasetLocatorError> {
        ensure_not_empty("site_id", &site_id)?;

        ensure_not_empty("tenant_id", &tenant_id)?;

        ensure_not_empty("system_id", &system_id)?;

        ensure_not_empty("path", &path)?;

        Ok(Self {
            site_id,
            tenant_id,
            system_id,
            path,
        })
    }

    pub fn reconstitute(
        site_id: String,
        tenant_id: String,
        system_id: String,
        path: String,
    ) -> Result<Self, DatasetLocatorError> {
        Self::new(site_id, tenant_id, system_id, path)
    }

    pub fn site_id(&self) -> &str {
        &self.site_id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn system_id(&self) -> &str {
        &self.system_id
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

fn ensure_not_empty(field: &'static str, value: &str) -> Result<(), DatasetLocatorError> {
    if value.is_empty() {
        return Err(DatasetLocatorError::EmptyField(field));
    }

    Ok(())
}

#[derive(Clone, Debug, Error)]
pub enum DatasetError {
    #[error("Dataset contains duplicate item path: {0}")]
    DuplicateItemPath(String),

    #[error("Dataset item sizes exceed the supported u64 total")]
    ItemSizeOverflow,

    #[error("Dataset declared size {declared} does not match calculated item size {calculated}")]
    SizeMismatch { declared: u64, calculated: u64 },

    #[error("Dataset contains invalid tags: {0}")]
    InvalidTags(#[source] TagsError),

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[derive(Clone, Debug, Error)]
pub enum DatasetItemError {
    #[error("Dataset item path MUST not be empty")]
    EmptyPath,
}

#[derive(Clone, Debug, Error)]
pub enum DatasetLocatorError {
    #[error("Dataset locator field {0} MUST not be empty")]
    EmptyField(&'static str),
}

#[cfg(test)]
#[path = "dataset.test.rs"]
mod dataset_test;
