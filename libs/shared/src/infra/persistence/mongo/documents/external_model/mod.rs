pub mod external_model_indexes;

use mongodb::bson::{oid::ObjectId, DateTime, Uuid};
use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    domain::entities::model::external_model as domain,
    infra::persistence::mongo::documents::task::Task,
    shared_kernel::{identifiers::ExternalModelId, value_objects::TimeStamp},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub provider: ModelProvider,
    pub huggingface_repo_locator: Option<HuggingFaceRepoLocator>,
    pub tapis_system_locator: Option<TapisSystemLocator>,
    pub metadata: ModelMetadata,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    HuggingFace,
    Tapis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceRepoLocator {
    pub id: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapisSystemLocator {
    pub site_id: String,
    pub tenant_id: String,
    pub system_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub derived: DerivedMetadata,
    pub canonical: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedMetadata {
    pub name: Option<String>,
    pub author: Option<String>,
    pub deployment_strategies: Vec<DeploymentStrategyReference>,
    pub inference_runtimes: Vec<String>,
    pub tags: Vec<String>,
    pub task_types: Vec<Task>,
    pub license: Option<String>,
    pub size: u64,
    pub gated: bool,
    pub private: bool,
    pub likes: Option<u64>,
    pub downloads: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStrategyReference {
    pub name: String,
    pub platform: Platform,
}

impl TryFrom<&domain::ExternalModel> for ExternalModel {
    type Error = domain::ExternalModelError;

    fn try_from(value: &domain::ExternalModel) -> Result<Self, Self::Error> {
        let (provider, huggingface_repo_locator, tapis_system_locator) = match value.locator() {
            domain::ModelLocator::HuggingFace(locator) => (
                ModelProvider::HuggingFace,
                Some(HuggingFaceRepoLocator {
                    id: locator.id().into(),
                    sha: locator.sha().into(),
                }),
                None,
            ),
            domain::ModelLocator::Tapis(locator) => (
                ModelProvider::Tapis,
                None,
                Some(TapisSystemLocator {
                    site_id: locator.site_id().into(),
                    tenant_id: locator.tenant_id().into(),
                    system_id: locator.system_id().into(),
                    path: locator.path().into(),
                }),
            ),
        };

        let derived = value.metadata().derived();

        let likes = derived
            .likes()
            .map(u64::try_from)
            .transpose()
            .map_err(|error| domain::ExternalModelError::DataIntegrityError(error.to_string()))?;

        let downloads = derived
            .downloads()
            .map(u64::try_from)
            .transpose()
            .map_err(|error| domain::ExternalModelError::DataIntegrityError(error.to_string()))?;

        Ok(Self {
            _id: None,
            id: Uuid::from_bytes(*value.id().as_uuid().as_bytes()),
            provider,
            huggingface_repo_locator,
            tapis_system_locator,
            metadata: ModelMetadata {
                derived: DerivedMetadata {
                    name: derived.name().map(Into::into),
                    author: derived.author().map(Into::into),
                    deployment_strategies: derived
                        .deployment_strategies()
                        .iter()
                        .map(|strategy| DeploymentStrategyReference {
                            name: strategy.name().into(),
                            platform: strategy.platform().clone(),
                        })
                        .collect(),
                    inference_runtimes: derived.inference_runtimes().to_vec(),
                    tags: derived
                        .tags()
                        .iter()
                        .map(|tag| tag.as_str().to_owned())
                        .collect(),
                    task_types: derived
                        .task_types()
                        .iter()
                        .cloned()
                        .map(Into::into)
                        .collect(),
                    license: derived.license().map(Into::into),
                    size: derived.size(),
                    gated: derived.gated(),
                    private: derived.private(),
                    likes,
                    downloads,
                },
                canonical: value.metadata().canonical().clone(),
            },
            updated_at: DateTime::from_millis(value.updated_at().into_inner().timestamp_millis()),
            created_at: DateTime::from_millis(value.created_at().into_inner().timestamp_millis()),
        })
    }
}

impl TryFrom<ExternalModel> for domain::ExternalModel {
    type Error = domain::ExternalModelError;

    fn try_from(value: ExternalModel) -> Result<Self, Self::Error> {
        let provider = match value.provider {
            ModelProvider::HuggingFace => domain::ModelProvider::HuggingFace,
            ModelProvider::Tapis => domain::ModelProvider::Tapis,
        };

        let locator = match (
            provider.clone(),
            value.huggingface_repo_locator,
            value.tapis_system_locator,
        ) {
            (domain::ModelProvider::HuggingFace, Some(locator), None) => {
                domain::ModelLocator::HuggingFace(
                    domain::HuggingFaceRepoLocator::reconstitute(locator.id, locator.sha).map_err(
                        |error| domain::ExternalModelError::DataIntegrityError(error.to_string()),
                    )?,
                )
            }
            (domain::ModelProvider::Tapis, None, Some(locator)) => domain::ModelLocator::Tapis(
                domain::TapisSystemLocator::reconstitute(
                    locator.site_id,
                    locator.tenant_id,
                    locator.system_id,
                    locator.path,
                )
                .map_err(|error| {
                    domain::ExternalModelError::DataIntegrityError(error.to_string())
                })?,
            ),
            _ => {
                return Err(domain::ExternalModelError::DataIntegrityError(
                    "ExternalModel provider does not match exactly one locator".into(),
                ))
            }
        };

        let derived = value.metadata.derived;

        let derived = domain::DerivedMetadata::reconstitute(
            derived.name,
            derived.author,
            derived
                .deployment_strategies
                .into_iter()
                .map(|strategy| {
                    domain::DeploymentStrategyReference::new(strategy.name, strategy.platform)
                })
                .collect(),
            derived.inference_runtimes,
            derived.tags,
            derived.task_types.into_iter().map(Into::into).collect(),
            derived.license,
            derived.size,
            derived.gated,
            derived.private,
            derived.likes.map(u128::from),
            derived.downloads.map(u128::from),
        )
        .map_err(|error| domain::ExternalModelError::DataIntegrityError(error.to_string()))?;

        domain::ExternalModel::reconstitute(domain::ReconstituteExternalModelProps {
            id: ExternalModelId::reconstitute(uuid::Uuid::from_bytes(value.id.bytes())),
            provider,
            locator,
            metadata: domain::ModelMetadata::reconstitute(derived, value.metadata.canonical),
            updated_at: TimeStamp::from(value.updated_at.to_chrono()),
            created_at: TimeStamp::from(value.created_at.to_chrono()),
        })
    }
}

#[cfg(test)]
#[path = "external_model.test.rs"]
mod external_model_test;
