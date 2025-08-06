use crate::domain::entities as domain;
use crate::infra::db::mongo::documents;
use shared::errors::Error;


impl TryFrom<domain::Kind> for documents::Kind {
    type Error = Error;
    
    fn try_from(value: domain::Kind) -> Result<Self, Self::Error> {
        match value {
            domain::Kind::InferenceServer => Ok(Self::InferenceServer),
            domain::Kind::InferenceServerDeployment => Ok(Self::InferenceServerDeployment),
            domain::Kind::Interface => Ok(Self::Interface)
        }
    }
}

impl TryFrom<domain::InterfaceType> for documents::InterfaceType {
    type Error = Error;
    
    fn try_from(value: domain::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            domain::InterfaceType::Container => Ok(Self::Container),
            domain::InterfaceType::Model => Ok(Self::Model),
            domain::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}

impl TryFrom<domain::ContainerInterfaceMetadata> for documents::ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: domain::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<domain::ContainerInterface> for documents::ContainerInterface {
    type Error = Error;
    
    fn try_from(value: domain::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = documents::Kind::try_from(value.kind)?;
        if kind != documents::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = documents::InterfaceType::try_from(value.r#type)?;
        if r#type != documents::InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: documents::ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: documents::ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<domain::Protocol> for documents::Protocol {
    type Error = Error;
    
    fn try_from(value: domain::Protocol) -> Result<Self, Self::Error> {
        match value {
            domain::Protocol::Http => Ok(Self::Http),
            domain::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}

impl TryFrom<domain::Port> for documents::Port {
    type Error = Error;
    
    fn try_from(value: domain::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: documents::Protocol::try_from(value.protocol)?
        })
    }
}

impl TryFrom<domain::GpuResourceDefinition> for documents::GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: domain::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}

impl TryFrom<domain::ResourcesDefinition> for documents::ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: domain::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| documents::GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}

impl TryFrom<domain::Resources> for documents::Resources {
    type Error = Error;
    
    fn try_from(value: domain::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| documents::ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| documents::ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}

impl TryFrom<domain::ContainerInterfaceSpec> for documents::ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: domain::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| documents::Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<documents::Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(documents::Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}

impl TryFrom<domain::EndpointLabels> for documents::EndpointLabels {
    type Error = Error;
    
    fn try_from(value: domain::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}

impl TryFrom<domain::OpenApiV3Spec> for documents::OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: domain::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| documents::EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}

impl TryFrom<domain::RestApiInterfaceSpec> for documents::RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: domain::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            domain::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(documents::RestApiInterfaceSpec::OpenApiV3(documents::OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}

impl TryFrom<domain::RestApiInterfaceFormat> for documents::RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: domain::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            domain::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}

impl TryFrom<domain::RestApiInterfaceMetadata> for documents::RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: domain::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<domain::RestApiInterface> for documents::RestApiInterface {
    type Error = Error;
    
    fn try_from(value: domain::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = documents::Kind::try_from(value.kind)?;
        if kind != documents::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = documents::InterfaceType::try_from(value.r#type)?;
        if r#type != documents::InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = documents::RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: documents::RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: documents::RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<domain::ModelInterfaceMetadataSelectors> for documents::ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: domain::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}

/// Converts a ModelInterfaceMetadataDiscoveryCriteria domain entity into a 
/// mongo-specific database entity
impl TryFrom<domain::ModelInterfaceMetadataDiscoveryCriteria> for documents::ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: domain::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut criteria: Vec<documents::ModelMetadata> = Vec::with_capacity(1);
        for criterion in value.criteria {
            criteria.push(documents::ModelMetadata::try_from(criterion)?);
        }
        
        Ok(Self {
            platform: value.platform,
            confidence: value.confidence,
            criteria
        })
    }
}

impl TryFrom<domain::ModelInterfaceMetadata> for documents::ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: domain::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<documents::ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = documents::ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| documents::ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
            .transpose()?;

        Ok(Self {
            name: value.name,
            description: value.description,
            discovery_criteria,
            labels: value.labels,
            selectors: Some(selectors)
        })
    }
}

impl TryFrom<domain::ModelInterfaceSpec> for documents::ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: domain::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}

impl TryFrom<domain::ModelInterface> for documents::ModelInterface {
    type Error = Error;
    
    fn try_from(value: domain::ModelInterface) -> Result<Self, Self::Error> {
        let kind = documents::Kind::try_from(value.kind)?;
        if kind != documents::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = documents::InterfaceType::try_from(value.r#type)?;
        if r#type != documents::InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: documents::ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: documents::ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<domain::InferenceServerInterface> for documents::InferenceServerInterface {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServerInterface) -> Result<Self, Self::Error> {
        match value {
            domain::InferenceServerInterface::Container(interface) => {
                let r#type = documents::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != documents::InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(documents::InferenceServerInterface::Container(documents::ContainerInterface::try_from(interface)?))
            },
            domain::InferenceServerInterface::RestApi(interface) => {
                let r#type = documents::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != documents::InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(documents::InferenceServerInterface::RestApi(documents::RestApiInterface::try_from(interface)?))
            },
            domain::InferenceServerInterface::Model(interface) => {
                let r#type = documents::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != documents::InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(documents::InferenceServerInterface::Model(documents::ModelInterface::try_from(interface)?))
            },
        }
    }
}

impl TryFrom<domain::InferenceServerMetadata> for documents::InferenceServerMetadata {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<domain::InferenceServerSpec> for documents::InferenceServerSpec {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<documents::InferenceServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(documents::InferenceServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}

impl TryFrom<domain::InferenceServer> for documents::InferenceServer {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServer) -> Result<Self, Self::Error> {
        let kind = documents::Kind::try_from(value.kind)?;
        if kind != documents::Kind::InferenceServer {
            return Err(Error::from_str("Field 'kind' on InferenceServer must be variant Kind::InferenceServer"));
        }

        Ok(Self {
            _id: None,
            kind,
            metadata: documents::InferenceServerMetadata::try_from(value.metadata)?,
            spec: documents::InferenceServerSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<domain::SystemRequirement> for documents::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: domain::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<domain::Accelerator> for documents::Accelerator {
    type Error = Error;
    
    fn try_from(value: domain::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<documents::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(documents::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

impl TryFrom<domain::HardwareRequirements> for documents::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: domain::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<documents::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(documents::Accelerator::try_from(accelerator)?);
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

impl TryFrom<domain::ModelIO> for documents::ModelIO {
    type Error = Error;
    
    fn try_from(value: domain::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<domain::ModelMetadata> for documents::ModelMetadata {
    type Error = Error;
    
    fn try_from(value: domain::ModelMetadata) -> Result<Self, Self::Error> {
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(documents::ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(documents::ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware
            .map(|hardware| documents::HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware
            .map(|hardware| documents::HardwareRequirements::try_from(hardware))
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

impl TryFrom<domain::InferenceServerDeployment> for documents::InferenceServerDeployment {
    type Error = Error;

    // TODO
    fn try_from(_value: domain::InferenceServerDeployment) -> Result<Self, Self::Error> {
        return Ok(Self {

        })
    }
}