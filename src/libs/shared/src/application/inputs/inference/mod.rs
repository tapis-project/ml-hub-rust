pub mod domain_to_inputs;

use std::collections::hash_map::HashMap;
use serde_json::Value;
use openapiv3::OpenAPI;
pub use crate::application::inputs::models::ModelMetadata;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FilterOperation {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    In,
    Nin,
    Pattern
}

#[derive(Debug)]
pub struct Filter {
    pub field: String,
    pub operation: FilterOperation,
    pub value: String
}

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc
}

#[derive(Debug)]
pub struct ListAll {
    pub limit: u64,
    pub offset: u64,
    pub fields: Vec<String>,
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub order_by: Option<Order>
}

pub type Labels = HashMap<String, String>;

pub type Description = String;

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    InferenceServer,
    InferenceServerDeployment,
    Interface,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InterfaceType {
    Container,
    Model,
    RestApi
}

#[derive(Debug)]
pub struct ContainerInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

#[derive(Debug)]
pub struct ContainerInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ContainerInterfaceMetadata,
    pub spec: ContainerInterfaceSpec
}

#[derive(Debug)]
pub enum Protocol {
    Http,
    Tcp
}

#[derive(Debug)]
pub struct Port {
    pub name: Option<String>,
    pub port: u16,
    pub protocol: Protocol
}

#[derive(Debug)]
pub struct GpuResourceDefinition {
    pub nvidia: Option<String>,
    pub amd: Option<String>
}

#[derive(Debug)]
pub struct ResourcesDefinition {
    pub cpu: Option<String>,
    pub disk: Option<String>,
    pub memory: Option<String>,
    pub gpu: Option<GpuResourceDefinition>
}

#[derive(Debug)]
pub struct Resources {
    pub limits: Option<ResourcesDefinition>,
    pub requests: Option<ResourcesDefinition>
}

#[derive(Debug)]
pub struct ContainerInterfaceSpec {
    pub image: String,
    pub ports: Option<Vec<Port>>,
    pub resources: Option<Resources>
}

#[derive(Debug)]
pub struct EndpointLabels {
    pub operation_id: String,
    pub labels: Labels
}

#[derive(Debug)]
pub struct OpenApiV3Spec {
    pub endpoint_labels: Option<EndpointLabels>,
    pub spec: OpenAPI
}

#[derive(Debug)]
pub enum RestApiInterfaceSpec {
    OpenApiV3(OpenApiV3Spec)
}

#[derive(Debug)]
pub enum RestApiInterfaceFormat {
    OpenApiV3
}

#[derive(Debug)]
pub struct RestApiInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

#[derive(Debug)]
pub struct RestApiInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub format: RestApiInterfaceFormat,
    pub metadata: RestApiInterfaceMetadata,
    pub spec: RestApiInterfaceSpec
}

#[derive(Debug)]
pub struct ModelInterfaceMetadataSelectors {
    pub match_server_labels: Option<Labels>,
    pub match_endpoint_labels: Option<Labels>
}

#[derive(Debug)]
pub struct ModelInterfaceMetadataDiscoveryCriteria {
    pub platform: String,
    pub confidence: Option<u8>,
    pub criteria: Vec<ModelMetadata>
}

#[derive(Debug)]
pub struct ModelInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>,
    pub discovery_criteria: Option<ModelInterfaceMetadataDiscoveryCriteria>,
    pub selectors: Option<Vec<ModelInterfaceMetadataSelectors>>
}

#[derive(Debug)]
pub struct ModelInterfaceSpec {
    pub input: Option<HashMap<String, Value>>,
    pub output: Option<HashMap<String, Value>>
}

#[derive(Debug)]
pub struct ModelInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ModelInterfaceMetadata,
    pub spec: ModelInterfaceSpec,
}

#[derive(Debug)]
pub enum InferenceServerInterface {
    Container(ContainerInterface),
    RestApi(RestApiInterface),
    Model(ModelInterface),
}

#[derive(Debug)]
pub struct InferenceServerMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub labels: Labels
}

#[derive(Debug)]
pub struct InferenceServerSpec {
    pub interfaces: Option<Vec<InferenceServerInterface>>
}

#[derive(Debug)]
pub struct CreateInferenceServerInput {
    pub kind: Kind,
    pub metadata: InferenceServerMetadata,
    pub spec: InferenceServerSpec
}
