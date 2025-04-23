use crate::requests;
use crate::errors::Error;
use serde_json::Value;

pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

impl TryFrom<requests::SystemRequirement> for SystemRequirement {
    type Error = Error;
    
    fn try_from(value: requests::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

impl TryFrom<requests::Accelerator> for Accelerator {
    type Error = Error;
    
    fn try_from(value: requests::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

impl TryFrom<requests::HardwareRequirements> for HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: requests::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(Accelerator::try_from(accelerator)?);
        }

        Ok(Self {
            cpus: value.cpus,
            memory_gb: value.memory_gb,
            disk_gb: value.disk_gb,
            accelerators: Some(accelerators),
            architectures: value.architectures
        })
    }
}

pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

impl TryFrom<requests::ModelIO> for ModelIO {
    type Error = Error;
    
    fn try_from(value: requests::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

pub struct ModelDiscoveryCriteria {
    // General fields
    pub name: Option<String>,
    pub model_type: Option<String>,
    pub version: Option<String>,
    pub framework: Option<String>,
    pub image: Option<String>,

    /// Arbitrary labels
    pub labels: Option<Vec<String>>,
    pub label_map: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,
    pub model_inputs: Option<Vec<ModelIO>>,
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    pub task_types: Option<Vec<String>>,
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

impl TryFrom<requests::ModelDiscoveryCriteria> for ModelDiscoveryCriteria {
    type Error = Error;
    
    fn try_from(value: requests::ModelDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware
            .map(|hardware| HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware
            .map(|hardware| HardwareRequirements::try_from(hardware))
            .transpose()?;

        Ok(Self {
            name: value.name,
            framework: value.framework,
            model_type: value.model_type,
            version: value.version,
            image: value.image,
            labels: value.labels,
            label_map: value.label_map,
            multi_modal: value.multi_modal,
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: value.task_types,
            inference_precision: value.inference_precision,
            inference_hardware,
            inference_software_dependencies: value.inference_software_dependencies,
            inference_max_energy_consumption_watts: value.inference_max_energy_consumption_watts,
            inference_max_latency_ms: value.inference_max_latency_ms,
            inference_min_throughput: value.inference_min_throughput,
            inference_max_compute_utilization_percentage: value.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: value.inference_max_memory_usage_mb,
            inference_distributed: value.inference_distributed,
            training_time: value.training_time,
            training_precision: value.training_precision,
            training_hardware,
            pretraining_datasets: value.pretraining_datasets,
            finetuning_datasets: value.finetuning_datasets,
            edge_optimized: value.edge_optimized,
            quantization_aware: value.quantization_aware,
            supports_quantization: value.supports_quantization,
            pretrained: value.pretrained,
            pruned: value.pruned,
            slimmed: value.slimmed,
            training_distributed: value.training_distributed,
            training_max_energy_consumption_watts: value.training_max_energy_consumption_watts,
            regulatory: value.regulatory,
            license: value.license,
            bias_evaluation_score: value.bias_evaluation_score,

        })
    }
}