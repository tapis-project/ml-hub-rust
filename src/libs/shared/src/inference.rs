use std::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use openapiv3::OpenAPI;
use crate::requests::ModelDiscoveryCriteria;

pub type Labels = HashMap<String, String>;
pub type Description = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum Kind {
    InferenceServer,
    InferenceServerDeployment,
    Interface,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InterfaceType {
    Container,
    Model,
    RestApi
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerInterface {
    kind: Kind,
    r#type: InterfaceType,
    metadata: ContainerInterfaceMetadata,
    spec: ContainerInterfaceSpec
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol {
    Http,
    Tcp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    name: Option<String>,
    port: u16,
    protocol: Protocol
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuResourceDefinition {
    nvidia: Option<String>,
    amd: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesDefinition {
    cpu: Option<String>,
    disk: Option<String>,
    memory: Option<String>,
    gpu: Option<GpuResourceDefinition>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    limits: Option<ResourcesDefinition>,
    requests: Option<ResourcesDefinition>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerInterfaceSpec {
    image: String,
    ports: Option<Vec<Port>>,
    resources: Option<Resources>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointLabels {
    operation_id: String,
    labels: Labels
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiV3Spec {
    endpoint_labels: Option<EndpointLabels>,
    spec: OpenAPI
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RestApiInterfaceSpec {
    OpenApiV3(OpenApiV3Spec)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RestApiInterfaceFormat {
    OpenApiV3
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestApiInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestApiInterface {
    kind: Kind,
    r#type: InterfaceType,
    format: RestApiInterfaceFormat,
    metadata: RestApiInterfaceMetadata,
    spec: RestApiInterfaceSpec
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInterfaceMetadataSelectors {
    match_server_labels: Option<Labels>,
    match_endpoint_labels: Option<Labels>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInterfaceMetadataDiscoveryCriteria {
    platform: String,
    confidence: Option<u8>,
    criteria: Vec<ModelDiscoveryCriteria>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>,
    pub discovery_criteria: Option<ModelInterfaceMetadataDiscoveryCriteria>,
    pub selectors: Option<Vec<ModelInterfaceMetadataSelectors>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInterfaceSpec {
    input: Option<HashMap<String, Value>>,
    output: Option<HashMap<String, Value>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInterface {
    kind: Kind,
    r#type: InterfaceType,
    metadata: ModelInterfaceMetadata,
    spec: ModelInterfaceSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InferenceServerInterface {
    Container(ContainerInterface),
    RestApi(RestApiInterface),
    Model(ModelInterface),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceServerMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub labels: Labels
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceServerSpec {
    pub interfaces: Option<Vec<InferenceServerInterface>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceServer {
    pub kind: Kind,
    pub metadata: InferenceServerMetadata,
    pub spec: InferenceServerSpec
}