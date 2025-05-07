//! Contains conversions between domain entities and request and response dtos
use crate::models::web::v1::dto::ModelDiscoveryCriteria as ModelDiscoveryCriteriaDto;
use crate::inference::domain::entities;
use crate::inference::web::v1::dto;
use crate::errors::Error;

impl TryFrom<entities::Kind> for dto::Kind {
    type Error = Error;
    
    fn try_from(value: entities::Kind) -> Result<Self, Self::Error> {
        match value {
            entities::Kind::InferenceServer => Ok(Self::InferenceServer),
            entities::Kind::InferenceServerDeployment => Ok(Self::InferenceServerDeployment),
            entities::Kind::Interface => Ok(Self::Interface)
        }
    }
}
impl TryFrom<entities::InterfaceType> for dto::InterfaceType {
    type Error = Error;
    
    fn try_from(value: entities::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            entities::InterfaceType::Container => Ok(Self::Container),
            entities::InterfaceType::Model => Ok(Self::Model),
            entities::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}
impl TryFrom<entities::ContainerInterfaceMetadata> for dto::ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: entities::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::ContainerInterface> for dto::ContainerInterface {
    type Error = Error;
    
    fn try_from(value: entities::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = dto::Kind::try_from(value.kind)?;
        if kind != dto::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = dto::InterfaceType::try_from(value.r#type)?;
        if r#type != dto::InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: dto::ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: dto::ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}
impl TryFrom<entities::Protocol> for dto::Protocol {
    type Error = Error;
    
    fn try_from(value: entities::Protocol) -> Result<Self, Self::Error> {
        match value {
            entities::Protocol::Http => Ok(Self::Http),
            entities::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}
impl TryFrom<entities::Port> for dto::Port {
    type Error = Error;
    
    fn try_from(value: entities::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: dto::Protocol::try_from(value.protocol)?
        })
    }
}
impl TryFrom<entities::GpuResourceDefinition> for dto::GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: entities::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}
impl TryFrom<entities::ResourcesDefinition> for dto::ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: entities::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| dto::GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}
impl TryFrom<entities::Resources> for dto::Resources {
    type Error = Error;
    
    fn try_from(value: entities::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| dto::ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| dto::ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}
impl TryFrom<entities::ContainerInterfaceSpec> for dto::ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: entities::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| dto::Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<dto::Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(dto::Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}
impl TryFrom<entities::EndpointLabels> for dto::EndpointLabels {
    type Error = Error;
    
    fn try_from(value: entities::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::OpenApiV3Spec> for dto::OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: entities::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| dto::EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}
impl TryFrom<entities::RestApiInterfaceSpec> for dto::RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: entities::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            entities::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(dto::RestApiInterfaceSpec::OpenApiV3(dto::OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}
impl TryFrom<entities::RestApiInterfaceFormat> for dto::RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: entities::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            entities::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}
impl TryFrom<entities::RestApiInterfaceMetadata> for dto::RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: entities::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::RestApiInterface> for dto::RestApiInterface {
    type Error = Error;
    
    fn try_from(value: entities::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = dto::Kind::try_from(value.kind)?;
        if kind != dto::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = dto::InterfaceType::try_from(value.r#type)?;
        if r#type != dto::InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = dto::RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: dto::RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: dto::RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}
impl TryFrom<entities::ModelInterfaceMetadataSelectors> for dto::ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: entities::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}
impl TryFrom<entities::ModelInterfaceMetadataDiscoveryCriteria> for dto::ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: entities::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut criteria: Vec<ModelDiscoveryCriteriaDto> = Vec::with_capacity(1);
        for criterion in value.criteria {
            criteria.push(ModelDiscoveryCriteriaDto::try_from(criterion)?);
        }
        
        Ok(Self {
            platform: value.platform,
            confidence: value.confidence,
            criteria
        })
    }
}
impl TryFrom<entities::ModelInterfaceMetadata> for dto::ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: entities::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<dto::ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = dto::ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| dto::ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
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
impl TryFrom<entities::ModelInterfaceSpec> for dto::ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: entities::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}
impl TryFrom<entities::ModelInterface> for dto::ModelInterface {
    type Error = Error;
    
    fn try_from(value: entities::ModelInterface) -> Result<Self, Self::Error> {
        let kind = dto::Kind::try_from(value.kind)?;
        if kind != dto::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = dto::InterfaceType::try_from(value.r#type)?;
        if r#type != dto::InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: dto::ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: dto::ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}
impl TryFrom<entities::InferenceServerInterface> for dto::InferenceServerInterface {
    type Error = Error;
    
    fn try_from(value: entities::InferenceServerInterface) -> Result<Self, Self::Error> {
        match value {
            entities::InferenceServerInterface::Container(interface) => {
                let r#type = dto::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != dto::InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(dto::InferenceServerInterface::Container(dto::ContainerInterface::try_from(interface)?))
            },
            entities::InferenceServerInterface::RestApi(interface) => {
                let r#type = dto::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != dto::InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(dto::InferenceServerInterface::RestApi(dto::RestApiInterface::try_from(interface)?))
            },
            entities::InferenceServerInterface::Model(interface) => {
                let r#type = dto::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != dto::InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(dto::InferenceServerInterface::Model(dto::ModelInterface::try_from(interface)?))
            },
        }
    }
}
impl TryFrom<entities::InferenceServerMetadata> for dto::InferenceServerMetadata {
    type Error = Error;
    
    fn try_from(value: entities::InferenceServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}
impl TryFrom<entities::InferenceServerSpec> for dto::InferenceServerSpec {
    type Error = Error;
    
    fn try_from(value: entities::InferenceServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<dto::InferenceServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(dto::InferenceServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}
impl TryFrom<entities::InferenceServer> for dto::InferenceServer {
    type Error = Error;
    
    fn try_from(value: entities::InferenceServer) -> Result<Self, Self::Error> {
        let kind = dto::Kind::try_from(value.kind)?;
        if kind != dto::Kind::InferenceServer {
            return Err(Error::from_str("Field 'kind' on InferenceServer must be variant Kind::InferenceServer"));
        }

        Ok(Self {
            kind,
            metadata: dto::InferenceServerMetadata::try_from(value.metadata)?,
            spec: dto::InferenceServerSpec::try_from(value.spec)?
        })
    }
}