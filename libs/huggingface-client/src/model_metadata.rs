use serde::Deserialize;
use clients::{ClientError, ClientErrorScope};


#[derive(Deserialize)]
pub struct HFModelMetadata {
    pub author: String,
    pub id: String,
    pub library_name: String,
    pub pipeline_tag: String,
    pub tags: Vec<String>,
    pub gated: bool,
    pub private: bool,
    pub likes: u128,
    pub downloads: u128,
    pub sha: String,
}

pub struct CompoundTag {
    pub name: String,
    pub value: String
}

impl HFModelMetadata {
    pub fn parse_compound_tags(&self) -> Vec<CompoundTag> {
        let mut hf_tags:Vec<CompoundTag> = Vec::new();
        for tag in self.tags.clone() {
            let parts: Vec<String> = tag.clone()
                .split(":")
                .map(|s| String::from(s))
                .collect();


            if parts.len() >= 2 {
                let name = if let Some(n) = parts.get(0) { n.clone() } else { continue };
                let value = parts[1..].to_vec().join(":");
                
                hf_tags.push(CompoundTag {
                    name,
                    value
                })
            }
        }

        hf_tags
    }
    pub fn get_model_name(&self) -> Result<String, ClientError> {
        let parts: Vec<String> = self.id.clone()
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