use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;
use serde_json::Value;
use hf_model_etl::database::{initialize_client, ClientParams};
use hf_model_etl::bootstrap::{build_deployment_strategy_provider, model_metadata_service_factory};
use shared::application::inputs::model_metadata::CreateModelMetadata;
use client_provider::ClientProvider;
use clients::ClientError;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Database connection
    let db_name = env::var("MONGO_NAME").expect("MONGO_NAME env var not set");
    let client = initialize_client(ClientParams{
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: env::var("MONGO_NAME").expect("MONGO_NAME env var not set"),
        replica_set: Some(env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set")),
    })
        .await
        .map_err(|err| {
            panic!("Database initialization error: {}", err.to_string().as_str()); 
        })
        .expect("Datbase initialization error");

    let max_processable_entries = env::var("MAX_PROCESSABLE_ENTRIES").expect("MAX_PROCESSABLE_ENTRIES env var not set")
        .parse::<i128>().expect("Failed to parse MAX_PROCESSABLE_ENTRIES into an i128");

    let deployment_strategy_provider = build_deployment_strategy_provider();

    let client_strategy_sets = match deployment_strategy_provider {
        Ok(p) => Arc::new(p.provide().clone()),
        Err(_) => {
            // TODO Log the error
            Arc::new(vec![])
        }
    };

    let artifact_service = model_metadata_service_factory(&client, db_name, client_strategy_sets)
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

    // Fetch the huggingface model metadata conversion client from the client provider
    let huggingface_client = ClientProvider
        ::provide_model_metadata_conversion_client("huggingface")
        .expect("HuggingfaceClient provided");

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
                    if let Ok(hf_model) = serde_json::from_str::<Value>(line.as_str()) {
                        let metadata = match huggingface_client.from_platform_metadata(hf_model) {
                            Ok(m) => m,
                            Err(err) => {
                                match err {
                                    ClientError::Unimplemented => {
                                        eprintln!("Metadata client not implemented");
                                        return 
                                    },
                                    _ => {
                                        eprintln!("Error converting metadata: {}", err.to_string());
                                        continue
                                    }
                                }
                            }
                        };
                        match artifact_service.create_model_metadata(CreateModelMetadata { metadata }).await {
                            Ok(_) => (),
                            Err(err) => {
                                eprintln!("Error saving metadata to the database: {}", err.to_string());
                                continue
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
