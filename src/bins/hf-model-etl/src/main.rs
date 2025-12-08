use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;
use serde_json::Value;
use hf_model_etl::database::{get_db, ClientParams};
use hf_model_etl::bootstrap::model_metadata_service_factory;
use shared::application::inputs::model_metadata::CreateModelMetadata;
use client_provider::ClientProvider;

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
                                eprintln!("Error converting metadata: {}", err.to_string());
                                continue
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
