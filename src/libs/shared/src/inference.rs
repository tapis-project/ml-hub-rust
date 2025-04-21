use std::collections::hash_map::HashMap;
use serde_json::Value;
use openapiv3::OpenAPI;
use crate::models::ModelDiscoveryCriteria;
use crate::requests::inference as requests;
use crate::errors::Error;

pub type Labels = HashMap<String, String>;

pub type Description = String;

#[derive(PartialEq, Eq)]
pub enum Kind {
    InferenceServer,
    InferenceServerDeployment,
    Interface,
}

impl TryFrom<requests::Kind> for Kind {
    type Error = Error;
    
    fn try_from(value: requests::Kind) -> Result<Self, Self::Error> {
        match value {
            requests::Kind::InferenceServer => Ok(Self::InferenceServer),
            requests::Kind::InferenceServerDeployment => Ok(Self::InferenceServerDeployment),
            requests::Kind::Interface => Ok(Self::Interface)
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum InterfaceType {
    Container,
    Model,
    RestApi
}

impl TryFrom<requests::InterfaceType> for InterfaceType {
    type Error = Error;
    
    fn try_from(value: requests::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            requests::InterfaceType::Container => Ok(Self::Container),
            requests::InterfaceType::Model => Ok(Self::Model),
            requests::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}

pub struct ContainerInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

impl TryFrom<requests::ContainerInterfaceMetadata> for ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: requests::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

pub struct ContainerInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ContainerInterfaceMetadata,
    pub spec: ContainerInterfaceSpec
}

impl TryFrom<requests::ContainerInterface> for ContainerInterface {
    type Error = Error;
    
    fn try_from(value: requests::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = Kind::try_from(value.kind)?;
        if kind != Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = InterfaceType::try_from(value.r#type)?;
        if r#type != InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}

pub enum Protocol {
    Http,
    Tcp
}

impl TryFrom<requests::Protocol> for Protocol {
    type Error = Error;
    
    fn try_from(value: requests::Protocol) -> Result<Self, Self::Error> {
        match value {
            requests::Protocol::Http => Ok(Self::Http),
            requests::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}

pub struct Port {
    pub name: Option<String>,
    pub port: u16,
    pub protocol: Protocol
}

impl TryFrom<requests::Port> for Port {
    type Error = Error;
    
    fn try_from(value: requests::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: Protocol::try_from(value.protocol)?
        })
    }
}

pub struct GpuResourceDefinition {
    pub nvidia: Option<String>,
    pub amd: Option<String>
}

impl TryFrom<requests::GpuResourceDefinition> for GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: requests::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}

pub struct ResourcesDefinition {
    pub cpu: Option<String>,
    pub disk: Option<String>,
    pub memory: Option<String>,
    pub gpu: Option<GpuResourceDefinition>
}

impl TryFrom<requests::ResourcesDefinition> for ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: requests::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}

pub struct Resources {
    pub limits: Option<ResourcesDefinition>,
    pub requests: Option<ResourcesDefinition>
}

impl TryFrom<requests::Resources> for Resources {
    type Error = Error;
    
    fn try_from(value: requests::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}

pub struct ContainerInterfaceSpec {
    pub image: String,
    pub ports: Option<Vec<Port>>,
    pub resources: Option<Resources>
}

impl TryFrom<requests::ContainerInterfaceSpec> for ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: requests::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}

pub struct EndpointLabels {
    pub operation_id: String,
    pub labels: Labels
}

impl TryFrom<requests::EndpointLabels> for EndpointLabels {
    type Error = Error;
    
    fn try_from(value: requests::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}

pub struct OpenApiV3Spec {
    pub endpoint_labels: Option<EndpointLabels>,
    pub spec: OpenAPI
}

impl TryFrom<requests::OpenApiV3Spec> for OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: requests::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}

pub enum RestApiInterfaceSpec {
    OpenApiV3(OpenApiV3Spec)
}

impl TryFrom<requests::RestApiInterfaceSpec> for RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: requests::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            requests::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(RestApiInterfaceSpec::OpenApiV3(OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}

pub enum RestApiInterfaceFormat {
    OpenApiV3
}

impl TryFrom<requests::RestApiInterfaceFormat> for RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: requests::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            requests::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}

pub struct RestApiInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

impl TryFrom<requests::RestApiInterfaceMetadata> for RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: requests::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

pub struct RestApiInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub format: RestApiInterfaceFormat,
    pub metadata: RestApiInterfaceMetadata,
    pub spec: RestApiInterfaceSpec
}

impl TryFrom<requests::RestApiInterface> for RestApiInterface {
    type Error = Error;
    
    fn try_from(value: requests::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = Kind::try_from(value.kind)?;
        if kind != Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = InterfaceType::try_from(value.r#type)?;
        if r#type != InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}

pub struct ModelInterfaceMetadataSelectors {
    pub match_server_labels: Option<Labels>,
    pub match_endpoint_labels: Option<Labels>
}

impl TryFrom<requests::ModelInterfaceMetadataSelectors> for ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: requests::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}

pub struct ModelInterfaceMetadataDiscoveryCriteria {
    pub platform: String,
    pub confidence: Option<u8>,
    pub criteria: Vec<ModelDiscoveryCriteria>
}

impl TryFrom<requests::ModelInterfaceMetadataDiscoveryCriteria> for ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: requests::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
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

pub struct ModelInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>,
    pub discovery_criteria: Option<ModelInterfaceMetadataDiscoveryCriteria>,
    pub selectors: Option<Vec<ModelInterfaceMetadataSelectors>>
}

impl TryFrom<requests::ModelInterfaceMetadata> for ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: requests::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
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

pub struct ModelInterfaceSpec {
    pub input: Option<HashMap<String, Value>>,
    pub output: Option<HashMap<String, Value>>
}

impl TryFrom<requests::ModelInterfaceSpec> for ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: requests::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}

pub struct ModelInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ModelInterfaceMetadata,
    pub spec: ModelInterfaceSpec,
}

impl TryFrom<requests::ModelInterface> for ModelInterface {
    type Error = Error;
    
    fn try_from(value: requests::ModelInterface) -> Result<Self, Self::Error> {
        let kind = Kind::try_from(value.kind)?;
        if kind != Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = InterfaceType::try_from(value.r#type)?;
        if r#type != InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}

pub enum InferenceServerInterface {
    Container(ContainerInterface),
    RestApi(RestApiInterface),
    Model(ModelInterface),
}

impl TryFrom<requests::InferenceServerInterface> for InferenceServerInterface {
    type Error = Error;
    
    fn try_from(value: requests::InferenceServerInterface) -> Result<Self, Self::Error> {
        match value {
            requests::InferenceServerInterface::Container(interface) => {
                let r#type = InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(InferenceServerInterface::Container(ContainerInterface::try_from(interface)?))
            },
            requests::InferenceServerInterface::RestApi(interface) => {
                let r#type = InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(InferenceServerInterface::RestApi(RestApiInterface::try_from(interface)?))
            },
            requests::InferenceServerInterface::Model(interface) => {
                let r#type = InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(InferenceServerInterface::Model(ModelInterface::try_from(interface)?))
            },
        }
    }
}

pub struct InferenceServerMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub labels: Labels
}

impl TryFrom<requests::InferenceServerMetadata> for InferenceServerMetadata {
    type Error = Error;
    
    fn try_from(value: requests::InferenceServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}

pub struct InferenceServerSpec {
    pub interfaces: Option<Vec<InferenceServerInterface>>
}

impl TryFrom<requests::InferenceServerSpec> for InferenceServerSpec {
    type Error = Error;
    
    fn try_from(value: requests::InferenceServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<InferenceServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(InferenceServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}

pub struct InferenceServer {
    pub kind: Kind,
    pub metadata: InferenceServerMetadata,
    pub spec: InferenceServerSpec
}

impl TryFrom<requests::InferenceServer> for InferenceServer {
    type Error = Error;
    
    fn try_from(value: requests::InferenceServer) -> Result<Self, Self::Error> {
        let kind = Kind::try_from(value.kind)?;
        if kind != Kind::InferenceServer {
            return Err(Error::from_str("Field 'kind' on InferenceServer must be variant Kind::InferenceServer"));
        }

        Ok(Self {
            kind,
            metadata: InferenceServerMetadata::try_from(value.metadata)?,
            spec: InferenceServerSpec::try_from(value.spec)?
        })
    }
}