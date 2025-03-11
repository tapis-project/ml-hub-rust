use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>,
    pub version: Option<String>
}

pub mod artifact_helpers {
    use crate::errors::Error;
    use crate::artifacts::{Archive, StagedArtifact};
    use uuid::Uuid;

    type Header = (String, String);
    type Boundry = String;
    pub struct StagedArtifactResponseHeaders {
        pub content_type_header: Header,
        pub content_disposition_header: Option<Header>,
        pub boundry: Option<Boundry>
    }

    impl StagedArtifactResponseHeaders {
        pub fn new(staged_artifact: &StagedArtifact, download_filename: &Option<String>, archive: &Option<Archive>) -> Result<Self, Error> {
            let is_unarchived_single_file_response = 
                archive.is_none()
                && staged_artifact.artifact.include_paths.clone()
                    .unwrap_or_else(|| Vec::with_capacity(0))
                    .len() == 1;

            let download_filename = download_filename.clone()
                .unwrap_or_else(|| {
                    if let Some(filename) = staged_artifact.path.file_name() {
                        return filename
                            .to_string_lossy()
                            .to_string()
                    }

                    return String::from("artifact")
                });

            // Handle unarchived single-file artifacts
            if is_unarchived_single_file_response {
                let content_disposition_header_value = String::from(format!("attachment; filename=\"{}\"", download_filename));
                return Ok(Self {
                    content_type_header: (String::from("Content-Type"), String::from("application/octet-steam")),
                    content_disposition_header: Some((String::from("Content-Disposition"), content_disposition_header_value)),
                    boundry: None
                })
            }

            // Handle archived artifact headers 
            if archive.is_none() {
                let content_type_value = get_content_type_by_archive(archive)
                    .map(|value| String::from(value))
                    .ok_or_else(|| { Error::new(String::from("Content-Type header could not be resolved from provided archive")) })?;

                return Ok(Self {
                    content_type_header: (String::from("Content-Type"), content_type_value),
                    content_disposition_header: Some((String::from("Content-Disposition"), format!("attachment; filename=\"{}\"", download_filename))),
                    boundry: None
                })
            }
            
            // Handle multipart-mixed headers
            let boundry = String::from(Uuid::now_v7());
            let content_type_header_value = format!("multipart/mixed; boundry={}", boundry);

            
            Ok(Self {
                content_type_header: (String::from("Content-Type"), content_type_header_value),
                // Content disposition is handled in the body so we use None here
                content_disposition_header: None, 
                boundry: Some(boundry)
            })
        }
    }

    pub fn get_content_type_by_archive<'a>(archive: &Option<Archive>) -> Option<&'a str> {
        match archive {
            Some(Archive::Zip) => Some("application/zip"),
            _ => None
        }
    }
}