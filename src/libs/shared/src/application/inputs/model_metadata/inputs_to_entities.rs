use crate::application::inputs::model_metadata as inputs;
use crate::domain::entities::model_metadata as domain;
use crate::application::errors::ApplicationError;

impl TryFrom<inputs::SystemRequirement> for domain::SystemRequirement {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}


impl TryFrom<inputs::Accelerator> for domain::Accelerator {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<domain::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(domain::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

impl TryFrom<inputs::HardwareRequirements> for domain::HardwareRequirements {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<domain::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(domain::Accelerator::try_from(accelerator)?);
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

impl TryFrom<inputs::ModelIO> for domain::ModelIO {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<inputs::ModelMetadata> for domain::ModelMetadata {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::ModelMetadata) -> Result<Self, Self::Error> {
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(domain::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(domain::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware
            .map(|hardware| domain::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware
            .map(|hardware| domain::HardwareRequirements::try_from(hardware))
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

impl TryFrom<inputs::CreateModelMetadata> for domain::ModelMetadata {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::CreateModelMetadata) -> Result<Self, Self::Error> {
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.metadata.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(domain::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.metadata.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(domain::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.metadata.inference_hardware
            .map(|hardware| domain::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.metadata.training_hardware
            .map(|hardware| domain::HardwareRequirements::try_from(hardware))
            .transpose()?;

        Ok(Self {
            name: value.metadata.name,
            framework: value.metadata.framework,
            model_type: value.metadata.model_type,
            version: value.metadata.version,
            image: value.metadata.image,
            labels: value.metadata.labels,
            label_map: value.metadata.label_map,
            multi_modal: value.metadata.multi_modal,
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: value.metadata.task_types,
            inference_precision: value.metadata.inference_precision,
            inference_hardware,
            inference_software_dependencies: value.metadata.inference_software_dependencies,
            inference_max_energy_consumption_watts: value.metadata.inference_max_energy_consumption_watts,
            inference_max_latency_ms: value.metadata.inference_max_latency_ms,
            inference_min_throughput: value.metadata.inference_min_throughput,
            inference_max_compute_utilization_percentage: value.metadata.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: value.metadata.inference_max_memory_usage_mb,
            inference_distributed: value.metadata.inference_distributed,
            training_time: value.metadata.training_time,
            training_precision: value.metadata.training_precision,
            training_hardware,
            pretraining_datasets: value.metadata.pretraining_datasets,
            finetuning_datasets: value.metadata.finetuning_datasets,
            edge_optimized: value.metadata.edge_optimized,
            quantization_aware: value.metadata.quantization_aware,
            supports_quantization: value.metadata.supports_quantization,
            pretrained: value.metadata.pretrained,
            pruned: value.metadata.pruned,
            slimmed: value.metadata.slimmed,
            training_distributed: value.metadata.training_distributed,
            training_max_energy_consumption_watts: value.metadata.training_max_energy_consumption_watts,
            regulatory: value.metadata.regulatory,
            license: value.metadata.license,
            bias_evaluation_score: value.metadata.bias_evaluation_score,

        })
    }
}