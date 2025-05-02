use std::collections::HashMap;
use actix_web::web;
use serde::{Deserialize, Serialize};
use actix_web::HttpRequest;
use serde_json::Value;
use openapiv3::OpenAPI;
use crate::models::web::dto::ModelDiscoveryCriteria;
pub use crate::filtering::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetInferenceServerDocsPath {
    pub inference_server_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StartInferenceServerPath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunInferencePath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateInferenceServerPath {
    pub platform: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateInferencePath {
    pub inference_service_id: String
}

pub struct CreateInferenceServerRequest {
    pub req: HttpRequest,
    pub path: web::Path<CreateInferenceServerPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct ListAllInferenceServersRequest {
    pub req: HttpRequest,
    pub path: web::Path<String>,
    pub query: web::Query<Option<ListAll>>,
    pub body: web::Bytes,
}

pub struct CreateInferenceRequest {
    pub req: HttpRequest,
    pub path: web::Path<CreateInferencePath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct StartInferenceServerRequest {
    pub req: HttpRequest,
    pub path: web::Path<StartInferenceServerPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct RunInferenceRequest {
    pub req: HttpRequest,
    pub path: web::Path<RunInferencePath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub type Labels = HashMap<String, String>;
pub type Description = String;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Kind {
    InferenceServer,
    InferenceServerDeployment,
    Interface,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum InterfaceType {
    Container,
    Model,
    RestApi
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ContainerInterfaceMetadata,
    pub spec: ContainerInterfaceSpec
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Protocol {
    Http,
    Tcp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Port {
    pub name: Option<String>,
    pub port: u16,
    pub protocol: Protocol
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuResourceDefinition {
    pub nvidia: Option<String>,
    pub amd: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourcesDefinition {
    pub cpu: Option<String>,
    pub disk: Option<String>,
    pub memory: Option<String>,
    pub gpu: Option<GpuResourceDefinition>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub limits: Option<ResourcesDefinition>,
    pub requests: Option<ResourcesDefinition>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInterfaceSpec {
    pub image: String,
    pub ports: Option<Vec<Port>>,
    pub resources: Option<Resources>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndpointLabels {
    pub operation_id: String,
    pub labels: Labels
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenApiV3Spec {
    pub endpoint_labels: Option<EndpointLabels>,
    pub spec: OpenAPI
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RestApiInterfaceSpec {
    OpenApiV3(OpenApiV3Spec)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RestApiInterfaceFormat {
    OpenApiV3
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestApiInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestApiInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub format: RestApiInterfaceFormat,
    pub metadata: RestApiInterfaceMetadata,
    pub spec: RestApiInterfaceSpec
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceMetadataSelectors {
    pub match_server_labels: Option<Labels>,
    pub match_endpoint_labels: Option<Labels>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceMetadataDiscoveryCriteria {
    pub platform: String,
    pub confidence: Option<u8>,
    pub criteria: Vec<ModelDiscoveryCriteria>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceMetadata {
    pub name: String,
    pub description: Option<Description>,
    pub labels: Option<Labels>,
    pub discovery_criteria: Option<ModelInterfaceMetadataDiscoveryCriteria>,
    pub selectors: Option<Vec<ModelInterfaceMetadataSelectors>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterfaceSpec {
    pub input: Option<HashMap<String, Value>>,
    pub output: Option<HashMap<String, Value>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInterface {
    pub kind: Kind,
    pub r#type: InterfaceType,
    pub metadata: ModelInterfaceMetadata,
    pub spec: ModelInterfaceSpec,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InferenceServerInterface {
    Container(ContainerInterface),
    RestApi(RestApiInterface),
    Model(ModelInterface),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServerMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub labels: Labels
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServerSpec {
    pub interfaces: Option<Vec<InferenceServerInterface>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServer {
    pub kind: Kind,
    pub metadata: InferenceServerMetadata,
    pub spec: InferenceServerSpec
}