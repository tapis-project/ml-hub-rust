use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{HardwareRequirements, ModelIo};
/**You should use this struct via [`MlHubModelsClient::create_model_metadata`].

On request success, this will return a [`CreateModelMetadataResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelMetadataRequest {
    pub artifact_id: String,
    pub bias_evaluation_score: Option<i64>,
    pub edge_optimized: Option<bool>,
    pub finetuning_datasets: Option<Vec<String>>,
    pub framework: Option<String>,
    pub image: Option<String>,
    pub inference_distributed: Option<bool>,
    pub inference_hardware: Option<HardwareRequirements>,
    pub inference_max_compute_utilization_percentage: Option<i64>,
    pub inference_max_energy_consumption_watts: Option<i64>,
    pub inference_max_latency_ms: Option<i64>,
    pub inference_max_memory_usage_mb: Option<i64>,
    pub inference_min_throughput: Option<i64>,
    pub inference_precision: Option<String>,
    pub inference_software_dependencies: Option<Vec<String>>,
    pub label_map: Option<serde_json::Value>,
    pub labels: Option<Vec<String>>,
    pub license: Option<String>,
    pub model_inputs: Option<Vec<ModelIo>>,
    pub model_outputs: Option<Vec<ModelIo>>,
    pub model_type: Option<String>,
    pub multi_modal: Option<bool>,
    pub name: Option<String>,
    pub pretrained: Option<bool>,
    pub pretraining_datasets: Option<Vec<String>>,
    pub pruned: Option<bool>,
    pub quantization_aware: Option<bool>,
    pub regulatory: Option<Vec<String>>,
    pub slimmed: Option<bool>,
    pub supports_quantization: Option<bool>,
    pub task_types: Option<Vec<String>>,
    pub training_distributed: Option<bool>,
    pub training_hardware: Option<HardwareRequirements>,
    pub training_max_energy_consumption_watts: Option<i64>,
    pub training_precision: Option<String>,
    pub training_time: Option<i64>,
    pub version: Option<String>,
}
impl FluentRequest<'_, CreateModelMetadataRequest> {
    ///Set the value of the bias_evaluation_score field.
    pub fn bias_evaluation_score(mut self, bias_evaluation_score: i64) -> Self {
        self.params.bias_evaluation_score = Some(bias_evaluation_score);
        self
    }
    ///Set the value of the edge_optimized field.
    pub fn edge_optimized(mut self, edge_optimized: bool) -> Self {
        self.params.edge_optimized = Some(edge_optimized);
        self
    }
    ///Set the value of the finetuning_datasets field.
    pub fn finetuning_datasets(
        mut self,
        finetuning_datasets: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.finetuning_datasets = Some(
            finetuning_datasets.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the framework field.
    pub fn framework(mut self, framework: &str) -> Self {
        self.params.framework = Some(framework.to_owned());
        self
    }
    ///Set the value of the image field.
    pub fn image(mut self, image: &str) -> Self {
        self.params.image = Some(image.to_owned());
        self
    }
    ///Set the value of the inference_distributed field.
    pub fn inference_distributed(mut self, inference_distributed: bool) -> Self {
        self.params.inference_distributed = Some(inference_distributed);
        self
    }
    ///Set the value of the inference_hardware field.
    pub fn inference_hardware(
        mut self,
        inference_hardware: HardwareRequirements,
    ) -> Self {
        self.params.inference_hardware = Some(inference_hardware);
        self
    }
    ///Set the value of the inference_max_compute_utilization_percentage field.
    pub fn inference_max_compute_utilization_percentage(
        mut self,
        inference_max_compute_utilization_percentage: i64,
    ) -> Self {
        self.params.inference_max_compute_utilization_percentage = Some(
            inference_max_compute_utilization_percentage,
        );
        self
    }
    ///Set the value of the inference_max_energy_consumption_watts field.
    pub fn inference_max_energy_consumption_watts(
        mut self,
        inference_max_energy_consumption_watts: i64,
    ) -> Self {
        self.params.inference_max_energy_consumption_watts = Some(
            inference_max_energy_consumption_watts,
        );
        self
    }
    ///Set the value of the inference_max_latency_ms field.
    pub fn inference_max_latency_ms(mut self, inference_max_latency_ms: i64) -> Self {
        self.params.inference_max_latency_ms = Some(inference_max_latency_ms);
        self
    }
    ///Set the value of the inference_max_memory_usage_mb field.
    pub fn inference_max_memory_usage_mb(
        mut self,
        inference_max_memory_usage_mb: i64,
    ) -> Self {
        self.params.inference_max_memory_usage_mb = Some(inference_max_memory_usage_mb);
        self
    }
    ///Set the value of the inference_min_throughput field.
    pub fn inference_min_throughput(mut self, inference_min_throughput: i64) -> Self {
        self.params.inference_min_throughput = Some(inference_min_throughput);
        self
    }
    ///Set the value of the inference_precision field.
    pub fn inference_precision(mut self, inference_precision: &str) -> Self {
        self.params.inference_precision = Some(inference_precision.to_owned());
        self
    }
    ///Set the value of the inference_software_dependencies field.
    pub fn inference_software_dependencies(
        mut self,
        inference_software_dependencies: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.inference_software_dependencies = Some(
            inference_software_dependencies
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    ///Set the value of the label_map field.
    pub fn label_map(mut self, label_map: serde_json::Value) -> Self {
        self.params.label_map = Some(label_map);
        self
    }
    ///Set the value of the labels field.
    pub fn labels(mut self, labels: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.params.labels = Some(
            labels.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the license field.
    pub fn license(mut self, license: &str) -> Self {
        self.params.license = Some(license.to_owned());
        self
    }
    ///Set the value of the model_inputs field.
    pub fn model_inputs(mut self, model_inputs: Vec<ModelIo>) -> Self {
        self.params.model_inputs = Some(model_inputs);
        self
    }
    ///Set the value of the model_outputs field.
    pub fn model_outputs(mut self, model_outputs: Vec<ModelIo>) -> Self {
        self.params.model_outputs = Some(model_outputs);
        self
    }
    ///Set the value of the model_type field.
    pub fn model_type(mut self, model_type: &str) -> Self {
        self.params.model_type = Some(model_type.to_owned());
        self
    }
    ///Set the value of the multi_modal field.
    pub fn multi_modal(mut self, multi_modal: bool) -> Self {
        self.params.multi_modal = Some(multi_modal);
        self
    }
    ///Set the value of the name field.
    pub fn name(mut self, name: &str) -> Self {
        self.params.name = Some(name.to_owned());
        self
    }
    ///Set the value of the pretrained field.
    pub fn pretrained(mut self, pretrained: bool) -> Self {
        self.params.pretrained = Some(pretrained);
        self
    }
    ///Set the value of the pretraining_datasets field.
    pub fn pretraining_datasets(
        mut self,
        pretraining_datasets: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.pretraining_datasets = Some(
            pretraining_datasets.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the pruned field.
    pub fn pruned(mut self, pruned: bool) -> Self {
        self.params.pruned = Some(pruned);
        self
    }
    ///Set the value of the quantization_aware field.
    pub fn quantization_aware(mut self, quantization_aware: bool) -> Self {
        self.params.quantization_aware = Some(quantization_aware);
        self
    }
    ///Set the value of the regulatory field.
    pub fn regulatory(
        mut self,
        regulatory: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.regulatory = Some(
            regulatory.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the slimmed field.
    pub fn slimmed(mut self, slimmed: bool) -> Self {
        self.params.slimmed = Some(slimmed);
        self
    }
    ///Set the value of the supports_quantization field.
    pub fn supports_quantization(mut self, supports_quantization: bool) -> Self {
        self.params.supports_quantization = Some(supports_quantization);
        self
    }
    ///Set the value of the task_types field.
    pub fn task_types(
        mut self,
        task_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.task_types = Some(
            task_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the training_distributed field.
    pub fn training_distributed(mut self, training_distributed: bool) -> Self {
        self.params.training_distributed = Some(training_distributed);
        self
    }
    ///Set the value of the training_hardware field.
    pub fn training_hardware(mut self, training_hardware: HardwareRequirements) -> Self {
        self.params.training_hardware = Some(training_hardware);
        self
    }
    ///Set the value of the training_max_energy_consumption_watts field.
    pub fn training_max_energy_consumption_watts(
        mut self,
        training_max_energy_consumption_watts: i64,
    ) -> Self {
        self.params.training_max_energy_consumption_watts = Some(
            training_max_energy_consumption_watts,
        );
        self
    }
    ///Set the value of the training_precision field.
    pub fn training_precision(mut self, training_precision: &str) -> Self {
        self.params.training_precision = Some(training_precision.to_owned());
        self
    }
    ///Set the value of the training_time field.
    pub fn training_time(mut self, training_time: i64) -> Self {
        self.params.training_time = Some(training_time);
        self
    }
    ///Set the value of the version field.
    pub fn version(mut self, version: &str) -> Self {
        self.params.version = Some(version.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreateModelMetadataRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreateModelMetadataResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/artifacts/{artifact_id}/metadata", artifact_id = self.params
                .artifact_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.bias_evaluation_score {
                r = r.json(serde_json::json!({ "bias_evaluation_score" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.edge_optimized {
                r = r.json(serde_json::json!({ "edge_optimized" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.finetuning_datasets {
                r = r.json(serde_json::json!({ "finetuning_datasets" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.framework {
                r = r.json(serde_json::json!({ "framework" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.image {
                r = r.json(serde_json::json!({ "image" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.inference_distributed {
                r = r.json(serde_json::json!({ "inference_distributed" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.inference_hardware {
                r = r.json(serde_json::json!({ "inference_hardware" : unwrapped }));
            }
            if let Some(ref unwrapped) = self
                .params
                .inference_max_compute_utilization_percentage
            {
                r = r
                    .json(
                        serde_json::json!(
                            { "inference_max_compute_utilization_percentage" : unwrapped
                            }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self
                .params
                .inference_max_energy_consumption_watts
            {
                r = r
                    .json(
                        serde_json::json!(
                            { "inference_max_energy_consumption_watts" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.inference_max_latency_ms {
                r = r
                    .json(serde_json::json!({ "inference_max_latency_ms" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.inference_max_memory_usage_mb {
                r = r
                    .json(
                        serde_json::json!(
                            { "inference_max_memory_usage_mb" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.inference_min_throughput {
                r = r
                    .json(serde_json::json!({ "inference_min_throughput" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.inference_precision {
                r = r.json(serde_json::json!({ "inference_precision" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.inference_software_dependencies {
                r = r
                    .json(
                        serde_json::json!(
                            { "inference_software_dependencies" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.label_map {
                r = r.json(serde_json::json!({ "label_map" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.labels {
                r = r.json(serde_json::json!({ "labels" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.license {
                r = r.json(serde_json::json!({ "license" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.model_inputs {
                r = r.json(serde_json::json!({ "model_inputs" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.model_outputs {
                r = r.json(serde_json::json!({ "model_outputs" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.model_type {
                r = r.json(serde_json::json!({ "model_type" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.multi_modal {
                r = r.json(serde_json::json!({ "multi_modal" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.name {
                r = r.json(serde_json::json!({ "name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.pretrained {
                r = r.json(serde_json::json!({ "pretrained" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.pretraining_datasets {
                r = r.json(serde_json::json!({ "pretraining_datasets" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.pruned {
                r = r.json(serde_json::json!({ "pruned" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.quantization_aware {
                r = r.json(serde_json::json!({ "quantization_aware" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.regulatory {
                r = r.json(serde_json::json!({ "regulatory" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.slimmed {
                r = r.json(serde_json::json!({ "slimmed" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.supports_quantization {
                r = r.json(serde_json::json!({ "supports_quantization" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.task_types {
                r = r.json(serde_json::json!({ "task_types" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.training_distributed {
                r = r.json(serde_json::json!({ "training_distributed" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.training_hardware {
                r = r.json(serde_json::json!({ "training_hardware" : unwrapped }));
            }
            if let Some(ref unwrapped) = self
                .params
                .training_max_energy_consumption_watts
            {
                r = r
                    .json(
                        serde_json::json!(
                            { "training_max_energy_consumption_watts" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.training_precision {
                r = r.json(serde_json::json!({ "training_precision" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.training_time {
                r = r.json(serde_json::json!({ "training_time" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.version {
                r = r.json(serde_json::json!({ "version" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Create metadata for a model artifact
    pub fn create_model_metadata(
        &self,
        artifact_id: &str,
    ) -> FluentRequest<'_, CreateModelMetadataRequest> {
        FluentRequest {
            client: self,
            params: CreateModelMetadataRequest {
                artifact_id: artifact_id.to_owned(),
                bias_evaluation_score: None,
                edge_optimized: None,
                finetuning_datasets: None,
                framework: None,
                image: None,
                inference_distributed: None,
                inference_hardware: None,
                inference_max_compute_utilization_percentage: None,
                inference_max_energy_consumption_watts: None,
                inference_max_latency_ms: None,
                inference_max_memory_usage_mb: None,
                inference_min_throughput: None,
                inference_precision: None,
                inference_software_dependencies: None,
                label_map: None,
                labels: None,
                license: None,
                model_inputs: None,
                model_outputs: None,
                model_type: None,
                multi_modal: None,
                name: None,
                pretrained: None,
                pretraining_datasets: None,
                pruned: None,
                quantization_aware: None,
                regulatory: None,
                slimmed: None,
                supports_quantization: None,
                task_types: None,
                training_distributed: None,
                training_hardware: None,
                training_max_energy_consumption_watts: None,
                training_precision: None,
                training_time: None,
                version: None,
            },
        }
    }
}
