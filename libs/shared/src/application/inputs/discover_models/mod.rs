use serde_json::Value;
use crate::application::inputs::task::Task;

#[derive(Debug, Clone)]
pub struct SystemRequirement {
    pub name: Option<String>,
    pub version: Option<String>
}

#[derive(Debug, Clone)]
pub struct Accelerator {
    pub accelerator_type: Option<String>,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Option<Vec<SystemRequirement>>
}

#[derive(Debug, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Debug, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Debug, Clone)]
pub struct SearchCriterion {
    // General fields
    pub name: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub image: Option<String>,

    /// Arbitrary labels
    pub keywords: Option<Vec<String>>,
    pub annotations: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,
    pub model_inputs: Option<Vec<ModelIO>>,
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    pub task_types: Option<Vec<Task>>,
    pub inference_precision: Option<String>,
    pub inference_hardware: Option<HardwareRequirements>,
    pub inference_software_dependencies: Option<Vec<String>>,
    pub inference_max_energy_consumption_watts: Option<i32>,

    /// Inference performance fields
    pub inference_max_latency_ms: Option<i32>,
    pub inference_min_throughput: Option<i32>,
    pub inference_max_compute_utilization_percentage: Option<i32>,
    pub inference_max_memory_usage_mb: Option<i32>,
    pub inference_distributed: Option<bool>,

    /// Training-related Fields
    pub training_time: Option<i64>,
    pub training_precision: Option<String>,
    pub training_hardware: Option<HardwareRequirements>,
    pub pretraining_datasets: Option<Vec<String>>,
    pub finetuning_datasets: Option<Vec<String>>,
    pub edge_optimized: Option<bool>,
    pub quantization_aware: Option<bool>,
    pub supports_quantization: Option<bool>,
    pub pretrained: Option<bool>,
    pub pruned: Option<bool>,
    pub slimmed: Option<bool>,
    pub training_distributed: Option<bool>,

    /// Training performance fields
    pub training_max_energy_consumption_watts: Option<i32>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    pub bias_evaluation_score: Option<i8>,
}

/// Each field in the ModelMetadata will be ANDed and each individual ModelMetadata
/// themselves will be ORed
#[derive(Debug, Clone)]
pub struct DiscoverModelsInput {
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