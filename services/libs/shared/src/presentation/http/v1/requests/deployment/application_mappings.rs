//! Contains conversions between web layer dtos and application layer inputs
use crate::presentation::http::v1::requests::deployments as requests;
use crate::presentation::http::v1::requests::filtering::{FilterOperation, Filter, Order, ListAll};
use crate::application::inputs::inference as inputs;
use crate::application::inputs::model_metadata::ModelMetadata;
use crate::errors::Error;

impl TryFrom<FilterOperation> for inputs::FilterOperation {
    type Error = Error;
    
    fn try_from(value: FilterOperation) -> Result<Self, Self::Error> {
        match value {
            FilterOperation::Eq => Ok(Self::Eq),
            FilterOperation::Ne => Ok(Self::Ne),
            FilterOperation::Gt => Ok(Self::Gt),
            FilterOperation::Gte => Ok(Self::Gte),
            FilterOperation::Lt => Ok(Self::Lt),
            FilterOperation::Lte => Ok(Self::Lte),
            FilterOperation::In => Ok(Self::In),
            FilterOperation::Nin => Ok(Self::Nin),
            FilterOperation::Pattern => Ok(Self::Pattern),
        }
    }
}

impl TryFrom<Filter> for inputs::Filter {
    type Error = Error;
    
    fn try_from(value: Filter) -> Result<Self, Self::Error> {
        Ok(Self {
            field: value.field,
            operation: inputs::FilterOperation::try_from(value.operation)?,
            value: value.value
        })
    }
}

impl TryFrom<Order> for inputs::Order {
    type Error = Error;
    
    fn try_from(value: Order) -> Result<Self, Self::Error> {
        match value {
            Order::Asc => Ok(Self::Asc),
            Order::Desc => Ok(Self::Desc),
        }
    }
}

impl TryFrom<ListAll> for inputs::ListAll {
    type Error = Error;
    
    fn try_from(value: ListAll) -> Result<Self, Self::Error> {
        let page = value.page.unwrap_or(1);
        if page == 0 {
            return Err(Self::Error::new(String::from("Value for field 'page' must be >= 1")));
        }
        
        let page_size = value.page_size.unwrap_or(100);
        if page_size == 0 || page_size > 1000 {
            return Err(Self::Error::new(String::from("Value for field 'page_size' must be > 0 and <= 1000")));
        }

        let mut filters: Vec<inputs::Filter> = Vec::new();
        for filter in value.filters.unwrap_or(Vec::new()) {
            let converted_filter = inputs::Filter::try_from(filter)
                .map_err(|err| Self::Error::new(err.to_string()))?;
            filters.push(converted_filter);
        }

        Ok(Self {
            limit: page_size,
            offset: page_size * (page - 1),
            fields: value.fields.unwrap_or(Vec::new()),
            sort_by: value.sort_by,
            filters: Some(filters),
            order_by: Some(inputs::Order::try_from(value.order_by.unwrap_or(Order::Asc))?)
        })
    }
}

impl TryFrom<requests::Kind> for inputs::Kind {
    type Error = Error;
    
    fn try_from(value: requests::Kind) -> Result<Self, Self::Error> {
        match value {
            requests::Kind::ModelServer => Ok(Self::ModelServer),
            requests::Kind::ModelServerDeployment => Ok(Self::ModelServerDeployment),
            requests::Kind::Interface => Ok(Self::Interface)
        }
    }
}

impl TryFrom<requests::InterfaceType> for inputs::InterfaceType {
    type Error = Error;
    
    fn try_from(value: requests::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            requests::InterfaceType::Container => Ok(Self::Container),
            requests::InterfaceType::Model => Ok(Self::Model),
            requests::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}

impl TryFrom<requests::ContainerInterfaceMetadata> for inputs::ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: requests::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<requests::ContainerInterface> for inputs::ContainerInterface {
    type Error = Error;
    
