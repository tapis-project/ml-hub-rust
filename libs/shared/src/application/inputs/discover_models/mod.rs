use crate::shared_kernel::enums::Task;

#[derive(Debug, Clone)]
pub struct SearchCriterion {
    // General fields
    pub name: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
}

/// Each field in the ModelMetadata will be ANDed and each individual SearchCriteron
/// themselves will be ORed
#[derive(Debug, Clone)]
pub struct SearchModelsInput {
    pub criteria: Vec<SearchCriterion>,
    pub options: SearchOptions
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    limit: Option<u16>,
    cursor: Option<String>,
    include_count: Option<bool>,
    include_global_models: Option<bool>,
}

impl SearchOptions {
    pub const MAX_LIMIT: u16 = 1000;
    pub const DEFAULT_LIMIT: u16 = 100;
    pub const DEFAULT_INCLUDE_COUNT: bool = false;
    pub const DEFAULT_INCLUDE_GLOBAL_MODELS: bool = true;

    pub fn new(
        limit: Option<u16>, 
        cursor: Option<String>, 
        include_count: Option<bool>, 
        include_global_models: Option<bool>,
    ) -> Self {
        let limit_final = if let Some(l) = limit {
            l.min(Self::MAX_LIMIT)
        } else {
            Self::DEFAULT_LIMIT
        };

        let include_count_final = if let Some(ic) = include_count {
            ic
        } else {
            Self::DEFAULT_INCLUDE_COUNT
        };

        let include_global_models_final = if let Some(igm) = include_global_models {
            igm
        } else {
            Self::DEFAULT_INCLUDE_GLOBAL_MODELS
        };

        Self {
            limit: Some(limit_final),
            cursor,
            include_count: Some(include_count_final),
            include_global_models: Some(include_global_models_final),
        }
    }

    pub fn limit(&self) -> Option<u16> {
        return self.limit.clone()
    }

    pub fn cursor(&self) -> Option<String> {
        return self.cursor.clone()
    }

    pub fn include_count(&self) -> Option<bool> {
        return self.include_count.clone()
    }

    pub fn include_global_models(&self) -> Option<bool> {
        return self.include_global_models.clone()
    }
}