//! Contains conversions between domain entities and request and response dtos
use crate::presentation::http::v1::requests::models::ModelMetadata as ModelMetadataDto;
use crate::domain::entities::deployments as entities;
use crate::presentation::http::v1::requests::deployments as requests;
use crate::errors::Error;

impl TryFrom<entities::Kind> for requests::Kind {
    type Error = Error;
    
    fn try_from(value: entities::Kind) -> Result<Self, Self::Error> {
        match value {
            entities::Kind::ModelServer => Ok(Self::ModelServer),
            entities::Kind::ModelServerDeployment => Ok(Self::ModelServerDeployment),
            entities::Kind::Interface => Ok(Self::Interface)
        }
    }
}
impl TryFrom<entities::InterfaceType> for requests::InterfaceType {
    type Error = Error;
    
    fn try_from(value: entities::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            entities::InterfaceType::Container => Ok(Self::Container),
            entities::InterfaceType::Model => Ok(Self::Model),
            entities::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}
impl TryFrom<entities::ContainerInterfaceMetadata> for requests::ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: entities::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::ContainerInterface> for requests::ContainerInterface {
    type Error = Error;
    
    fn try_from(value: entities::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = requests::Kind::try_from(value.kind)?;
        if kind != requests::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = requests::InterfaceType::try_from(value.r#type)?;
        if r#type != requests::InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: requests::ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: requests::ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}
impl TryFrom<entities::Protocol> for requests::Protocol {
    type Error = Error;
    
    fn try_from(value: entities::Protocol) -> Result<Self, Self::Error> {
        match value {
            entities::Protocol::Http => Ok(Self::Http),
            entities::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}
impl TryFrom<entities::Port> for requests::Port {
    type Error = Error;
    
    fn try_from(value: entities::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: requests::Protocol::try_from(value.protocol)?
        })
    }
}
impl TryFrom<entities::GpuResourceDefinition> for requests::GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: entities::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}
impl TryFrom<entities::ResourcesDefinition> for requests::ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: entities::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| requests::GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}
impl TryFrom<entities::Resources> for requests::Resources {
    type Error = Error;
    
    fn try_from(value: entities::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| requests::ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| requests::ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}
impl TryFrom<entities::ContainerInterfaceSpec> for requests::ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: entities::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| requests::Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<requests::Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(requests::Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}
impl TryFrom<entities::EndpointLabels> for requests::EndpointLabels {
    type Error = Error;
    
    fn try_from(value: entities::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::OpenApiV3Spec> for requests::OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: entities::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| requests::EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}
impl TryFrom<entities::RestApiInterfaceSpec> for requests::RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: entities::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            entities::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(requests::RestApiInterfaceSpec::OpenApiV3(requests::OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}
impl TryFrom<entities::RestApiInterfaceFormat> for requests::RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: entities::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            entities::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}
impl TryFrom<entities::RestApiInterfaceMetadata> for requests::RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: entities::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::RestApiInterface> for requests::RestApiInterface {
    type Error = Error;
    
    fn try_from(value: entities::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = requests::Kind::try_from(value.kind)?;
        if kind != requests::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = requests::InterfaceType::try_from(value.r#type)?;
        if r#type != requests::InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = requests::RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: requests::RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: requests::RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}
impl TryFrom<entities::ModelInterfaceMetadataSelectors> for requests::ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: entities::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}
impl TryFrom<entities::ModelInterfaceMetadataDiscoveryCriteria> for requests::ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: entities::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut criteria: Vec<ModelMetadataDto> = Vec::with_capacity(1);
        for criterion in value.criteria {
            criteria.push(ModelMetadataDto::try_from(criterion)?);
        }
        
        Ok(Self {
            platform: value.platform,
            confidence: value.confidence,
            criteria
        })
    }
}
impl TryFrom<entities::ModelInterfaceMetadata> for requests::ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: entities::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<requests::ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = requests::ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| requests::ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
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
impl TryFrom<entities::ModelInterfaceSpec> for requests::ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: entities::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}
impl TryFrom<entities::ModelInterface> for requests::ModelInterface {
    type Error = Error;
    
    fn try_from(value: entities::ModelInterface) -> Result<Self, Self::Error> {
        let kind = requests::Kind::try_from(value.kind)?;
        if kind != requests::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = requests::InterfaceType::try_from(value.r#type)?;
        if r#type != requests::InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: requests::ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: requests::ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}
impl TryFrom<entities::ModelServerInterface> for requests::ModelServerInterface {
    type Error = Error;
    
    fn try_from(value: entities::ModelServerInterface) -> Result<Self, Self::Error> {
        match value {
            entities::ModelServerInterface::Container(interface) => {
                let r#type = requests::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != requests::InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(requests::ModelServerInterface::Container(requests::ContainerInterface::try_from(interface)?))
            },
            entities::ModelServerInterface::RestApi(interface) => {
                let r#type = requests::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != requests::InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(requests::ModelServerInterface::RestApi(requests::RestApiInterface::try_from(interface)?))
            },
            entities::ModelServerInterface::Model(interface) => {
                let r#type = requests::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != requests::InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(requests::ModelServerInterface::Model(requests::ModelInterface::try_from(interface)?))
            },
        }
    }
}
impl TryFrom<entities::ModelServerMetadata> for requests::ModelServerMetadata {
    type Error = Error;
    
    fn try_from(value: entities::ModelServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::ModelServerSpec> for requests::ModelServerSpec {
    type Error = Error;
    
    fn try_from(value: entities::ModelServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<requests::ModelServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(requests::ModelServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}
impl TryFrom<entities::ModelServer> for requests::ModelServer {
    type Error = Error;
    
    fn try_from(value: entities::ModelServer) -> Result<Self, Self::Error> {
        let kind = requests::Kind::try_from(value.kind)?;
        if kind != requests::Kind::ModelServer {
            return Err(Error::from_str("Field 'kind' on ModelServer must be variant Kind::ModelServer"));
        }

        Ok(Self {
            kind,
            metadata: requests::ModelServerMetadata::try_from(value.metadata)?,
            spec: requests::ModelServerSpec::try_from(value.spec)?
        })
    }
}