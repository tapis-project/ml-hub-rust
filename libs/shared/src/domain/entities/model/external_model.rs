use platforms::Platform;
use serde_json::{Map, Value};
use thiserror::Error;

use crate::shared_kernel::{
    constants::GLOBAL_TENANT,
    enums::Task,
    identifiers::{traits::UrnGenerator, urn::Urn, ExternalModelId},
    value_objects::{Tags, TagsError, TimeStamp},
};

#[derive(Debug, Clone)]
pub struct ExternalModel {
    id: ExternalModelId,
    provider: ModelProvider,
    locator: ModelLocator,
    metadata: ModelMetadata,
    updated_at: TimeStamp,
    created_at: TimeStamp,
}

impl UrnGenerator for ExternalModel {
    fn urn(&self) -> Urn {
        Urn::new(format!(
            "urn:mlhub:v1:{GLOBAL_TENANT}:external_model:{}",
            self.id
        ))
    }
}

impl ExternalModel {
    pub fn ingest(
        provider: ModelProvider,
        locator: ModelLocator,
        metadata: ModelMetadata,
    ) -> Result<Self, ExternalModelError> {
        Self::validate_provider_locator(&provider, &locator)?;

        let now = TimeStamp::now();

        Ok(Self {
            id: ExternalModelId::new(),
            provider,
            locator,
            metadata,
            updated_at: now.clone(),
            created_at: now,
        })
    }

    pub fn reconstitute(props: ReconstituteExternalModelProps) -> Result<Self, ExternalModelError> {
        Self::validate_provider_locator(&props.provider, &props.locator)
            .map_err(|error| ExternalModelError::DataIntegrityError(error.to_string()))?;

        Ok(Self {
            id: props.id,
            provider: props.provider,
            locator: props.locator,
            metadata: props.metadata,
            updated_at: props.updated_at,
            created_at: props.created_at,
        })
    }

    pub fn refresh(&mut self, metadata: ModelMetadata) {
        self.metadata = metadata;

        self.updated_at = TimeStamp::now();
    }

    pub fn replace_deployment_strategies(&mut self, strategies: Vec<DeploymentStrategyReference>) {
        self.metadata.derived.deployment_strategies = strategies;

        self.updated_at = TimeStamp::now();
    }

    pub fn id(&self) -> &ExternalModelId {
        &self.id
    }

    pub fn provider(&self) -> &ModelProvider {
        &self.provider
    }

    pub fn locator(&self) -> &ModelLocator {
        &self.locator
    }

    pub fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    pub fn updated_at(&self) -> &TimeStamp {
        &self.updated_at
    }

    pub fn created_at(&self) -> &TimeStamp {
        &self.created_at
    }

    pub fn get_field_value_at_field_path(
        &self,
        field_path: &[String],
    ) -> Result<FieldValue, ExternalModelError> {
        let path = field_path.iter().map(String::as_str).collect::<Vec<_>>();

        let derived = self.metadata.derived();

        match path.as_slice() {
            ["provider"] => Ok(FieldValue::String(Some(self.provider.to_string()))),
            ["metadata", "derived", "name"] => Ok(FieldValue::String(derived.name.clone())),
            ["metadata", "derived", "author"] => Ok(FieldValue::String(derived.author.clone())),
            ["metadata", "derived", "inference_runtimes"] => {
                Ok(FieldValue::Strings(derived.inference_runtimes.clone()))
            }
            ["metadata", "derived", "tags"] => Ok(FieldValue::Strings(
                derived
                    .tags
                    .iter()
                    .map(|tag| tag.as_str().to_owned())
                    .collect(),
            )),
            ["metadata", "derived", "task_types"] => Ok(FieldValue::Strings(
                derived
                    .task_types
                    .iter()
                    .cloned()
                    .map(String::from)
                    .collect(),
            )),
            ["metadata", "derived", "license"] => Ok(FieldValue::String(derived.license.clone())),
            ["metadata", "derived", "size"] => Ok(FieldValue::Unsigned(derived.size)),
            ["metadata", "derived", "gated"] => Ok(FieldValue::Boolean(derived.gated)),
            ["metadata", "derived", "private"] => Ok(FieldValue::Boolean(derived.private)),
            ["metadata", "derived", "likes"] => Ok(FieldValue::OptionalUnsigned(derived.likes)),
            ["metadata", "derived", "downloads"] => {
                Ok(FieldValue::OptionalUnsigned(derived.downloads))
            }
            other => Err(ExternalModelError::InvalidFieldPath(
                other.iter().map(|part| (*part).to_owned()).collect(),
            )),
        }
    }

