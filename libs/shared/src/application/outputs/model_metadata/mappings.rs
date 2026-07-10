use platforms::Platform;

use crate::domain::entities::deployment_strategy::strategy::Strategy;
use crate::domain::entities::model_metadata as entities;
use crate::application::outputs::model_metadata as outputs;
use crate::errors::Error;

impl TryFrom<(&entities::ModelMetadata, &Vec<Strategy>)> for outputs::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: (&entities::ModelMetadata, &Vec<Strategy>)) -> Result<Self, Self::Error> {      
        let (model, strategies) = value;

        let mut model_inputs = Vec::with_capacity(1);
        for input in model.model_inputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(outputs::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for outputs in model.model_outputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(outputs::ModelIO::try_from(outputs)?)
        }

        let inference_hardware = model.inference_hardware.clone()
            .map(|hardware| outputs::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = model.training_hardware.clone()
            .map(|hardware| outputs::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let canonical = model.canonical
            .clone()
            .map(|c| outputs::Canonical::from(c));

        // Hashmap for O(1) lookup
        let strategy_description_map: std::collections::HashMap<String, Option<String>> = strategies
            .iter()
            .map(|s| (format_strategy_description_key(&s.platform, &s.name), s.description.clone()))
            .collect();

        let deployment_strategy_refs: Vec<outputs::DeploymentStrategyReference> = model.deployment_strategy_refs
            .clone()
            .into_iter()
            .map(|r| outputs::DeploymentStrategyReference {
                name: r.name.clone(),
                platform: r.platform.clone(),
                description: strategy_description_map
                    .get(&format_strategy_description_key(&r.platform, &r.name))
                    .and_then(|k| k.clone()),
            })
            .collect();

        Ok(Self {
            name: model.name.clone(),
            author: model.author.clone(),
            artifact_id: model.artifact_id.clone(),
            description: model.description.clone(),
            canonical,
            libraries: model.libraries.clone(),
            model_type: model.model_type.clone(),
            tenant_id: model.tenant_id.clone(),
            image: model.image.clone(),
            tags: model.tags.clone(),
            annotations: model.annotations.clone(),
            multi_modal: model.multi_modal.clone(),
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: model.task_types.clone(),
            inference_precision: model.inference_precision.clone(),
            inference_hardware,
            inference_software_dependencies: model.inference_software_dependencies.clone(),
            inference_max_energy_consumption_watts: model.inference_max_energy_consumption_watts,
            inference_max_latency_ms: model.inference_max_latency_ms,
            inference_min_throughput: model.inference_min_throughput,
            inference_max_compute_utilization_percentage: model.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: model.inference_max_memory_usage_mb,
            inference_distributed: model.inference_distributed,
            training_time: model.training_time,
            training_precision: model.training_precision.clone(),
            training_hardware,
            pretraining_datasets: model.pretraining_datasets.clone(),
            finetuning_datasets: model.finetuning_datasets.clone(),
            edge_optimized: model.edge_optimized,
            quantization_aware: model.quantization_aware,
            supports_quantization: model.supports_quantization,
            pretrained: model.pretrained,
            pruned: model.pruned,
            slimmed: model.slimmed,
            training_distributed: model.training_distributed,
            training_max_energy_consumption_watts: model.training_max_energy_consumption_watts,
            regulatory: model.regulatory.clone(),
            license: model.license.clone(),
            bias_evaluation_score: model.bias_evaluation_score,
            deployment_strategy_refs,
        })
    }
}

impl From<entities::Canonical> for outputs::Canonical {
    fn from(value: entities::Canonical) -> Self {
        Self {
            platform: value.platform,
            model_id: value.model_id,
            locator: outputs::Locator::from(value.locator),
            author: value.author,
            likes: value.likes,
            downloads: value.likes,
            gated: value.gated,
            private: value.private,
            sha: value.sha
        }
    }
}

impl From<entities::Locator> for outputs::Locator {
    fn from(value: entities::Locator) -> Self {
        Self { url: value.url }
    }
}

impl TryFrom<entities::SystemRequirement> for outputs::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: entities::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<entities::Accelerator> for outputs::Accelerator {
    type Error = Error;
    
    fn try_from(value: entities::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<outputs::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(outputs::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements,
        })
    }
}

impl TryFrom<entities::HardwareRequirements> for outputs::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: entities::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<outputs::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(outputs::Accelerator::try_from(accelerator)?);
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

impl TryFrom<entities::ModelIO> for outputs::ModelIO {
    type Error = Error;
    
    fn try_from(value: entities::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

// Helper - Formatter
fn format_strategy_description_key(platform: &Platform, name: &String) -> String {
    format!("{}:{}", platform, name)
}