use std::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use openapiv3::OpenAPI;
use mongodb::bson::oid::ObjectId;

pub type Labels = HashMap<String, String>;

pub type Description = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Kind {
    InferenceServer,
    InferenceServerDeployment,
    Interface,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub criteria: Vec<ModelMetadata>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub kind: Kind,
    pub metadata: InferenceServerMetadata,
    pub spec: InferenceServerSpec
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelMetadata {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceServerDeployment {

}