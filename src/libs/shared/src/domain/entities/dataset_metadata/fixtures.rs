use crate::domain::entities::{dataset_metadata::{HardwareRequirements, DatasetIO, DatasetMetadata, Accelerator, SystemRequirement}, task::Task};
use serde_json::json;

pub fn full_dataset_metadata() -> DatasetMetadata {
    DatasetMetadata {
        name: Some("foo".into()),
        annotations: Some(json!({
            "canonical": {
                "gated": false,
                "private": true
            }
        })),
        author: Some("bar".into()),
        dataset_inputs: Some(
            vec![DatasetIO {
                data_type: Some("f64".into()),
                shape: Some(vec![1000, 1000, 3])
            }]
        ),
        dataset_outputs: Some(
            vec![DatasetIO {
                data_type: Some("f64".into()),
                shape: Some(vec![1000, 1000, 3])
            }]
        ),
        dataset_type: Some("cnn".into()),
        libraries: Some(vec!["transformers".into(), "diffusers".into()]),
        image: Some("dockerhub://my/dataset".into()),
        keywords: Some(vec!["text-generation".into(), "transformers".into()]),
        multi_modal: Some(true),
        task_types: Some(vec![Task::ImageClassification]),
        inference_distributed: Some(true),
        inference_hardware: Some(
            HardwareRequirements {
                cpus: Some(2),
                memory_gb: Some(5),
                disk_gb: Some(200),
                accelerators: Some(vec![
                    Accelerator {
                        memory_gb: Some(5),
                        accelerator_type: "gpu".into(),
                        cores: Some(2),
                        system_requirements: vec![
                            SystemRequirement {
                                name: "cuda".into(),
                                version: "1.2.3".into(),
                            }
                        ]
                    }
                ]),
                architectures: Some(vec!["x86".into()]),
            }
        ),
        inference_max_compute_utilization_percentage: Some(45),
        inference_max_energy_consumption_watts: Some(60),
        inference_max_latency_ms: Some(100),
        inference_max_memory_usage_mb: Some(64),
        inference_min_throughput: Some(20),
        inference_precision: Some("fp64".into()),
        inference_software_dependencies: Some(vec!["transformers".into()]),
        training_distributed: Some(true),
        training_hardware: Some(
            HardwareRequirements {
                cpus: Some(2),
                memory_gb: Some(5),
                disk_gb: Some(200),
                accelerators: Some(vec![
                    Accelerator {
                        memory_gb: Some(5),
                        accelerator_type: "gpu".into(),
                        cores: Some(2),
                        system_requirements: vec![
                            SystemRequirement {
                                name: "cuda".into(),
                                version: "1.2.3".into(),
                            }
                        ]
                    }
                ]),
                architectures: Some(vec!["x86".into()]),
            }
        ),
        training_max_energy_consumption_watts: Some(3600),
        training_precision: Some("f64".into()),
        training_time: Some(3600),
        pretrained: Some(true),
        pretraining_datasets: Some(vec!["FOO".into()]),
        finetuning_datasets: Some(vec!["FOO".into()]),
        edge_optimized: Some(false),
        quantization_aware: Some(true),
        supports_quantization: Some(true),
        pruned: Some(true),
        slimmed: Some(false),
        regulatory: Some(vec!["HIPPA".into()]),
        license: Some("MIT".into()),
        bias_evaluation_score: Some(-1),
    }
}