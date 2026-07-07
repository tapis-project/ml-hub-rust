use crate::infra::persistence::mongo::documents::model_metadata;
use crate::infra::persistence::mongo::documents::task as document_task;
use crate::application::inputs::model_metadata as inputs;
use crate::errors::Error;
use crate::shared_kernal::identity::IdentityContext;

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

impl TryFrom<inputs::Locator> for model_metadata::Locator {
    type Error = Error;
    
    fn try_from(value: inputs::Locator) -> Result<Self, Self::Error> {
        Ok(Self {
            url: value.url  
        })
    }
}


impl TryFrom<inputs::Canonical> for model_metadata::Canonical {
    type Error = Error;
    
    fn try_from(value: inputs::Canonical) -> Result<Self, Self::Error> {
        Ok(Self {
            platform: value.platform.to_string(),
            model_id: value.model_id,
            locator: model_metadata::Locator::try_from(value.locator)?,
            author: value.author,
            likes: value.likes.map(|v| v as u64),
            downloads: value.downloads.map(|v| v as u64),
            gated: value.gated,
            private: value.private,
            sha: value.sha,
        })
    }
}


impl TryFrom<(&inputs::ModelMetadata, &IdentityContext)> for model_metadata::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: (&inputs::ModelMetadata, &IdentityContext)) -> Result<Self, Self::Error> {

        let mut task_types: Vec<document_task::Task> = Vec::new();
        for task_type in value.0.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(document_task::Task::from(task_type))
        }

        let mut model_inputs = Vec::with_capacity(1);
        for input in value.0.model_inputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(model_metadata::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.0.model_outputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(model_metadata::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.0.inference_hardware.clone()
            .map(|hardware| model_metadata::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.0.training_hardware.clone()
            .map(|hardware| model_metadata::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let canonical = value.0.canonical
            .clone()
            .map(|v| model_metadata::Canonical::try_from(v))
            .transpose()?;

        let tenant_id = value.1.actor_tenant_id().clone();

        let author = value.1.actor_principal_id().clone();

        Ok(Self {
            _id: None,
            artifact_id: None,
            description: value.0.description.clone(),
            author,
            tenant_id,
            canonical,
            name: value.0.name.clone(),
            libraries: value.0.libraries.clone(),
            model_type: value.0.model_type.clone(),
            image: value.0.image.clone(),
            tags: value.0.tags.clone(),
            annotations: value.0.annotations.clone(),
            multi_modal: value.0.multi_modal.clone(),
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: Some(task_types),
            inference_precision: value.0.inference_precision.clone(),
            inference_hardware,
            inference_software_dependencies: value.0.inference_software_dependencies.clone(),
            inference_max_energy_consumption_watts: value.0.inference_max_energy_consumption_watts,
            inference_max_latency_ms: value.0.inference_max_latency_ms,
            inference_min_throughput: value.0.inference_min_throughput,
            inference_max_compute_utilization_percentage: value.0.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: value.0.inference_max_memory_usage_mb,
            inference_distributed: value.0.inference_distributed,
            training_time: value.0.training_time,
            training_precision: value.0.training_precision.clone(),
            training_hardware,
            pretraining_datasets: value.0.pretraining_datasets.clone(),
            finetuning_datasets: value.0.finetuning_datasets.clone(),
            edge_optimized: value.0.edge_optimized,
            quantization_aware: value.0.quantization_aware,
            supports_quantization: value.0.supports_quantization,
            pretrained: value.0.pretrained,
            pruned: value.0.pruned,
            slimmed: value.0.slimmed,
            training_distributed: value.0.training_distributed,
            training_max_energy_consumption_watts: value.0.training_max_energy_consumption_watts,
            regulatory: value.0.regulatory.clone(),
            license: value.0.license.clone(),
            bias_evaluation_score: value.0.bias_evaluation_score,
        })
    }
}