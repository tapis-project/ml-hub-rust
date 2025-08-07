use serde_json::to_vec;
use crate::presentation::http::v1::dto::models as dto;
use crate::application::inputs::model_metadata as inputs;
use crate::application::inputs::artifacts as artifact_inputs;
use crate::errors::Error;
use uuid::Uuid;

impl TryFrom<dto::SystemRequirement> for inputs::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: dto::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<dto::Accelerator> for inputs::Accelerator {
    type Error = Error;
    
    fn try_from(value: dto::Accelerator) -> Result<Self, Self::Error> {
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

impl TryFrom<dto::HardwareRequirements> for inputs::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: dto::HardwareRequirements) -> Result<Self, Self::Error> {
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

impl TryFrom<dto::ModelIO> for inputs::ModelIO {
    type Error = Error;
    
    fn try_from(value: dto::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<dto::CreateModelMetadata> for inputs::CreateModelMetadata {
    type Error = Error;

    fn try_from(value: dto::CreateModelMetadata) -> Result<Self, Self::Error> {
        let metadata = inputs::ModelMetadata::try_from(value.metadata)?;
        let artifact_id = Uuid::parse_str(&value.artifact_id)
            .map_err(|err| Self::Error::new(err.to_string()))?;
        
        return Ok(Self {
            artifact_id,
            metadata
        })
    }
}

impl TryFrom<dto::ModelMetadata> for inputs::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: dto::ModelMetadata) -> Result<Self, Self::Error> {
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

/// Model Ingestion

impl TryFrom<dto::IngestModelRequest> for artifact_inputs::IngestArtifactInput {
    type Error = Error;
    fn try_from(value: dto::IngestModelRequest) -> Result<Self, Self::Error> {
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

impl TryFrom<dto::UploadModelRequest> for artifact_inputs::UploadArtifactInput {
    type Error = Error;
    fn try_from(_value: dto::UploadModelRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact_type: artifact_inputs::ArtifactType::Model
        })
    }
}

impl TryFrom<dto::DownloadModelRequest> for artifact_inputs::DownloadArtifactInput {
    type Error = Error;
    fn try_from(value: dto::DownloadModelRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact_type: artifact_inputs::ArtifactType::Model,
            artifact_id: value.path.artifact_id.clone()
        })
    }
}