use clients::{ClientError, ClientErrorScope};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct HFModel {
    pub author: Option<String>,
    pub id: String,
    pub library_name: Option<String>,
    pub pipeline_tag: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub gated: bool,
    #[serde(default)]
    pub private: bool,
    pub likes: Option<u128>,
    pub downloads: Option<u128>,
    pub sha: String,
    #[serde(default)]
    pub siblings: Vec<HFModelFile>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct HFModelFile {
    pub size: Option<u64>,
}

pub struct CompoundTag {
    pub name: String,
    pub value: String,
}

impl HFModel {
    pub fn parse_compound_tags(&self) -> Vec<CompoundTag> {
        let mut hf_tags: Vec<CompoundTag> = Vec::new();
        for tag in self.tags.clone() {
            let parts: Vec<String> =
                tag.clone().split(":").map(|s| String::from(s)).collect();

            if parts.len() >= 2 {
                let name = if let Some(n) = parts.get(0) {
                    n.clone()
                } else {
                    continue;
                };

                let value = parts[1..].to_vec().join(":");

                hf_tags.push(CompoundTag { name, value })
            }
        }

        hf_tags
    }

    pub fn get_model_name(&self) -> Result<String, ClientError> {
        let parts: Vec<String> = self
            .id
            .clone()
            .split("/")
            .into_iter()
            .map(|p| String::from(p))
            .collect();

        match parts.get(1) {
            Some(p) => Ok(String::from(p)),
            None => Err(ClientError::Internal{
                msg: format!("Expected there to be a '/' in the model's id but none found. Found '{}'", &self.id),
                scope: ClientErrorScope::Server
            })
        }
    }
}
