use crate::application::outputs::model_metadata as output;
use crate::presentation::http::v1::responses::models as responses;
use crate::presentation::http::v1::responses::tasks::Task;
use crate::errors::Error;

impl TryFrom<&output::ModelMetadata> for responses::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: &output::ModelMetadata) -> Result<Self, Self::Error> {

        let mut task_types: Vec<Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(Task::from(task_type))
        }

        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(responses::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(responses::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware.clone()
            .map(|hardware| responses::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware.clone()
            .map(|hardware| responses::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let canonical = value.canonical
            .clone()
            .map(|c| responses::Canonical::from(c));

        let deployment_strategy_refs: Vec<responses::DeploymentStrategyReference> = value.deployment_strategy_refs
            .clone()
            .into_iter()
            .map(|r| responses::DeploymentStrategyReference {
                name: r.name.clone(),
                platform: r.platform.clone(),
                description: r.description.clone(),
            })
            .collect();

        Ok(Self {
            name: value.name.clone(),
            author: value.author.clone(),
            description: value.description.clone(),
            canonical,
            libraries: value.libraries.clone(),
            model_type: value.model_type.clone(),
            tenant_id: value.tenant_id.clone(),
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
            deployment_strategy_refs,
        })
    }
}

impl From<output::Canonical> for responses::Canonical {
    fn from(value: output::Canonical) -> Self {
        Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: responses::Locator::from(value.locator),
            author: value.author,
            likes: value.likes,
            downloads: value.likes,
            gated: value.gated,
            private: value.private,
            sha: value.sha
        }
    }
}

impl From<output::Locator> for responses::Locator {
    fn from(value: output::Locator) -> Self {
        Self { url: value.url }
    }
}

impl TryFrom<output::SystemRequirement> for responses::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: output::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<output::Accelerator> for responses::Accelerator {
    type Error = Error;
    
    fn try_from(value: output::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<responses::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(responses::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements,
        })
    }
}

impl TryFrom<output::HardwareRequirements> for responses::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: output::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<responses::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(responses::Accelerator::try_from(accelerator)?);
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

impl TryFrom<output::ModelIO> for responses::ModelIO {
    type Error = Error;
    
    fn try_from(value: output::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}