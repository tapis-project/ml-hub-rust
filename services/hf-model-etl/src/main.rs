use client_provider::ClientProvider;
use clients::ClientError;
use hf_model_etl::bootstrap::{
    build_deployment_strategy_provider, external_model_ingestion_service_factory,
};
use hf_model_etl::database::{initialize_client, ClientParams};
use log::error;
use serde_json::Value;
use std::env;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Database connection
    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");
    let client = initialize_client(ClientParams {
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set"),
        replica_set: Some(
            env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set"),
        ),
    })
    .await
    .map_err(|e| {
        error!("Database initialization error: {}", e.to_string().as_str());
    })
    .expect("Datbase initialization error");

    let max_processable_entries = env::var("MAX_PROCESSABLE_ENTRIES")
        .expect("MAX_PROCESSABLE_ENTRIES env var not set")
        .parse::<i128>()
        .expect("Failed to parse MAX_PROCESSABLE_ENTRIES into an i128");

    let deployment_strategy_provider = build_deployment_strategy_provider();

    let client_strategy_sets = match deployment_strategy_provider {
        Ok(p) => Arc::new(p.list_all().await.clone()),
        Err(e) => {
            error!(
                "Deployment strategy provider initialization error: {}",
                e.to_string().as_str()
            );
            panic!()
        }
    };

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
                    Ok(entry) => file_paths.push(entry.path()),
                    Err(e) => panic!("Error with dir entry: {}", e.to_string()),
                }
            }
        }
        Err(e) => panic!("Error reading dir: {}", e.to_string()),
    };

    let ingestion_service =
        external_model_ingestion_service_factory(&client, db_name, client_strategy_sets);

    // Fetch the Hugging Face model conversion client from the client provider.
    let huggingface_client = ClientProvider::provide_model_conversion_client("hugging-face")
        .expect("HuggingfaceClient provided");

    let mut entries_processed = 0;
    for path in file_paths {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "Error opening file at path '{}': {}",
                    &path.to_string_lossy().to_string().as_str(),
                    e.to_string()
                );
                continue;
            }
        };

        let reader = BufReader::new(file);
        for maybe_line in reader.lines() {
            if entries_processed > max_processable_entries {
                return;
            }
            entries_processed += 1;
            match maybe_line {
                Ok(line) => {
                    if let Ok(hf_model) = serde_json::from_str::<Value>(line.as_str()) {
                        let external_model =
                            match huggingface_client.from_platform_metadata(hf_model) {
                                Ok(m) => m,
                                Err(e) => match e {
                                    ClientError::Unimplemented => {
                                        eprintln!("Model conversion client not implemented");
                                        return;
                                    }
                                    _ => {
                                        eprintln!("Error converting model: {}", e.to_string());
                                        continue;
                                    }
                                },
                            };

                        match ingestion_service
                            .ingest_external_model(external_model)
                            .await
                        {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("Error saving model to the database: {}", e.to_string());
                                continue;
                            }
                        }
                    };
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e.to_string());
                    continue;
                }
            }
        }
    }
}