    fn try_from(value: requests::ContainerInterface) -> Result<Self, Self::Error> {
        let kind = inputs::Kind::try_from(value.kind)?;
        if kind != inputs::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be of variant Kind::Interface"))
        }

        let r#type = inputs::InterfaceType::try_from(value.r#type)?;
        if r#type != inputs::InterfaceType::Container {
            return Err(Error::from_str("Field 'type' must be of variant InterfaceType::Container"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: inputs::ContainerInterfaceMetadata::try_from(value.metadata)?,
            spec: inputs::ContainerInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<requests::Protocol> for inputs::Protocol {
    type Error = Error;
    
    fn try_from(value: requests::Protocol) -> Result<Self, Self::Error> {
        match value {
            requests::Protocol::Http => Ok(Self::Http),
            requests::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}

impl TryFrom<requests::Port> for inputs::Port {
    type Error = Error;
    
    fn try_from(value: requests::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: inputs::Protocol::try_from(value.protocol)?
        })
    }
}

impl TryFrom<requests::GpuResourceDefinition> for inputs::GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: requests::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}

impl TryFrom<requests::ResourcesDefinition> for inputs::ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: requests::ResourcesDefinition) -> Result<Self, Self::Error> {
        let gpu = value.gpu
            .map(|gpu| inputs::GpuResourceDefinition::try_from(gpu))
            .transpose()?;

        Ok(Self {
            cpu: value.cpu,
            disk: value.disk,
            memory: value.memory,
            gpu,
        })
    }
}

impl TryFrom<requests::Resources> for inputs::Resources {
    type Error = Error;
    
    fn try_from(value: requests::Resources) -> Result<Self, Self::Error> {
        let limits = value.limits
            .map(|limits| inputs::ResourcesDefinition::try_from(limits))
            .transpose()?;

        let requests = value.requests
            .map(|requests| inputs::ResourcesDefinition::try_from(requests))
            .transpose()?;

        Ok(Self {
            limits,
            requests,
        })
    }
}

impl TryFrom<requests::ContainerInterfaceSpec> for inputs::ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: requests::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
        let resources = value.resources
            .map(|resources| inputs::Resources::try_from(resources))
            .transpose()?;

        let mut ports: Vec<inputs::Port> = Vec::with_capacity(1);
        for p in value.ports.unwrap_or(Vec::with_capacity(0)) {
            ports.push(inputs::Port::try_from(p)?)
        }

        Ok(Self {
            image: value.image,
            resources,
            ports: Some(ports)
        })
    }
}

impl TryFrom<requests::EndpointLabels> for inputs::EndpointLabels {
    type Error = Error;
    
    fn try_from(value: requests::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}

impl TryFrom<requests::OpenApiV3Spec> for inputs::OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: requests::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| inputs::EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}

impl TryFrom<requests::RestApiInterfaceSpec> for inputs::RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: requests::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            requests::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(inputs::RestApiInterfaceSpec::OpenApiV3(inputs::OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}

impl TryFrom<requests::RestApiInterfaceFormat> for inputs::RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: requests::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            requests::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}

impl TryFrom<requests::RestApiInterfaceMetadata> for inputs::RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: requests::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<requests::RestApiInterface> for inputs::RestApiInterface {
    type Error = Error;
    
    fn try_from(value: requests::RestApiInterface) -> Result<Self, Self::Error> {
        let kind = inputs::Kind::try_from(value.kind)?;
        if kind != inputs::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = inputs::InterfaceType::try_from(value.r#type)?;
        if r#type != inputs::InterfaceType::RestApi {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::RestApi"))
        }

        let format = inputs::RestApiInterfaceFormat::try_from(value.format)?;
        

        Ok(Self {
            kind,
            r#type,
            format,
            metadata: inputs::RestApiInterfaceMetadata::try_from(value.metadata)?,
            spec: inputs::RestApiInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<requests::ModelInterfaceMetadataSelectors> for inputs::ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: requests::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}

impl TryFrom<requests::ModelInterfaceMetadataDiscoveryCriteria> for inputs::ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: requests::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut criteria: Vec<ModelMetadata> = Vec::with_capacity(1);
        for criterion in value.criteria {
            criteria.push(ModelMetadata::try_from(criterion)?);
        }
        
        Ok(Self {
            platform: value.platform,
            confidence: value.confidence,
            criteria
        })
    }
}

impl TryFrom<requests::ModelInterfaceMetadata> for inputs::ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: requests::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
        let mut selectors: Vec<inputs::ModelInterfaceMetadataSelectors> = Vec::with_capacity(1);
        for request_selector in value.selectors.unwrap_or(Vec::with_capacity(0)) {
            let selector = inputs::ModelInterfaceMetadataSelectors::try_from(request_selector)?;
            selectors.push(selector)
        }

