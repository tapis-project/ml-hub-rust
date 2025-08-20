use crate::infra::persistence::mongo::documents::model_metadata;
use crate::application::inputs::model_metadata as inputs;
use crate::errors::Error;
use mongodb::bson::Uuid;

impl TryFrom<inputs::SystemRequirement> for model_metadata::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: inputs::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<inputs::Accelerator> for model_metadata::Accelerator {
    type Error = Error;
    
    fn try_from(value: inputs::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<model_metadata::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(model_metadata::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

impl TryFrom<inputs::HardwareRequirements> for model_metadata::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: inputs::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<model_metadata::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(model_metadata::Accelerator::try_from(accelerator)?);
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

impl TryFrom<inputs::ModelIO> for model_metadata::ModelIO {
    type Error = Error;
    
    fn try_from(value: inputs::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<&inputs::CreateModelMetadata> for model_metadata::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: &inputs::CreateModelMetadata) -> Result<Self, Self::Error> {
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.metadata.model_inputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(model_metadata::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.metadata.model_outputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(model_metadata::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.metadata.inference_hardware.clone()
            .map(|hardware| model_metadata::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.metadata.training_hardware.clone()
            .map(|hardware| model_metadata::HardwareRequirements::try_from(hardware))
            .transpose()?;

        Ok(Self {
            _id: None,
            artifact_id: Uuid::from_bytes(value.artifact_id.into_bytes()),
            name: value.metadata.name.clone(),
            framework: value.metadata.framework.clone(),
            model_type: value.metadata.model_type.clone(),
            version: value.metadata.version.clone(),
            image: value.metadata.image.clone(),
            labels: value.metadata.labels.clone(),
            label_map: value.metadata.label_map.clone(),
            multi_modal: value.metadata.multi_modal.clone(),
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: value.metadata.task_types.clone(),
            inference_precision: value.metadata.inference_precision.clone(),
            inference_hardware,
            inference_software_dependencies: value.metadata.inference_software_dependencies.clone(),
            inference_max_energy_consumption_watts: value.metadata.inference_max_energy_consumption_watts,
            inference_max_latency_ms: value.metadata.inference_max_latency_ms,
            inference_min_throughput: value.metadata.inference_min_throughput,
            inference_max_compute_utilization_percentage: value.metadata.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: value.metadata.inference_max_memory_usage_mb,
            inference_distributed: value.metadata.inference_distributed,
            training_time: value.metadata.training_time,
            training_precision: value.metadata.training_precision.clone(),
            training_hardware,
            pretraining_datasets: value.metadata.pretraining_datasets.clone(),
            finetuning_datasets: value.metadata.finetuning_datasets.clone(),
            edge_optimized: value.metadata.edge_optimized,
            quantization_aware: value.metadata.quantization_aware,
            supports_quantization: value.metadata.supports_quantization,
            pretrained: value.metadata.pretrained,
            pruned: value.metadata.pruned,
            slimmed: value.metadata.slimmed,
            training_distributed: value.metadata.training_distributed,
            training_max_energy_consumption_watts: value.metadata.training_max_energy_consumption_watts,
            regulatory: value.metadata.regulatory.clone(),
            license: value.metadata.license.clone(),
            bias_evaluation_score: value.metadata.bias_evaluation_score,

        })
    }
}