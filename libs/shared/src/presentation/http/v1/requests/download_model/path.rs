use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DownloadModelPath {
    pub artifact_id: String,
}