        let discovery_criteria = value.discovery_criteria
            .map(|criteria| inputs::ModelInterfaceMetadataDiscoveryCriteria::try_from(criteria))
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

impl TryFrom<requests::ModelInterfaceSpec> for inputs::ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: requests::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}

impl TryFrom<requests::ModelInterface> for inputs::ModelInterface {
    type Error = Error;
    
    fn try_from(value: requests::ModelInterface) -> Result<Self, Self::Error> {
        let kind = inputs::Kind::try_from(value.kind)?;
        if kind != inputs::Kind::Interface {
            return Err(Error::from_str("Field 'kind' must be variant Kind::Interface"))
        }

        let r#type = inputs::InterfaceType::try_from(value.r#type)?;
        if r#type != inputs::InterfaceType::Model {
            return Err(Error::from_str("Field 'type' must be variant InterfaceType::Model"))
        }

        Ok(Self {
            kind,
            r#type,
            metadata: inputs::ModelInterfaceMetadata::try_from(value.metadata)?,
            spec: inputs::ModelInterfaceSpec::try_from(value.spec)?
        })
    }
}

impl TryFrom<requests::ModelServerInterface> for inputs::ModelServerInterface {
    type Error = Error;
    
    fn try_from(value: requests::ModelServerInterface) -> Result<Self, Self::Error> {
        match value {
            requests::ModelServerInterface::Container(interface) => {
                let r#type = inputs::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != inputs::InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(inputs::ModelServerInterface::Container(inputs::ContainerInterface::try_from(interface)?))
            },
            requests::ModelServerInterface::RestApi(interface) => {
                let r#type = inputs::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != inputs::InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(inputs::ModelServerInterface::RestApi(inputs::RestApiInterface::try_from(interface)?))
            },
            requests::ModelServerInterface::Model(interface) => {
                let r#type = inputs::InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != inputs::InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(inputs::ModelServerInterface::Model(inputs::ModelInterface::try_from(interface)?))
            },
        }
    }
}

impl TryFrom<requests::ModelServerMetadata> for inputs::ModelServerMetadata {
    type Error = Error;
    
    fn try_from(value: requests::ModelServerMetadata) -> Result<Self, Self::Error> {
        
        Ok(Self {
            name: value.name,
            version: value.version,
            description: value.description,
            labels: value.labels
        })
    }
}

impl TryFrom<requests::ModelServerSpec> for inputs::ModelServerSpec {
    type Error = Error;
    
    fn try_from(value: requests::ModelServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<inputs::ModelServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(inputs::ModelServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}

impl TryFrom<requests::ModelServer> for inputs::CreateModelServerInput {
    type Error = Error;
    
    fn try_from(value: requests::ModelServer) -> Result<Self, Self::Error> {
        let kind = inputs::Kind::try_from(value.kind)?;
        if kind != inputs::Kind::ModelServer {
            return Err(Error::from_str("Field 'kind' on ModelServer must be variant Kind::ModelServer"));
        }

        Ok(Self {
            kind,
            metadata: inputs::ModelServerMetadata::try_from(value.metadata)?,
            spec: inputs::ModelServerSpec::try_from(value.spec)?
        })
    }
}