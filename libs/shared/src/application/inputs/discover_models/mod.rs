use platforms::Platform;

use crate::{domain::entities::model::external_model::ModelProvider, shared_kernel::enums::Task};

#[derive(Debug, Clone, Default)]
pub struct SearchCriterion {
    pub provider: Option<ModelProvider>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub inference_runtimes: Vec<String>,
    pub tags: Vec<String>,
    pub task_types: Vec<Task>,
    pub license: Option<String>,
    pub size: NumericRange<u64>,
    pub likes: NumericRange<u128>,
    pub downloads: NumericRange<u128>,
    pub deployment_strategies: Vec<DeploymentStrategyCriterion>,
    pub has_deployment_strategies: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct NumericRange<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

#[derive(Debug, Clone)]
pub struct DeploymentStrategyCriterion {
    pub name: String,
    pub platform: Platform,
}

#[derive(Debug, Clone)]
pub struct SearchExternalModelsInput {
    pub criteria: Vec<SearchCriterion>,
    pub options: SearchOptions,
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    limit: u16,
    cursor: Option<String>,
    include_count: bool,
}

impl SearchOptions {
    pub const MAX_LIMIT: u16 = 1000;
    pub const DEFAULT_LIMIT: u16 = 100;

    pub fn new(limit: Option<u16>, cursor: Option<String>, include_count: Option<bool>) -> Self {
        Self {
            limit: limit
                .unwrap_or(Self::DEFAULT_LIMIT)
                .clamp(1, Self::MAX_LIMIT),
            cursor,
            include_count: include_count.unwrap_or(false),
        }
    }

    pub fn limit(&self) -> u16 {
        self.limit
    }

    pub fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    pub fn include_count(&self) -> bool {
        self.include_count
    }
}
