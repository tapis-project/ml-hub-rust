use platforms::Platform;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    application::outputs::model::ModelWithExternalModel,
    domain::entities::model::external_model as domain,
    presentation::http::v1::responses::{tasks::Task, visibility::Visibility},
};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ModelArtifact {
    pub id: String,
    pub created_at: String,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub owner: String,
    pub artifact_id: Option<Uuid>,
    pub external_model_id: Uuid,
    pub external_model: ExternalModel,
    pub visibility: Visibility,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExternalModel {
    pub id: Uuid,
    pub provider: ModelProvider,
    pub huggingface_repo_locator: Option<HuggingFaceRepoLocator>,
    pub tapis_system_locator: Option<TapisSystemLocator>,
    pub metadata: ExternalModelMetadata,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ModelProvider {
    HuggingFace,
    Tapis,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HuggingFaceRepoLocator {
    pub id: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TapisSystemLocator {
    pub site_id: String,
    pub tenant_id: String,
    pub system_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExternalModelMetadata {
    pub derived: DerivedModelMetadata,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DerivedModelMetadata {
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
    pub likes: Option<u128>,
    pub downloads: Option<u128>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeploymentStrategyReference {
    pub name: String,
    pub platform: Platform,
}

impl From<ModelWithExternalModel> for Model {
    fn from(value: ModelWithExternalModel) -> Self {
        Self {
            id: *value.model.id(),
            name: value.model.name().into(),
            description: value.model.description().map(Into::into),
            tenant_id: value.model.tenant_id().into(),
            owner: value.model.owner().into(),
            artifact_id: value.model.artifact_id().copied(),
            external_model_id: value.model.external_model_id().into_uuid(),
            external_model: value.external_model.into(),
            visibility: value.model.visibility().clone().into(),
            updated_at: value.model.updated_at().clone().into(),
            created_at: value.model.created_at().clone().into(),
        }
    }
}

impl From<domain::ExternalModel> for ExternalModel {
    fn from(value: domain::ExternalModel) -> Self {
        let (huggingface_repo_locator, tapis_system_locator) = match value.locator() {
            domain::ModelLocator::HuggingFace(locator) => (
                Some(HuggingFaceRepoLocator {
                    id: locator.id().into(),
                    sha: locator.sha().into(),
                }),
                None,
            ),
            domain::ModelLocator::Tapis(locator) => (
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
        Self {
            id: value.id().into_uuid(),
            provider: match value.provider() {
                domain::ModelProvider::HuggingFace => ModelProvider::HuggingFace,
                domain::ModelProvider::Tapis => ModelProvider::Tapis,
            },
            huggingface_repo_locator,
            tapis_system_locator,
            metadata: ExternalModelMetadata {
                derived: DerivedModelMetadata {
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
                    likes: derived.likes(),
                    downloads: derived.downloads(),
                },
            },
            updated_at: value.updated_at().clone().into(),
            created_at: value.created_at().clone().into(),
        }
    }
}
