use reqwest::blocking::Client as ReqwestClient;
use clients::artifacts::ArtifactGenerator;
// use shared::requests::param_to_string;
use shared::logging::SharedLogger;

#[derive(Debug)]
pub struct S3Client {
    _client: ReqwestClient,
    _logger: SharedLogger
}

impl ArtifactGenerator for S3Client {}

impl S3Client {
    pub fn new() -> Self {
        Self {
            _client: ReqwestClient::new(),
            _logger: SharedLogger::new(),
        }
    }
}
