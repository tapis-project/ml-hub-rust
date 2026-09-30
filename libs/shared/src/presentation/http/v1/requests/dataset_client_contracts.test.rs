use std::collections::HashMap;

use bytes::Bytes;

use super::{
    artifacts::{DownloadArtifactBody, IngestArtifactRequest},
    common::headers::Headers,
    download_dataset::{path::DownloadDatasetPath, DownloadDatasetRequest},
    get_dataset_by_platform::{path::GetDatasetByPlatformPath, GetDatasetByPlatformRequest},
    ingest_dataset::{path::IngestDatasetPath, IngestDatasetRequest},
    list_datasets_by_platform::{path::ListDatasetsByPlatformPath, ListDatasetsByPlatformRequest},
    publish_dataset::{path::PublishDatasetPath, PublishDatasetRequest},
};

#[test]
fn dataset_client_paths_preserve_provider_and_dataset_identifiers(
) -> Result<(), Box<dyn std::error::Error>> {
    let get_path = GetDatasetByPlatformPath {
        platform: "HuggingFace".into(),
        dataset_id: "owner/dataset".into(),
    };

    let serialized = serde_json::to_value(get_path)?;

    assert_eq!(serialized["platform"], "HuggingFace");
    assert_eq!(serialized["dataset_id"], "owner/dataset");

    Ok(())
}

#[test]
fn dataset_client_request_contracts_remain_available() {
    let list_request = ListDatasetsByPlatformRequest {
        headers: Headers::new(Vec::new()),
        path: ListDatasetsByPlatformPath {
            platform: "HuggingFace".into(),
        },
        query: HashMap::new(),
        body: Bytes::new(),
    };

    let get_request = GetDatasetByPlatformRequest {
        headers: Headers::new(Vec::new()),
        path: GetDatasetByPlatformPath {
            platform: "HuggingFace".into(),
            dataset_id: "owner/dataset".into(),
        },
        query: HashMap::new(),
        body: Bytes::new(),
    };

    let ingest_request = IngestDatasetRequest {
        headers: Headers::new(Vec::new()),
        path: IngestDatasetPath {
            platform: "Git".into(),
            dataset_id: "owner/dataset".into(),
        },
        query: HashMap::new(),
        body: IngestArtifactRequest {
            include_paths: None,
            exclude_paths: None,
            webhook_url: None,
            params: None,
        },
    };

    let download_request = DownloadDatasetRequest {
        headers: Headers::new(Vec::new()),
        path: DownloadDatasetPath {
            platform: "Git".into(),
            dataset_id: "owner/dataset".into(),
        },
        query: HashMap::new(),
        body: DownloadArtifactBody {
            download_filename: None,
            params: None,
        },
    };

    let publish_path = PublishDatasetPath {
        platform: "HuggingFace".into(),
        dataset_id: "owner/dataset".into(),
    };

    let publish_request_type: Option<PublishDatasetRequest> = None;

    assert_eq!(list_request.path.platform, "HuggingFace");
    assert_eq!(get_request.path.dataset_id, "owner/dataset");
    assert_eq!(ingest_request.path.platform, "Git");
    assert_eq!(download_request.path.dataset_id, "owner/dataset");
    assert_eq!(publish_path.platform, "HuggingFace");
    assert!(publish_request_type.is_none());
}
