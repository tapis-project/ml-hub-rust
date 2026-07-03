use crate::infra::persistence::mongo::documents::model_metadata_filter;
use crate::application::inputs::discover_models as inputs;
use crate::infra::persistence::mongo::documents::task as document_task;
use crate::errors::Error;

use mongodb::bson::doc;

impl TryFrom<inputs::SystemRequirement> for model_metadata_filter::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: inputs::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<inputs::Accelerator> for model_metadata_filter::Accelerator {
    type Error = Error;
    
    fn try_from(value: inputs::Accelerator) -> Result<Self, Self::Error> {
        let system_requirements: Option<Vec<model_metadata_filter::SystemRequirement>> = match value.system_requirements {
            Some(sr) => {
                let mut reqs: Vec<model_metadata_filter::SystemRequirement> = Vec::with_capacity(1);
                for requirement in sr {
                    reqs.push(model_metadata_filter::SystemRequirement::try_from(requirement)?);
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

impl TryFrom<inputs::HardwareRequirements> for model_metadata_filter::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: inputs::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<model_metadata_filter::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(model_metadata_filter::Accelerator::try_from(accelerator)?);
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

impl TryFrom<inputs::ModelIO> for model_metadata_filter::ModelIO {
    type Error = Error;
    
    fn try_from(value: inputs::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<(&inputs::SearchCriterion, &Vec<String>)> for model_metadata_filter::ModelMetadataFilter {
    type Error = Error;
    fn try_from(value: (&inputs::SearchCriterion, &Vec<String>)) -> Result<Self, Self::Error> {
        let search_criteria = value.0;
        let mut task_types: Vec<document_task::Task> = Vec::new();
        for task_type in search_criteria.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(document_task::Task::from(task_type))
        }

        let mut model_inputs = Vec::with_capacity(1);
        for input in search_criteria.model_inputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(model_metadata_filter::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in search_criteria.model_outputs.clone().unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(model_metadata_filter::ModelIO::try_from(output)?)
        }

        let inference_hardware = search_criteria.inference_hardware.clone()
            .map(|hardware| model_metadata_filter::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = search_criteria.training_hardware.clone()
            .map(|hardware| model_metadata_filter::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let tenant_ids = value.1;
        let mut tenancy_selector = Some(
            doc! {
                "$in": tenant_ids
            }
        );

        // If no tenants are provided
        if tenant_ids.len() == 0 {
            tenancy_selector = None;
        }

        Ok(Self {
            name: search_criteria.name.clone(),
            author: search_criteria.author.clone(),
            tenant_id: tenancy_selector,
            libraries: search_criteria.libraries.clone(),
            model_type: search_criteria.model_type.clone(),
            version: search_criteria.version.clone(),
            image: search_criteria.image.clone(),
            tags: search_criteria.tags.clone(),
            annotations: search_criteria.annotations.clone(),
            multi_modal: search_criteria.multi_modal.clone(),
            model_inputs: Some(model_inputs),
            model_outputs: Some(model_outputs),
            task_types: Some(task_types),
            inference_precision: search_criteria.inference_precision.clone(),
            inference_hardware,
            inference_software_dependencies: search_criteria.inference_software_dependencies.clone(),
            inference_max_energy_consumption_watts: search_criteria.inference_max_energy_consumption_watts,
            inference_max_latency_ms: search_criteria.inference_max_latency_ms,
            inference_min_throughput: search_criteria.inference_min_throughput,
            inference_max_compute_utilization_percentage: search_criteria.inference_max_compute_utilization_percentage,
            inference_max_memory_usage_mb: search_criteria.inference_max_memory_usage_mb,
            inference_distributed: search_criteria.inference_distributed,
            training_time: search_criteria.training_time,
            training_precision: search_criteria.training_precision.clone(),
            training_hardware,
            pretraining_datasets: search_criteria.pretraining_datasets.clone(),
            finetuning_datasets: search_criteria.finetuning_datasets.clone(),
            edge_optimized: search_criteria.edge_optimized,
            quantization_aware: search_criteria.quantization_aware,
            supports_quantization: search_criteria.supports_quantization,
            pretrained: search_criteria.pretrained,
            pruned: search_criteria.pruned,
            slimmed: search_criteria.slimmed,
            training_distributed: search_criteria.training_distributed,
            training_max_energy_consumption_watts: search_criteria.training_max_energy_consumption_watts,
            regulatory: search_criteria.regulatory.clone(),
            license: search_criteria.license.clone(),
            bias_evaluation_score: search_criteria.bias_evaluation_score,
        })
    }
}