use crate::presentation::http::v1::requests::models as requests;
use crate::presentation::http::v1::requests::task;
use crate::domain::entities::model_metadata as entities;
use crate::errors::Error;

impl TryFrom<entities::SystemRequirement> for requests::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: entities::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}
impl TryFrom<entities::Accelerator> for requests::Accelerator {
    type Error = Error;
    
    fn try_from(value: entities::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<requests::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(requests::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}
impl TryFrom<entities::HardwareRequirements> for requests::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: entities::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<requests::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(requests::Accelerator::try_from(accelerator)?);
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
impl TryFrom<entities::ModelIO> for requests::ModelIO {
    type Error = Error;
    
    fn try_from(value: entities::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}
impl TryFrom<entities::ModelMetadata> for requests::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: entities::ModelMetadata) -> Result<Self, Self::Error> {
        let mut task_types: Vec<task::Task> = Vec::new();
        for task_type in value.task_types.unwrap_or(Vec::with_capacity(0)) {
            task_types.push(task::Task::from(task_type))
        }

        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(requests::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(requests::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware
            .map(|hardware| requests::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware
            .map(|hardware| requests::HardwareRequirements::try_from(hardware))
            .transpose()?;

        Ok(Self {
            name: value.name,
            author: value.author,
            framework: value.framework,
            model_type: value.model_type,
            image: value.image,
            keywords: value.keywords,
            annotation: value.annotation,
            multi_modal: value.multi_modal,
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: Some(task_types),
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