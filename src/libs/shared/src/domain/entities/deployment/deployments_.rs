use std::collections::hash_map::HashMap;
use serde_json::Value;
use openapiv3::OpenAPI;
use crate::domain::entities::model_metadata::ModelMetadata;

pub type Labels = HashMap<String, String>;

pub type Description = String;

#[derive(PartialEq, Eq)]
pub enum Kind {
    ModelServer,
    ModelServerDeployment,
    Interface,
}

#[derive(PartialEq, Eq, Clone)]
pub enum InterfaceType {
    Container,
    Model,
    RestApi
}

pub struct ContainerInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

pub struct ContainerInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ContainerInterfaceMetadata,
    pub spec: ContainerInterfaceSpec
}

pub enum Protocol {
    Http,
    Tcp
}

pub struct Port {
    pub name: Option<String>,
    pub port: u16,
    pub protocol: Protocol
}

pub struct ContainerInterfaceSpec {
    pub image: String,
    pub ports: Option<Vec<Port>>,
    pub resources: Option<Resources>
}

pub struct EndpointLabels {
    pub operation_id: String,
    pub labels: Labels
}

pub struct OpenApiV3Spec {
    pub endpoint_labels: Option<EndpointLabels>,
    pub spec: OpenAPI
}

pub enum RestApiInterfaceSpec {
    OpenApiV3(OpenApiV3Spec)
}

pub enum RestApiInterfaceFormat {
    OpenApiV3
}

pub struct RestApiInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

pub struct RestApiInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub format: RestApiInterfaceFormat,
    pub metadata: RestApiInterfaceMetadata,
    pub spec: RestApiInterfaceSpec
}

pub struct ModelInterfaceMetadataSelectors {
    pub match_server_labels: Option<Labels>,
    pub match_endpoint_labels: Option<Labels>
}

pub struct ModelInterfaceMetadataDiscoveryCriteria {
    pub platform: String,
    pub confidence: Option<u8>,
    pub criteria: Vec<ModelMetadata>
}

pub struct ModelInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>,
    pub discovery_criteria: Option<ModelInterfaceMetadataDiscoveryCriteria>,
    pub selectors: Option<Vec<ModelInterfaceMetadataSelectors>>
}

pub struct ModelInterfaceSpec {
    pub input: Option<HashMap<String, Value>>,
    pub output: Option<HashMap<String, Value>>
}

pub struct ModelInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ModelInterfaceMetadata,
    pub spec: ModelInterfaceSpec,
}

pub enum ModelServerInterface {
    Container(ContainerInterface),
    RestApi(RestApiInterface),
    Model(ModelInterface),
}

pub struct ModelServerMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub labels: Labels
}

pub struct ModelServerSpec {
    pub interfaces: Option<Vec<ModelServerInterface>>
}

pub struct ModelServer {
    pub kind: Kind,
    pub metadata: ModelServerMetadata,
    pub spec: ModelServerSpec
}

// TODO
pub struct ModelServerDeployment;