    fn validate_provider_locator(
        provider: &ModelProvider,
        locator: &ModelLocator,
    ) -> Result<(), ExternalModelError> {
        if matches!(
            (provider, locator),
            (ModelProvider::HuggingFace, ModelLocator::HuggingFace(_))
                | (ModelProvider::Tapis, ModelLocator::Tapis(_))
        ) {
            return Ok(());
        }

        Err(ExternalModelError::ProviderLocatorMismatch)
    }
}

#[derive(Debug, Clone)]
pub struct ReconstituteExternalModelProps {
    pub id: ExternalModelId,
    pub provider: ModelProvider,
    pub locator: ModelLocator,
    pub metadata: ModelMetadata,
    pub updated_at: TimeStamp,
    pub created_at: TimeStamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelProvider {
    HuggingFace,
    Tapis,
}

impl std::fmt::Display for ModelProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HuggingFace => write!(f, "HuggingFace"),
            Self::Tapis => write!(f, "Tapis"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelMetadata {
    derived: DerivedMetadata,
    canonical: Map<String, Value>,
}

impl ModelMetadata {
    pub fn new(derived: DerivedMetadata, canonical: Map<String, Value>) -> Self {
        Self { derived, canonical }
    }

    pub fn reconstitute(derived: DerivedMetadata, canonical: Map<String, Value>) -> Self {
        Self { derived, canonical }
    }

    pub fn derived(&self) -> &DerivedMetadata {
        &self.derived
    }

    pub fn canonical(&self) -> &Map<String, Value> {
        &self.canonical
    }
}

#[derive(Debug, Clone)]
pub struct DerivedMetadata {
    name: Option<String>,
    author: Option<String>,
    deployment_strategies: Vec<DeploymentStrategyReference>,
    inference_runtimes: Vec<String>,
    tags: Tags,
    task_types: Vec<Task>,
    license: Option<String>,
    size: u64,
    gated: bool,
    private: bool,
    likes: Option<u128>,
    downloads: Option<u128>,
}

impl DerivedMetadata {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<String>,
        author: Option<String>,
        inference_runtimes: Vec<String>,
        tags: Vec<String>,
        task_types: Vec<Task>,
        license: Option<String>,
        size: u64,
        gated: bool,
        private: bool,
        likes: Option<u128>,
        downloads: Option<u128>,
    ) -> Result<Self, ExternalModelError> {
        Self::build(
            name,
            author,
            Vec::new(),
            inference_runtimes,
            tags,
            task_types,
            license,
            size,
            gated,
            private,
            likes,
            downloads,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        name: Option<String>,
        author: Option<String>,
        deployment_strategies: Vec<DeploymentStrategyReference>,
        inference_runtimes: Vec<String>,
        tags: Vec<String>,
        task_types: Vec<Task>,
        license: Option<String>,
        size: u64,
        gated: bool,
        private: bool,
        likes: Option<u128>,
        downloads: Option<u128>,
    ) -> Result<Self, ExternalModelError> {
        Self::build(
            name,
            author,
            deployment_strategies,
            inference_runtimes,
            tags,
            task_types,
            license,
            size,
            gated,
            private,
            likes,
            downloads,
            true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build(
        name: Option<String>,
        author: Option<String>,
        deployment_strategies: Vec<DeploymentStrategyReference>,
        inference_runtimes: Vec<String>,
        tags: Vec<String>,
        task_types: Vec<Task>,
        license: Option<String>,
        size: u64,
        gated: bool,
        private: bool,
        likes: Option<u128>,
        downloads: Option<u128>,
        reconstituting: bool,
    ) -> Result<Self, ExternalModelError> {
        let tags = if reconstituting {
            Tags::reconstitute(tags)
        } else {
            Tags::new(tags)
        }
        .map_err(ExternalModelError::InvalidTags)?;

        if inference_runtimes.iter().any(|value| value.is_empty()) {
            return Err(ExternalModelError::EmptyInferenceRuntime);
        }

        Ok(Self {
            name,
            author,
            deployment_strategies,
            inference_runtimes,
            tags,
            task_types,
            license,
            size,
            gated,
            private,
            likes,
            downloads,
        })
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn deployment_strategies(&self) -> &[DeploymentStrategyReference] {
        &self.deployment_strategies
    }

    pub fn inference_runtimes(&self) -> &[String] {
        &self.inference_runtimes
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn task_types(&self) -> &[Task] {
        &self.task_types
    }

    pub fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn gated(&self) -> bool {
        self.gated
    }

    pub fn private(&self) -> bool {
        self.private
    }

    pub fn likes(&self) -> Option<u128> {
        self.likes
    }

    pub fn downloads(&self) -> Option<u128> {
        self.downloads
    }
}

#[derive(Debug, Clone)]
pub enum ModelLocator {
    HuggingFace(HuggingFaceRepoLocator),
    Tapis(TapisSystemLocator),
}

#[derive(Clone, Debug)]
pub struct HuggingFaceRepoLocator {
    id: String,
    sha: String,
}

impl HuggingFaceRepoLocator {
    pub fn new(id: String, sha: String) -> Result<Self, ModelLocatorError> {
        ensure_not_empty("id", &id)?;
        ensure_not_empty("sha", &sha)?;

        Ok(Self { id, sha })
    }

    pub fn reconstitute(id: String, sha: String) -> Result<Self, ModelLocatorError> {
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
    ) -> Result<Self, ModelLocatorError> {
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
    ) -> Result<Self, ModelLocatorError> {
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

fn ensure_not_empty(field: &'static str, value: &str) -> Result<(), ModelLocatorError> {
    if value.is_empty() {
        return Err(ModelLocatorError::EmptyField(field));
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentStrategyReference {
    name: String,
    platform: Platform,
}

impl DeploymentStrategyReference {
    pub fn new(name: String, platform: Platform) -> Self {
        Self { name, platform }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn platform(&self) -> &Platform {
        &self.platform
    }
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    String(Option<String>),
    Strings(Vec<String>),
    Boolean(bool),
    Unsigned(u64),
    OptionalUnsigned(Option<u128>),
}

impl From<FieldValue> for Value {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(value) => serde_json::to_value(value).unwrap_or(Value::Null),
            FieldValue::Strings(value) => serde_json::to_value(value).unwrap_or(Value::Null),
            FieldValue::Boolean(value) => Value::Bool(value),
            FieldValue::Unsigned(value) => Value::Number(value.into()),
            FieldValue::OptionalUnsigned(value) => {
                serde_json::to_value(value).unwrap_or(Value::Null)
            }
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum ModelLocatorError {
    #[error("Model locator field {0} MUST not be empty")]
    EmptyField(&'static str),
}

#[cfg(test)]
#[path = "external_model.test.rs"]
mod external_model_test;

#[derive(Debug, Error)]
pub enum ExternalModelError {
    #[error("Model provider does not match its locator")]
    ProviderLocatorMismatch,

    #[error("Inference runtime MUST not be empty")]
    EmptyInferenceRuntime,

    #[error("External model contains invalid tags: {0}")]
    InvalidTags(#[source] TagsError),

    #[error("Invalid ExternalModel field path: {0:?}")]
    InvalidFieldPath(Vec<String>),

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}
