use crate::domain::entities as domain;
use crate::infra::db::mongo::documents;
use shared::errors::Error;


impl TryFrom<documents::Kind> for domain::Kind {
    type Error = Error;
    
    fn try_from(value: documents::Kind) -> Result<Self, Self::Error> {
        match value {
            documents::Kind::InferenceServer => Ok(Self::InferenceServer),
            documents::Kind::InferenceServerDeployment => Ok(Self::InferenceServerDeployment),
            documents::Kind::Interface => Ok(Self::Interface)
        }
    }
}

impl TryFrom<documents::InterfaceType> for domain::InterfaceType {
    type Error = Error;
    
    fn try_from(value: documents::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            documents::InterfaceType::Container => Ok(Self::Container),
            documents::InterfaceType::Model => Ok(Self::Model),
            documents::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}

impl TryFrom<documents::ContainerInterfaceMetadata> for domain::ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: documents::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<documents::ContainerInterface> for domain::ContainerInterface {
    type Error = Error;
    
    fn try_from(value: documents::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = domain::Kind::try_from(value.kind)?;
        if kind != domain::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = domain::InterfaceType::try_from(value.r#type)?;
        if r#type != domain::InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: domain::ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: domain::ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<documents::Protocol> for domain::Protocol {
    type Error = Error;
    
    fn try_from(value: documents::Protocol) -> Result<Self, Self::Error> {
        match value {
            documents::Protocol::Http => Ok(Self::Http),
            documents::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}

impl TryFrom<documents::Port> for domain::Port {
    type Error = Error;
    
    fn try_from(value: documents::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: domain::Protocol::try_from(value.protocol)?
        })
    }
}

impl TryFrom<documents::GpuResourceDefinition> for domain::GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: documents::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}

impl TryFrom<documents::ResourcesDefinition> for domain::ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: documents::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| domain::GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}

impl TryFrom<documents::Resources> for domain::Resources {
    type Error = Error;
    
    fn try_from(value: documents::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| domain::ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| domain::ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}

impl TryFrom<documents::ContainerInterfaceSpec> for domain::ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: documents::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| domain::Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<domain::Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(domain::Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}

impl TryFrom<documents::EndpointLabels> for domain::EndpointLabels {
    type Error = Error;
    
    fn try_from(value: documents::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}

impl TryFrom<documents::OpenApiV3Spec> for domain::OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: documents::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| domain::EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}

impl TryFrom<documents::RestApiInterfaceSpec> for domain::RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: documents::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            documents::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(domain::RestApiInterfaceSpec::OpenApiV3(domain::OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}

impl TryFrom<documents::RestApiInterfaceFormat> for domain::RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: documents::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            documents::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}

impl TryFrom<documents::RestApiInterfaceMetadata> for domain::RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: documents::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<documents::RestApiInterface> for domain::RestApiInterface {
    type Error = Error;
    
    fn try_from(value: documents::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = domain::Kind::try_from(value.kind)?;
        if kind != domain::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = domain::InterfaceType::try_from(value.r#type)?;
        if r#type != domain::InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = domain::RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: domain::RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: domain::RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<documents::ModelInterfaceMetadataSelectors> for domain::ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: documents::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}

/// Converts a ModelInterfaceMetadataDiscoveryCriteria domain entity into a 
/// mongo-specific database entity
impl TryFrom<documents::ModelInterfaceMetadataDiscoveryCriteria> for domain::ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: documents::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut criteria: Vec<domain::ModelDiscoveryCriteria> = Vec::with_capacity(1);
        for criterion in value.criteria {
            criteria.push(domain::ModelDiscoveryCriteria::try_from(criterion)?);
        }
        
        Ok(Self {
            platform: value.platform,
            confidence: value.confidence,
            criteria
        })
    }
}

impl TryFrom<documents::ModelInterfaceMetadata> for domain::ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: documents::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<domain::ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = domain::ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| domain::ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
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

impl TryFrom<documents::ModelInterfaceSpec> for domain::ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: documents::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}

impl TryFrom<documents::ModelInterface> for domain::ModelInterface {
    type Error = Error;
    
    fn try_from(value: documents::ModelInterface) -> Result<Self, Self::Error> {
        let kind = domain::Kind::try_from(value.kind)?;
        if kind != domain::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = domain::InterfaceType::try_from(value.r#type)?;
        if r#type != domain::InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: domain::ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: domain::ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<documents::InferenceServerInterface> for domain::InferenceServerInterface {
    type Error = Error;
    
    fn try_from(value: documents::InferenceServerInterface) -> Result<Self, Self::Error> {
        match value {
            documents::InferenceServerInterface::Container(interface) => {
                let r#type = domain::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != domain::InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(domain::InferenceServerInterface::Container(domain::ContainerInterface::try_from(interface)?))
            },
            documents::InferenceServerInterface::RestApi(interface) => {
                let r#type = domain::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != domain::InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(domain::InferenceServerInterface::RestApi(domain::RestApiInterface::try_from(interface)?))
            },
            documents::InferenceServerInterface::Model(interface) => {
                let r#type = domain::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != domain::InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(domain::InferenceServerInterface::Model(domain::ModelInterface::try_from(interface)?))
            },
        }
    }
}

impl TryFrom<documents::InferenceServerMetadata> for domain::InferenceServerMetadata {
    type Error = Error;
    
    fn try_from(value: documents::InferenceServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<documents::InferenceServerSpec> for domain::InferenceServerSpec {
    type Error = Error;
    
    fn try_from(value: documents::InferenceServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<domain::InferenceServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(domain::InferenceServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}

impl TryFrom<documents::InferenceServer> for domain::InferenceServer {
    type Error = Error;
    
    fn try_from(value: documents::InferenceServer) -> Result<Self, Self::Error> {
        let kind = domain::Kind::try_from(value.kind)?;
        if kind != domain::Kind::InferenceServer {
            return Err(Error::from_str("Field 'kind' on InferenceServer must be variant Kind::InferenceServer"));
        }

        Ok(Self {
            kind,
            metadata: domain::InferenceServerMetadata::try_from(value.metadata)?,
            spec: domain::InferenceServerSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<documents::SystemRequirement> for domain::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: documents::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<documents::Accelerator> for domain::Accelerator {
    type Error = Error;
    
    fn try_from(value: documents::Accelerator) -> Result<Self, Self::Error> {
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

impl TryFrom<documents::HardwareRequirements> for domain::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: documents::HardwareRequirements) -> Result<Self, Self::Error> {
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

impl TryFrom<documents::ModelIO> for domain::ModelIO {
    type Error = Error;
    
    fn try_from(value: documents::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<documents::ModelDiscoveryCriteria> for domain::ModelDiscoveryCriteria {
    type Error = Error;
    
    fn try_from(value: documents::ModelDiscoveryCriteria) -> Result<Self, Self::Error> {
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

impl TryFrom<documents::InferenceServerDeployment> for domain::InferenceServerDeployment {
    type Error = Error;

    // TODO
    fn try_from(_value: documents::InferenceServerDeployment) -> Result<Self, Self::Error> {
        return Ok(Self {

        })
    }
}