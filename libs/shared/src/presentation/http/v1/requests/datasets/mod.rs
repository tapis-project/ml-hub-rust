use crate::shared_kernel::value_objects::{MAX_TAGS, MAX_TAG_LENGTH_BYTES};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_register_dataset"))]
pub struct RegisterDatasetBody {
    #[serde(default)]
    #[validate(custom(function = "validate_tags"))]
    pub tags: Vec<String>,
    pub provider: DatasetProvider,
    #[schema(nullable)]
    #[validate(nested)]
    pub huggingface_repo_locator: Option<HuggingFaceRepoLocator>,
    #[schema(nullable)]
    #[validate(nested)]
    pub tapis_system_locator: Option<TapisSystemLocator>,
    #[validate(nested)]
    pub items: Vec<DatasetItem>,
    pub size: u64,
    #[serde(default)]
    pub visibility: Visibility,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DatasetItem {
    #[validate(length(min = 1))]
    pub path: String,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct HuggingFaceRepoLocator {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub sha: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TapisSystemLocator {
    #[validate(length(min = 1))]
    pub site_id: String,
    #[validate(length(min = 1))]
    pub tenant_id: String,
    #[validate(length(min = 1))]
    pub system_id: String,
    #[validate(length(min = 1))]
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub enum DatasetProvider {
    HuggingFace,
    Tapis,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
pub enum Visibility {
    Public,
    #[default]
    Private,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListDatasetsQueryParams {
    #[serde(default)]
    #[param(inline)]
    pub scope: Scope,
}

#[derive(Clone, Debug, Default, Deserialize, ToSchema)]
pub enum Scope {
    #[default]
    Owned,
    Shared,
    Global,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetDatasetPath {
    #[param(value_type = String, format = "uuid")]
    pub dataset_id: Uuid,
}

fn validate_tags(tags: &Vec<String>) -> Result<(), ValidationError> {
    if tags.len() > MAX_TAGS
        || tags
            .iter()
            .any(|tag| tag.is_empty() || tag.len() > MAX_TAG_LENGTH_BYTES)
    {
        return Err(ValidationError::new("invalid_tags"));
    }

    Ok(())
}

fn validate_register_dataset(body: &RegisterDatasetBody) -> Result<(), ValidationError> {
    let locator_matches = matches!(
        (
            &body.provider,
            &body.huggingface_repo_locator,
            &body.tapis_system_locator
        ),
        (DatasetProvider::HuggingFace, Some(_), None) | (DatasetProvider::Tapis, None, Some(_))
    );
    if !locator_matches {
        return Err(ValidationError::new("provider_locator_mismatch"));
    }

    let mut paths = std::collections::HashSet::new();

    if body.items.iter().any(|item| !paths.insert(&item.path)) {
        return Err(ValidationError::new("duplicate_dataset_item_path"));
    }

    Ok(())
}

#[cfg(test)]
#[path = "datasets.test.rs"]
mod datasets_test;
