use crate::domain::entities as domain;
use std::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use openapiv3::OpenAPI;
use shared::errors::Error;

pub type Labels = HashMap<String, String>;

pub type Description = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Kind {
    InferenceServer,
    InferenceServerDeployment,
    Interface,
}

impl TryFrom<domain::Kind> for Kind {
    type Error = Error;
    
    fn try_from(value: domain::Kind) -> Result<Self, Self::Error> {
        match value {
            domain::Kind::InferenceServer => Ok(Self::InferenceServer),
            domain::Kind::InferenceServerDeployment => Ok(Self::InferenceServerDeployment),
            domain::Kind::Interface => Ok(Self::Interface)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Container,
    Model,
    RestApi
}

impl TryFrom<domain::InterfaceType> for InterfaceType {
    type Error = Error;
    
    fn try_from(value: domain::InterfaceType) -> Result<Self, Self::Error> {
        match value {
            domain::InterfaceType::Container => Ok(Self::Container),
            domain::InterfaceType::Model => Ok(Self::Model),
            domain::InterfaceType::RestApi => Ok(Self::RestApi)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

impl TryFrom<domain::ContainerInterfaceMetadata> for ContainerInterfaceMetadata {
    type Error = Error;
    
    fn try_from(value: domain::ContainerInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ContainerInterfaceMetadata,
    pub spec: ContainerInterfaceSpec
}

impl TryFrom<domain::ContainerInterface> for ContainerInterface {
    type Error = Error;
    
    fn try_from(value: domain::ContainerInterface) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Protocol {
    Http,
    Tcp
}

impl TryFrom<domain::Protocol> for Protocol {
    type Error = Error;
    
    fn try_from(value: domain::Protocol) -> Result<Self, Self::Error> {
        match value {
            domain::Protocol::Http => Ok(Self::Http),
            domain::Protocol::Tcp => Ok(Self::Tcp),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Port {
    pub name: Option<String>,
    pub port: u16,
    pub protocol: Protocol
}

impl TryFrom<domain::Port> for Port {
    type Error = Error;
    
    fn try_from(value: domain::Port) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            port: value.port,
            protocol: Protocol::try_from(value.protocol)?
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuResourceDefinition {
    pub nvidia: Option<String>,
    pub amd: Option<String>
}

impl TryFrom<domain::GpuResourceDefinition> for GpuResourceDefinition {
    type Error = Error;
    
    fn try_from(value: domain::GpuResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            nvidia: value.nvidia,
            amd: value.amd
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourcesDefinition {
    pub cpu: Option<String>,
    pub disk: Option<String>,
    pub memory: Option<String>,
    pub gpu: Option<GpuResourceDefinition>
}

impl TryFrom<domain::ResourcesDefinition> for ResourcesDefinition {
    type Error = Error;
    
    fn try_from(value: domain::ResourcesDefinition) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub limits: Option<ResourcesDefinition>,
    pub requests: Option<ResourcesDefinition>
}

impl TryFrom<domain::Resources> for Resources {
    type Error = Error;
    
    fn try_from(value: domain::Resources) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInterfaceSpec {
    pub image: String,
    pub ports: Option<Vec<Port>>,
    pub resources: Option<Resources>
}

impl TryFrom<domain::ContainerInterfaceSpec> for ContainerInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: domain::ContainerInterfaceSpec) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndpointLabels {
    pub operation_id: String,
    pub labels: Labels
}

impl TryFrom<domain::EndpointLabels> for EndpointLabels {
    type Error = Error;
    
    fn try_from(value: domain::EndpointLabels) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            labels: value.labels
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenApiV3Spec {
    pub endpoint_labels: Option<EndpointLabels>,
    pub spec: OpenAPI
}

impl TryFrom<domain::OpenApiV3Spec> for OpenApiV3Spec {
    type Error = Error;
    
    fn try_from(value: domain::OpenApiV3Spec) -> Result<Self, Self::Error> {
        let endpoint_labels = value.endpoint_labels
            .map(|labels| EndpointLabels::try_from(labels))
            .transpose()?;

        Ok(Self {
            endpoint_labels,
            spec: value.spec
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RestApiInterfaceSpec {
    OpenApiV3(OpenApiV3Spec)
}

impl TryFrom<domain::RestApiInterfaceSpec> for RestApiInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: domain::RestApiInterfaceSpec) -> Result<Self, Self::Error> {
        match value {
            domain::RestApiInterfaceSpec::OpenApiV3(spec) => {
                Ok(RestApiInterfaceSpec::OpenApiV3(OpenApiV3Spec::try_from(spec)?))
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RestApiInterfaceFormat {
    OpenApiV3
}

impl TryFrom<domain::RestApiInterfaceFormat> for RestApiInterfaceFormat {
    type Error = Error;
    
    fn try_from(value: domain::RestApiInterfaceFormat) -> Result<Self, Self::Error> {
        match value {
            domain::RestApiInterfaceFormat::OpenApiV3 => Ok(Self::OpenApiV3),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestApiInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

impl TryFrom<domain::RestApiInterfaceMetadata> for RestApiInterfaceMetadata {
    type Error = Error;

    fn try_from(value: domain::RestApiInterfaceMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            labels: value.labels
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestApiInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub format: RestApiInterfaceFormat,
    pub metadata: RestApiInterfaceMetadata,
    pub spec: RestApiInterfaceSpec
}

impl TryFrom<domain::RestApiInterface> for RestApiInterface {
    type Error = Error;
    
    fn try_from(value: domain::RestApiInterface) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceMetadataSelectors {
    pub match_server_labels: Option<Labels>,
    pub match_endpoint_labels: Option<Labels>
}

impl TryFrom<domain::ModelInterfaceMetadataSelectors> for ModelInterfaceMetadataSelectors {
    type Error = Error;

    fn try_from(value: domain::ModelInterfaceMetadataSelectors) -> Result<Self, Self::Error> {
        Ok(Self {
            match_server_labels: value.match_server_labels,
            match_endpoint_labels: value.match_endpoint_labels
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceMetadataDiscoveryCriteria {
    pub platform: String,
    pub confidence: Option<u8>,
    pub criteria: Vec<ModelDiscoveryCriteria>
}

/// Converts a ModelInterfaceMetadataDiscoveryCriteria domain entity into a 
/// mongo-specific database entity
impl TryFrom<domain::ModelInterfaceMetadataDiscoveryCriteria> for ModelInterfaceMetadataDiscoveryCriteria {
    type Error = Error;

    fn try_from(value: domain::ModelInterfaceMetadataDiscoveryCriteria) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>,
    pub discovery_criteria: Option<ModelInterfaceMetadataDiscoveryCriteria>,
    pub selectors: Option<Vec<ModelInterfaceMetadataSelectors>>
}

impl TryFrom<domain::ModelInterfaceMetadata> for ModelInterfaceMetadata {
    type Error = Error;

    fn try_from(value: domain::ModelInterfaceMetadata) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceSpec {
    pub input: Option<HashMap<String, Value>>,
    pub output: Option<HashMap<String, Value>>
}

impl TryFrom<domain::ModelInterfaceSpec> for ModelInterfaceSpec {
    type Error = Error;
    
    fn try_from(value: domain::ModelInterfaceSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value.input,
            output: value.output
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ModelInterfaceMetadata,
    pub spec: ModelInterfaceSpec,
}

impl TryFrom<domain::ModelInterface> for ModelInterface {
    type Error = Error;
    
    fn try_from(value: domain::ModelInterface) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InferenceServerInterface {
    Container(ContainerInterface),
    RestApi(RestApiInterface),
    Model(ModelInterface),
}

impl TryFrom<domain::InferenceServerInterface> for InferenceServerInterface {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServerInterface) -> Result<Self, Self::Error> {
        match value {
            domain::InferenceServerInterface::Container(interface) => {
                let r#type = InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != InterfaceType::Container {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Container"))
                }
                Ok(InferenceServerInterface::Container(ContainerInterface::try_from(interface)?))
            },
            domain::InferenceServerInterface::RestApi(interface) => {
                let r#type = InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != InterfaceType::RestApi {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::RestApi"))
                }
                Ok(InferenceServerInterface::RestApi(RestApiInterface::try_from(interface)?))
            },
            domain::InferenceServerInterface::Model(interface) => {
                let r#type = InterfaceType::try_from(interface.r#type.clone())?;
                if r#type != InterfaceType::Model {
                    return Err(Error::from_str("Inference server interface field 'type' must be of of type InterfaceType::Model"))
                }
                Ok(InferenceServerInterface::Model(ModelInterface::try_from(interface)?))
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServerMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub labels: Labels
}

impl TryFrom<domain::InferenceServerMetadata> for InferenceServerMetadata {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServerSpec {
    pub interfaces: Option<Vec<InferenceServerInterface>>
}

impl TryFrom<domain::InferenceServerSpec> for InferenceServerSpec {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServerSpec) -> Result<Self, Self::Error> {
        let mut interfaces: Vec<InferenceServerInterface> = Vec::with_capacity(1);
        for inferface in value.interfaces.unwrap_or(Vec::with_capacity(0)) {
            interfaces.push(InferenceServerInterface::try_from(inferface)?);
        }
        Ok(Self {
            interfaces: Some(interfaces)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServer {
    pub kind: Kind,
    pub metadata: InferenceServerMetadata,
    pub spec: InferenceServerSpec
}

impl TryFrom<domain::InferenceServer> for InferenceServer {
    type Error = Error;
    
    fn try_from(value: domain::InferenceServer) -> Result<Self, Self::Error> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

impl TryFrom<domain::SystemRequirement> for SystemRequirement {
    type Error = Error;
    
    fn try_from(value: domain::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

impl TryFrom<domain::Accelerator> for Accelerator {
    type Error = Error;
    
    fn try_from(value: domain::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

impl TryFrom<domain::HardwareRequirements> for HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: domain::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(Accelerator::try_from(accelerator)?);
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

impl TryFrom<domain::ModelIO> for ModelIO {
    type Error = Error;
    
    fn try_from(value: domain::ModelIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelDiscoveryCriteria {
    // General fields
    pub name: Option<String>,
    pub model_type: Option<String>,
    pub version: Option<String>,
    pub framework: Option<String>,
    pub image: Option<String>,

    /// Arbitrary labels
    pub labels: Option<Vec<String>>,
    pub label_map: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,
    pub model_inputs: Option<Vec<ModelIO>>,
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    pub task_types: Option<Vec<String>>,
    pub inference_precision: Option<String>,
    pub inference_hardware: Option<HardwareRequirements>,
    pub inference_software_dependencies: Option<Vec<String>>,
    pub inference_max_energy_consumption_watts: Option<i32>,

    /// Inference performance fields
    pub inference_max_latency_ms: Option<i32>,
    pub inference_min_throughput: Option<i32>,
    pub inference_max_compute_utilization_percentage: Option<i32>,
    pub inference_max_memory_usage_mb: Option<i32>,
    pub inference_distributed: Option<bool>,

    /// Training-related Fields
    pub training_time: Option<i64>,
    pub training_precision: Option<String>,
    pub training_hardware: Option<HardwareRequirements>,
    pub pretraining_datasets: Option<Vec<String>>,
    pub finetuning_datasets: Option<Vec<String>>,
    pub edge_optimized: Option<bool>,
    pub quantization_aware: Option<bool>,
    pub supports_quantization: Option<bool>,
    pub pretrained: Option<bool>,
    pub pruned: Option<bool>,
    pub slimmed: Option<bool>,
    pub training_distributed: Option<bool>,

    /// Training performance fields
    pub training_max_energy_consumption_watts: Option<i32>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    pub bias_evaluation_score: Option<i8>,
}

impl TryFrom<domain::ModelDiscoveryCriteria> for ModelDiscoveryCriteria {
    type Error = Error;
    
    fn try_from(value: domain::ModelDiscoveryCriteria) -> Result<Self, Self::Error> {
        let mut model_inputs = Vec::with_capacity(1);
        for input in value.model_inputs.unwrap_or(Vec::with_capacity(0)) {
            model_inputs.push(ModelIO::try_from(input)?)
        }
        
        let mut model_outputs = Vec::with_capacity(1);
        for output in value.model_outputs.unwrap_or(Vec::with_capacity(0)) {
            model_outputs.push(ModelIO::try_from(output)?)
        }

        let inference_hardware = value.inference_hardware
            .map(|hardware| HardwareRequirements::try_from(hardware))
            .transpose()?;

        let training_hardware = value.training_hardware
            .map(|hardware| HardwareRequirements::try_from(hardware))
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServerDeployment {

}

impl TryFrom<domain::InferenceServerDeployment> for InferenceServerDeployment {
    type Error = Error;

    // TODO
    fn try_from(_value: domain::InferenceServerDeployment) -> Result<Self, Self::Error> {
        return Ok(Self {

        })
    }
}