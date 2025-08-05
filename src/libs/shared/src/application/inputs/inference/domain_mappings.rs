//! Contains conversions between domain entities and request and response dtos
use crate::domain::entities::models::ModelDiscoveryCriteria;
use crate::domain::entities::inference as entities;
use crate::application::inputs::inference as inputs;
use crate::errors::Error;

impl TryFrom<inputs::Kind> for entities::Kind {
    type Error = Error;
    
    fn try_from(value: inputs::Kind) -> Result<Self, Self::Error> {
        match value {
            inputs::Kind::InferenceServer => Ok(Self::InferenceServer),
            inputs::Kind::InferenceServerDeployment => Ok(Self::InferenceServerDeployment),
            inputs::Kind::Interface => Ok(Self::Interface)
        }
    }
}

impl TryFrom<inputs::InterfaceType> for entities::InterfaceType {
    type Error = Error;
    
    fn try_from(value: inputs::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            inputs::InterfaceType::Container => Ok(Self::Container),
            inputs::InterfaceType::Model => Ok(Self::Model),
            inputs::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}

impl TryFrom<inputs::ContainerInterfaceMetadata> for entities::ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: inputs::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<inputs::ContainerInterface> for entities::ContainerInterface {
    type Error = Error;
    
    fn try_from(value: inputs::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = entities::Kind::try_from(value.kind)?;
        if kind != entities::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = entities::InterfaceType::try_from(value.r#type)?;
        if r#type != entities::InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: entities::ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: entities::ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<inputs::Protocol> for entities::Protocol {
    type Error = Error;
    
    fn try_from(value: inputs::Protocol) -> Result<Self, Self::Error> {
        match value {
            inputs::Protocol::Http => Ok(Self::Http),
            inputs::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}

impl TryFrom<inputs::Port> for entities::Port {
    type Error = Error;
    
    fn try_from(value: inputs::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: entities::Protocol::try_from(value.protocol)?
        })
    }
}

impl TryFrom<inputs::GpuResourceDefinition> for entities::GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: inputs::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}

impl TryFrom<inputs::ResourcesDefinition> for entities::ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: inputs::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| entities::GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}

impl TryFrom<inputs::Resources> for entities::Resources {
    type Error = Error;
    
    fn try_from(value: inputs::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| entities::ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| entities::ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}

impl TryFrom<inputs::ContainerInterfaceSpec> for entities::ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: inputs::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| entities::Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<entities::Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(entities::Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}

impl TryFrom<inputs::EndpointLabels> for entities::EndpointLabels {
    type Error = Error;
    
    fn try_from(value: inputs::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}

impl TryFrom<inputs::OpenApiV3Spec> for entities::OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: inputs::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| entities::EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}

impl TryFrom<inputs::RestApiInterfaceSpec> for entities::RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: inputs::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            inputs::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(entities::RestApiInterfaceSpec::OpenApiV3(entities::OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}

impl TryFrom<inputs::RestApiInterfaceFormat> for entities::RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: inputs::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            inputs::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}

impl TryFrom<inputs::RestApiInterfaceMetadata> for entities::RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: inputs::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<inputs::RestApiInterface> for entities::RestApiInterface {
    type Error = Error;
    
    fn try_from(value: inputs::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = entities::Kind::try_from(value.kind)?;
        if kind != entities::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = entities::InterfaceType::try_from(value.r#type)?;
        if r#type != entities::InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = entities::RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: entities::RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: entities::RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<inputs::ModelInterfaceMetadataSelectors> for entities::ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: inputs::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}

impl TryFrom<inputs::ModelInterfaceMetadataDiscoveryCriteria> for entities::ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: inputs::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut criteria: Vec<ModelDiscoveryCriteria> = Vec::with_capacity(1);
        for criterion in value.criteria {
            criteria.push(ModelDiscoveryCriteria::try_from(criterion)?);
        }
        
        Ok(Self {
            platform: value.platform,
            confidence: value.confidence,
            criteria
        })
    }
}

impl TryFrom<inputs::ModelInterfaceMetadata> for entities::ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: inputs::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<entities::ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = entities::ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| entities::ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
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

impl TryFrom<inputs::ModelInterfaceSpec> for entities::ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: inputs::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}

impl TryFrom<inputs::ModelInterface> for entities::ModelInterface {
    type Error = Error;
    
    fn try_from(value: inputs::ModelInterface) -> Result<Self, Self::Error> {
        let kind = entities::Kind::try_from(value.kind)?;
        if kind != entities::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = entities::InterfaceType::try_from(value.r#type)?;
        if r#type != entities::InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: entities::ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: entities::ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<inputs::InferenceServerInterface> for entities::InferenceServerInterface {
    type Error = Error;
    
    fn try_from(value: inputs::InferenceServerInterface) -> Result<Self, Self::Error> {
        match value {
            inputs::InferenceServerInterface::Container(interface) => {
                let r#type = entities::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != entities::InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(entities::InferenceServerInterface::Container(entities::ContainerInterface::try_from(interface)?))
            },
            inputs::InferenceServerInterface::RestApi(interface) => {
                let r#type = entities::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != entities::InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(entities::InferenceServerInterface::RestApi(entities::RestApiInterface::try_from(interface)?))
            },
            inputs::InferenceServerInterface::Model(interface) => {
                let r#type = entities::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != entities::InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(entities::InferenceServerInterface::Model(entities::ModelInterface::try_from(interface)?))
            },
        }
    }
}

impl TryFrom<inputs::InferenceServerMetadata> for entities::InferenceServerMetadata {
    type Error = Error;
    
    fn try_from(value: inputs::InferenceServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<inputs::InferenceServerSpec> for entities::InferenceServerSpec {
    type Error = Error;
    
    fn try_from(value: inputs::InferenceServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<entities::InferenceServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(entities::InferenceServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}

impl TryFrom<inputs::CreateInferenceServerInput> for entities::InferenceServer {
    type Error = Error;
    
    fn try_from(value: inputs::CreateInferenceServerInput) -> Result<Self, Self::Error> {
        let kind = entities::Kind::try_from(value.kind)?;
        if kind != entities::Kind::InferenceServer {
            return Err(Error::from_str("Field 'kind' on InferenceServer must be variant Kind::InferenceServer"));
        }

        Ok(Self {
            kind,
            metadata: entities::InferenceServerMetadata::try_from(value.metadata)?,
            spec: entities::InferenceServerSpec::try_from(value.spec)?
        })
    }
}