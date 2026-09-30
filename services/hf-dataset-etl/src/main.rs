use std::{
    env,
    error::Error,
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use hf_dataset_etl::{
    bootstrap::{
        dataset_query_service_factory, dataset_registration_service_factory,
        dataset_repository_factory,
    },
    database::{initialize_client, ClientParams},
    transform::HuggingFaceDatasetRecord,
};
use shared::{
    application::{
        inputs::dataset::{DatasetProviderInput, RegisterDatasetInput},
        services::{
            dataset_query_service::DatasetQueryService,
            dataset_registration_service::DatasetRegistrationService,
        },
    },
    shared_kernel::context::RequestContext,
};

#[derive(Default)]
struct ProcessingSummary {
    processed: usize,
    registered: usize,
    skipped_existing: usize,
    rejected: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_name = required_env("MONGO_DBNAME")?;
    let client = initialize_client(ClientParams {
        username: required_env("MONGO_USERNAME")?,
        password: required_env("MONGO_PASSWORD")?,
        host: required_env("MONGO_HOST")?,
        port: required_env("MONGO_PORT")?,
        db: db_name.clone(),
        replica_set: Some(required_env("MONGO_REPLICA_SET")?),
    })
    .await?;

    let inbox_path = required_env("INBOX")?;
    let max_entries = parse_max_entries(&required_env("MAX_PROCESSABLE_ENTRIES")?)?;

    let dataset_repository = dataset_repository_factory(&client, db_name);
    let dataset_registration_service =
        dataset_registration_service_factory(dataset_repository.clone());
    let dataset_query_service = dataset_query_service_factory(dataset_repository);

    let context = RequestContext::system(None);

    let summary = process_inbox(
        Path::new(&inbox_path),
        max_entries,
        &dataset_registration_service,
        &dataset_query_service,
        &context,
    )
    .await?;

    println!(
        "Hugging Face Dataset ETL complete: processed={}, registered={}, skipped_existing={}, rejected={}",
        summary.processed, summary.registered, summary.skipped_existing, summary.rejected
    );

    Ok(())
}

async fn process_inbox(
    inbox: &Path,
    max_entries: Option<usize>,
    dataset_registration_service: &DatasetRegistrationService,
    dataset_query_service: &DatasetQueryService,
    context: &RequestContext,
) -> Result<ProcessingSummary, Box<dyn Error>> {
    if !inbox.is_dir() {
        return Err(format!("INBOX is not a directory: {}", inbox.display()).into());
    }

    let mut paths = read_dir(inbox)?
        .filter_map(|entry| entry.ok().map(|value| value.path()))
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "jsonl")
        })
        .collect::<Vec<PathBuf>>();

    paths.sort();

    let mut summary = ProcessingSummary::default();

    for path in paths {
        let reader = BufReader::new(File::open(&path)?);

        for line in reader.lines() {
            if max_entries.is_some_and(|limit| summary.processed >= limit) {
                return Ok(summary);
            }

            summary.processed += 1;

            let record = match line
                .map_err(Into::<Box<dyn Error>>::into)
                .and_then(|value| {
                    serde_json::from_str::<HuggingFaceDatasetRecord>(&value).map_err(Into::into)
                }) {
                Ok(record) => record,
                Err(error) => {
                    summary.rejected += 1;
                    eprintln!("Rejected record from {}: {error}", path.display());

                    continue;
                }
            };

            let input = match RegisterDatasetInput::try_from(record) {
                Ok(input) => input,
                Err(error) => {
                    summary.rejected += 1;
                    eprintln!("Rejected record from {}: {error}", path.display());

                    continue;
                }
            };

            let (repo_id, sha) = match &input.provider {
                DatasetProviderInput::HuggingFace(locator) => (&locator.id, &locator.sha),
                DatasetProviderInput::Tapis(_) => {
                    unreachable!("ETL only creates Hugging Face inputs")
                }
            };

            if dataset_query_service
                .find_by_huggingface_repo_locator(context, repo_id, sha)
                .await?
                .is_some()
            {
                summary.skipped_existing += 1;

                continue;
            }

            match dataset_registration_service
                .register_dataset(context, input)
                .await
            {
                Ok(_) => summary.registered += 1,
                Err(error) => {
                    summary.rejected += 1;
                    eprintln!(
                        "Failed to register Dataset from {}: {error}",
                        path.display()
                    );
                }
            }
        }
    }

    Ok(summary)
}

fn required_env(name: &str) -> Result<String, Box<dyn Error>> {
    env::var(name).map_err(|_| format!("{name} env var not set").into())
}

fn parse_max_entries(value: &str) -> Result<Option<usize>, Box<dyn Error>> {
    if value == "-1" {
        return Ok(None);
    }

    Ok(Some(value.parse()?))
}

#[cfg(test)]
#[path = "main.test.rs"]
mod main_test;
