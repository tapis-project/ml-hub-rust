use std::collections::HashMap;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    application::inputs::discover_models as inputs,
    domain::entities::model::external_model::ModelProvider as DomainModelProvider,
    presentation::http::v1::requests::{common::headers::Headers, common::tasks::Task},
    shared_kernel::enums::Task as DomainTask,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscoverModelsByPlatformPath {
    pub platform: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DiscoverExternalModelsBody {
    #[serde(default)]
    pub criteria: Vec<DiscoveryCriterion>,
}

pub type DiscoveryCriteria = DiscoverExternalModelsBody;
pub type DiscoverModelsQueryParams = DiscoverExternalModelsQueryParams;

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DiscoverExternalModelsQueryParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub include_count: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryCriterion {
    pub provider: Option<ModelProvider>,
    pub name: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub inference_runtimes: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub task_types: Vec<Task>,
    pub license: Option<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub min_likes: Option<u128>,
    pub max_likes: Option<u128>,
    pub min_downloads: Option<u128>,
    pub max_downloads: Option<u128>,
    #[serde(default)]
    pub deployment_strategies: Vec<DeploymentStrategyCriterion>,
    pub has_deployment_strategies: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum ModelProvider {
    HuggingFace,
    Tapis,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct DeploymentStrategyCriterion {
    pub name: String,
    pub platform: Platform,
}

impl From<DiscoveryCriterion> for inputs::SearchCriterion {
    fn from(value: DiscoveryCriterion) -> Self {
        Self {
            provider: value.provider.map(|provider| match provider {
                ModelProvider::HuggingFace => DomainModelProvider::HuggingFace,
                ModelProvider::Tapis => DomainModelProvider::Tapis,
            }),
            name: value.name,
            author: value.author,
            inference_runtimes: value.inference_runtimes,
            tags: value.tags,
            task_types: value.task_types.into_iter().map(DomainTask::from).collect(),
            license: value.license,
            size: inputs::NumericRange {
                min: value.min_size,
                max: value.max_size,
            },
            likes: inputs::NumericRange {
                min: value.min_likes,
                max: value.max_likes,
            },
            downloads: inputs::NumericRange {
                min: value.min_downloads,
                max: value.max_downloads,
            },
            deployment_strategies: value
                .deployment_strategies
                .into_iter()
                .map(|strategy| inputs::DeploymentStrategyCriterion {
                    name: strategy.name,
                    platform: strategy.platform,
                })
                .collect(),
            has_deployment_strategies: value.has_deployment_strategies,
        }
    }
}

// Retained only as an internal client transport contract. It is no longer exposed by Models HTTP.
pub struct DiscoverModelsByPlatformRequest {
    pub headers: Headers,
    pub path: DiscoverModelsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: DiscoverExternalModelsBody,
}
