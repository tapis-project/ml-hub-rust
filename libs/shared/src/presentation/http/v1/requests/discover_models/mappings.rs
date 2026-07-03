//! This module contains mappings of this request's structs to application-layer input structs

use crate::presentation::http::v1::requests::discover_models;
use crate::application::inputs::discover_models as inputs;
use crate::application::inputs::task as input_task;
use crate::errors::Error;

impl TryFrom<discover_models::SystemRequirement> for inputs::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: discover_models::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<discover_models::Accelerator> for inputs::Accelerator {
    type Error = Error;
    
    fn try_from(value: discover_models::Accelerator) -> Result<Self, Self::Error> {
        let system_requirements: Option<Vec<inputs::SystemRequirement>> = match value.system_requirements {
            Some(sr) => {
                let mut reqs: Vec<inputs::SystemRequirement> = Vec::with_capacity(1);
                for requirement in sr {
                    reqs.push(inputs::SystemRequirement::try_from(requirement)?);
                }

                Some(reqs)
            },
            None => None
        };

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements,
        })
    }
}

impl TryFrom<discover_models::HardwareRequirements> for inputs::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: discover_models::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<inputs::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(inputs::Accelerator::try_from(accelerator)?);
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

impl TryFrom<discover_models::ModelIO> for inputs::ModelIO {
    type Error = Error;
    
    fn try_from(value: discover_models::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<&discover_models::DiscoveryCriterion> for inputs::SearchCriterion {
    type Error = Error;
    
    fn try_from(value: &discover_models::DiscoveryCriterion) -> Result<Self, Self::Error> {
        let mut task_types: Vec<input_task::Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(input_task::Task::from(task_type))
        }

        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(inputs::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(inputs::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware.clone()
            .map(|hardware| inputs::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware.clone()
            .map(|hardware| inputs::HardwareRequirements::try_from(hardware))
            .transpose()?;

        Ok(Self {
            name: value.name.clone(),
            author: value.author.clone(),
            libraries: value.libraries.clone(),
            model_type: value.model_type.clone(),
            version: value.version.clone(),
            image: value.image.clone(),
            tags: value.tags.clone(),
            annotations: value.annotations.clone(),
            multi_modal: value.multi_modal.clone(),
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: Some(task_types),
            inference_precision: value.inference_precision.clone(),
            inference_hardware,
            inference_software_dependencies: value.inference_software_dependencies.clone(),
            inference_max_energy_consumption_watts: value.inference_max_energy_consumption_watts,
            inference_max_latency_ms: value.inference_max_latency_ms,
            inference_min_throughput: value.inference_min_throughput,
            inference_max_compute_utilization_percentage: value.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: value.inference_max_memory_usage_mb,
            inference_distributed: value.inference_distributed,
            training_time: value.training_time,
            training_precision: value.training_precision.clone(),
            training_hardware,
            pretraining_datasets: value.pretraining_datasets.clone(),
            finetuning_datasets: value.finetuning_datasets.clone(),
            edge_optimized: value.edge_optimized,
            quantization_aware: value.quantization_aware,
            supports_quantization: value.supports_quantization,
            pretrained: value.pretrained,
            pruned: value.pruned,
            slimmed: value.slimmed,
            training_distributed: value.training_distributed,
            training_max_energy_consumption_watts: value.training_max_energy_consumption_watts,
            regulatory: value.regulatory.clone(),
            license: value.license.clone(),
            bias_evaluation_score: value.bias_evaluation_score,

        })
    }
}