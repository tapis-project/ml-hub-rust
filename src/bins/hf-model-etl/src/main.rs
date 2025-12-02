use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;
use serde_json::{Value, Map};
use hf_model_etl::database::{get_db, ClientParams};
use hf_model_etl::bootstrap::model_metadata_service_factory;
use hf_model_etl::{CompoundTag, HFModelMetadata};
use shared::application::inputs::task::Task;
use shared::application::inputs::model_metadata::{CreateModelMetadata, ModelMetadata};

#[tokio::main]
async fn main() {
    // Database connection
    let db = get_db(ClientParams{
        username: env::var("ARTIFACTS_DB_USERNAME").expect("ARTIFACTS_DB_USERNAME env var not set"),
        password: env::var("ARTIFACTS_DB_PASSWORD").expect("ARTIFACTS_DB_PASSWORD env var not set"),
        host: env::var("ARTIFACTS_DB_HOST").expect("ARTIFACTS_DB_HOST env var not set"),
        port: env::var("ARTIFACTS_DB_PORT").expect("ARTIFACTS_DB_PORT env var not set"),
        db: env::var("ARTIFACTS_DB_NAME").expect("ARTIFACTS_DB_NAME env var not set"),
    })
        .await
        .map_err(|err| {
            panic!("Database initialization error: {}", err.to_string().as_str()); 
        })
        .expect("Datbase initialization error");

    let max_processable_entries = env::var("MAX_PROCESSABLE_ENTRIES").expect("MAX_PROCESSABLE_ENTRIES env var not set")
        .parse::<i128>().expect("Failed to parse MAX_PROCESSABLE_ENTRIES into an i128");

    let artifact_service = model_metadata_service_factory(&db)
        .await
        .expect("failed to initialize artifact service");

    let inbox_path = env::var("INBOX").expect("INBOX env var not set");

    let inbox = Path::new(&inbox_path);
    if !inbox.is_dir() {
        panic!("Expected inbox path to be a directory")
    }

    // Get the paths of all the files to be processed
    let mut file_paths = vec![];
    match read_dir(inbox) {
        Ok(entries) => {
            for maybe_entry in entries {
                match maybe_entry {
                    Ok(entry) => {
                        println!("{:#?}", &entry);
                        println!("entry path: {:#?}", entry.path());
                        file_paths.push(entry.path())
                    },
                    Err(err) => panic!("Error with dir entry: {}", err.to_string())
                }
            }
        },
        Err(err) => panic!("Error reading dir: {}", err.to_string())
    };

    let mut entries_processed = 0;
    for path in file_paths {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("Error opening file at path '{}': {}", &path.to_string_lossy().to_string().as_str(), err.to_string());
                continue
            }
        };
        let reader = BufReader::new(file);
        for maybe_line in reader.lines() {
            if entries_processed > max_processable_entries {
                return
            }
            entries_processed += 1;
            match maybe_line {
                Ok(line) => {
                    if let Ok(hf_model) = serde_json::from_str::<HFModelMetadata>(line.as_str()) {
                        // Annotations are used here for model provenance
                        let mut annotations = Map::new();
                        let mut canonical = Map::new();
                        let mut locator = Map::new();
                        locator.insert("url".into(), Value::String(format!("https://huggingface.co/{}", &hf_model.id)));
                        canonical.insert("platform".into(), Value::String("huggingface".into()));
                        canonical.insert("locator".into(), Value::Object(locator));
                        canonical.insert("model_id".into(), Value::String(hf_model.id.clone()));
                        canonical.insert("author".into(), Value::String(hf_model.author.clone()));
                        canonical.insert("likes".into(), Value::Number(serde_json::Number::from(hf_model.likes.clone() as u64)));
                        canonical.insert("downloads".into(), Value::Number(serde_json::Number::from(hf_model.downloads.clone() as u64)));
                        canonical.insert("gated".into(), Value::Bool(hf_model.gated.clone()));
                        canonical.insert("private".into(), Value::Bool(hf_model.private.clone()));
                        canonical.insert("sha".into(), Value::String(hf_model.sha.clone()));
                        annotations.insert("canonical".into(), Value::Object(canonical));

                        let keywords: Vec<String> = hf_model.tags.clone();

                        // Task types derived from the keywords. The "pipeline_tag"
                        // property will be the authroitative soure for the task type 
                        // if none are found
                        let mut derived_task_types: Vec<Task> = Vec::new();
                        for keyword in keywords.clone() {
                            match Task::try_from(Task::normalize_string(keyword).as_str()) {
                                Ok(t) => derived_task_types.push(t),
                                Err(_) => continue // Ignore as they keyword cannot be interpreted as a task type
                            }
                        }
                        
                        // Compound tags are huggingface keywords whose value contains the ":" char.
                        // From these compund tags we can derive properties we are interested in like
                        // license and task type
                        let compound_tags = hf_model.parse_compound_tags();

                        // Derive the license
                        let license = compound_tags.iter()
                            .filter(|ct| ct.name == "license")
                            .collect::<Vec<&CompoundTag>>()
                            .first()
                            .and_then(|ct| Some(ct.value.clone()));

                        // Convert pipeline tag to a variant of the task type enum.
                        let mut task_types: Vec<Task> = derived_task_types;
                        match Task::try_from(Task::normalize_string(hf_model.pipeline_tag.clone()).as_str()) {
                            Ok(t) => {
                                if !task_types.contains(&t) {
                                    task_types.push(t)
                                }
                            },
                            Err(err) => {
                                eprintln!("Failed to convert pipeline tag '{}' to Task for model {}: {}", &hf_model.pipeline_tag, &hf_model.id, err.to_string());
                                continue;
                            }
                        };

                        // Determine which python libraries this model can be used with
                        let mut libraries: Vec<String> = Vec::new();
                        let known_libs: &[String] = &["transformers".into(), "diffusers".into(), "tensorflow".into(), "pytorch".into()];
                        for lib in known_libs {
                            if keywords.contains(lib) && !libraries.contains(lib) {
                                libraries.push(lib.clone())
                            }
                        }

                        // Parse the model name from the model's id
                        let name = match hf_model.get_model_name() {
                            Ok(n) => n,
                            Err(err) => {
                                eprintln!("{}", err.to_string());
                                continue
                            }
                        };

                        match artifact_service.create_model_metadata(CreateModelMetadata {
                            metadata: ModelMetadata {
                                name: Some(name),
                                annotations: Some(Value::Object(annotations)),
                                author: Some(format!("_{}", hf_model.author)),
                                model_inputs: None,
                                model_outputs: None,
                                model_type: None,
                                libraries: Some(libraries),
                                image: None,
                                keywords: Some(keywords),
                                multi_modal: None,
                                task_types: Some(task_types),
                                inference_distributed: None,
                                inference_hardware: None,
                                inference_max_compute_utilization_percentage: None,
                                inference_max_energy_consumption_watts: None,
                                inference_max_latency_ms: None,
                                inference_max_memory_usage_mb: None,
                                inference_min_throughput: None,
                                inference_precision: None,
                                inference_software_dependencies: None,
                                training_distributed: None,
                                training_hardware: None,
                                training_max_energy_consumption_watts: None,
                                training_precision: None,
                                training_time: None,
                                pretrained: None,
                                pretraining_datasets: None,
                                finetuning_datasets: None,
                                edge_optimized: None,
                                quantization_aware: None,
                                supports_quantization: None,
                                pruned: None,
                                slimmed: None,
                                regulatory: None,
                                license,
                                bias_evaluation_score: None,
                            }
                        }).await {
                            Ok(_) => (),
                            Err(err) => {
                                eprintln!("Error saving metadata to the database: {}", err.to_string())
                            }
                        }
                    };
                },
                Err(err) => {
                    eprintln!("Error reading line: {}", err.to_string());
                    continue
                }
            }
        }
    };
}
