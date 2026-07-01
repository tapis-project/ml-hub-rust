use crate::application::inputs::model_metadata as inputs;
use crate::domain::entities::task as domain_task;
use crate::domain::entities::model_metadata as domain;

use crate::application::errors::ApplicationError;
use crate::shared_kernal::identity::IdentityContext;

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

impl TryFrom<inputs::Locator> for domain::Locator {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::Locator) -> Result<Self, Self::Error> {
        Ok(Self {
            url: value.url  
        })
    }
}


impl TryFrom<inputs::Canonical> for domain::Canonical {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::Canonical) -> Result<Self, Self::Error> {
        Ok(Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: domain::Locator::try_from(value.locator)?,
            author: value.author,
            likes: value.likes,
            downloads: value.downloads,
            gated: value.gated,
            private: value.private,
            sha: value.sha,
        })
    }
}

impl TryFrom<(inputs::ModelMetadata, &IdentityContext)> for domain::ModelMetadata {
    type Error = ApplicationError;
    
    fn try_from(value: (inputs::ModelMetadata, &IdentityContext)) -> Result<Self, Self::Error> {
        let mut task_types: Vec<domain_task::Task> = Vec::new();
        for task_type in value.0.task_types.unwrap_or(Vec::with_capacity(0)) {
            task_types.push(domain_task::Task::from(task_type))
        }
        
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.0.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(domain::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.0.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(domain::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.0.inference_hardware
            .map(|hardware| domain::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.0.training_hardware
            .map(|hardware| domain::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let canonical = value.0.canonical
            .map(|v| domain::Canonical::try_from(v))
            .transpose()?;
        
        Ok(Self {
            name: value.0.name,
            author: value.1.actor_principal_id().clone(),
            tenant_id: value.1.actor_tenant_id().clone(),
            artifact_id: None,
            canonical,
            libraries: value.0.libraries,
            model_type: value.0.model_type,
            image: value.0.image,
            keywords: value.0.keywords,
            annotations: value.0.annotations,
            multi_modal: value.0.multi_modal,
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: Some(task_types),
            inference_precision: value.0.inference_precision,
            inference_hardware,
            inference_software_dependencies: value.0.inference_software_dependencies,
            inference_max_energy_consumption_watts: value.0.inference_max_energy_consumption_watts,
            inference_max_latency_ms: value.0.inference_max_latency_ms,
            inference_min_throughput: value.0.inference_min_throughput,
            inference_max_compute_utilization_percentage: value.0.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: value.0.inference_max_memory_usage_mb,
            inference_distributed: value.0.inference_distributed,
            training_time: value.0.training_time,
            training_precision: value.0.training_precision,
            training_hardware,
            pretraining_datasets: value.0.pretraining_datasets,
            finetuning_datasets: value.0.finetuning_datasets,
            edge_optimized: value.0.edge_optimized,
            quantization_aware: value.0.quantization_aware,
            supports_quantization: value.0.supports_quantization,
            pretrained: value.0.pretrained,
            pruned: value.0.pruned,
            slimmed: value.0.slimmed,
            training_distributed: value.0.training_distributed,
            training_max_energy_consumption_watts: value.0.training_max_energy_consumption_watts,
            regulatory: value.0.regulatory,
            license: value.0.license,
            bias_evaluation_score: value.0.bias_evaluation_score,

        })
    }
}