use serde_json::to_vec;
use crate::presentation::http::v1::requests::models as requests;
use crate::application::inputs::model_metadata as inputs;
use crate::application::inputs::artifacts as artifact_inputs;
use crate::application::inputs::task as input_task;
use crate::errors::Error;
use uuid::Uuid;

impl TryFrom<requests::SystemRequirement> for inputs::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: requests::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<requests::Accelerator> for inputs::Accelerator {
    type Error = Error;
    
    fn try_from(value: requests::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<inputs::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(inputs::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

impl TryFrom<requests::HardwareRequirements> for inputs::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: requests::HardwareRequirements) -> Result<Self, Self::Error> {
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

impl TryFrom<requests::ModelIO> for inputs::ModelIO {
    type Error = Error;
    
    fn try_from(value: requests::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<requests::CreateModelMetadata> for inputs::UpsertModelMetadata {
    type Error = Error;

    fn try_from(value: requests::CreateModelMetadata) -> Result<Self, Self::Error> {
        let metadata = inputs::ModelMetadata::try_from(value.metadata)?;
        
        return Ok(Self {
            metadata
        })
    }
}

impl TryFrom<(&String, requests::AssociateModelMetadata)> for inputs::AssociateModelMetadata {
    type Error = Error;

    fn try_from(value: (&String, requests::AssociateModelMetadata)) -> Result<Self, Self::Error> {  
        let artifact_id= match Uuid::parse_str(&value.0) {
            Ok(uuid) => uuid,
            Err(_) => return Err(Error::new("Value provided for artifact_id is not a UUID".into()))
        };

        Ok(
            Self {
                artifact_id,
                name: value.1.name,
                author: value.1.author,
            }
        )
    }
}

impl TryFrom<requests::ModelMetadata> for inputs::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: requests::ModelMetadata) -> Result<Self, Self::Error> {
        let mut task_types: Vec<input_task::Task> = Vec::new();
        for task_type in value.task_types.clone().unwrap_or(Vec::with_capacity(0)) {
            task_types.push(input_task::Task::from(task_type))
        }

        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(inputs::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(inputs::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware
            .map(|hardware| inputs::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware
            .map(|hardware| inputs::HardwareRequirements::try_from(hardware))
            .transpose()?;

        Ok(Self {
            name: value.name,
            author: value.author,
            tenant_id: value.tenant_id,
            canonical: None,
            libraries: value.libraries,
            model_type: value.model_type,
            image: value.image,
            keywords: value.keywords,
            annotations: value.annotations,
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

impl TryFrom<requests::IngestModelRequest> for artifact_inputs::IngestArtifactInput {
    type Error = Error;
    fn try_from(value: requests::IngestModelRequest) -> Result<Self, Self::Error> {
        let serialized_client_request = to_vec(&value)
            .map_err(|err| Error::new(format!("Failed serialize the full client request: {}", err.to_string())))?;
        
        Ok(Self {
            artifact_type: artifact_inputs::ArtifactType::Model,
            platform: value.path.platform,
            platform_artifact_id: value.path.model_id,
            webhook_url: value.body.webhook_url,
            serialized_client_request
        })
    }
}

impl TryFrom<requests::UploadModelRequest> for artifact_inputs::UploadArtifactInput {
    type Error = Error;
    fn try_from(_value: requests::UploadModelRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact_type: artifact_inputs::ArtifactType::Model
        })
    }
}

impl TryFrom<requests::DownloadModelRequest> for artifact_inputs::DownloadArtifactInput {
    type Error = Error;
    fn try_from(value: requests::DownloadModelRequest) -> Result<Self, Self::Error> {
        let artifact_id= match Uuid::parse_str(&value.path.artifact_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(Error::new("Value provided for artifact_id is not a UUID".into()))
        };
        
        Ok(Self {
            artifact_type: artifact_inputs::ArtifactType::Model,
            artifact_id
        })
    }
}