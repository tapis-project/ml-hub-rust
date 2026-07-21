use serde::{Deserialize, Serialize};

pub mod bootstrap;
pub mod database;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatraAiModel {
    pub model_id: Option<u128>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub location: Option<String>,
    pub license: Option<String>,
    pub framework: Option<String>,
    pub model_type: Option<String>,
    pub test_accuracy: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatraModelMetadata {
    pub id: Option<u128>,
    pub uuid: String,
    pub name: String,
    pub version: Option<String>,
    pub short_description: Option<String>,
    pub full_description: Option<String>,
    pub keywords: Option<String>,
    pub author: Option<String>,
    pub input_data: Option<String>,
    pub output_data: Option<String>,
    pub input_type: Option<String>,
    pub categories: Option<String>,
    pub citation: Option<String>,
    pub foundational_model: Option<String>,
    pub training_datasheet_uuid: Option<String>,
    pub is_private: Option<bool>,
    pub is_gated: Option<bool>,
    pub ai_model: Option<PatraAiModel>,
}

impl PatraModelMetadata {
    pub fn parse_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = Vec::new();
        if let Some(keywords) = &self.keywords {
            for tag in keywords.split(",") {
                let tag = tag.trim();
                if !tag.is_empty() {
                    tags.push(tag.to_string());
                }
            }
        }

        tags
    }

    pub fn parse_libs(&self) -> Vec<String> {
        let libs = self
            .ai_model
            .as_ref()
            .map(|ai_model| ai_model.framework.clone())
            .flatten();

        let libraries: Vec<String> = match libs {
            Some(libs_str) => libs_str
                .split("/")
                .map(|s| s.to_lowercase().trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            None => Vec::new(),
        };

        libraries
    }

    pub fn description(&self) -> Option<String> {
        self.full_description
            .clone()
            .or_else(|| self.short_description.clone())
            .or_else(|| {
                self.ai_model
                    .as_ref()
                    .and_then(|ai_model| ai_model.description.clone())
            })
    }

    pub fn model_name(&self) -> String {
        format!(
            "{} ({})",
            self.name,
            self.version.clone().unwrap_or_default()
        )
    }

    pub fn locator_url(&self) -> String {
        format!(
            "https://patrabackend.pods.icicleai.tapis.io/modelcard/{}",
            self.uuid
        )
    }

    pub fn license(&self) -> Option<String> {
        self.ai_model
            .as_ref()
            .and_then(|ai_model| ai_model.license.clone())
    }

    pub fn model_type(&self) -> Option<String> {
        self.ai_model
            .as_ref()
            .and_then(|ai_model| ai_model.model_type.clone())
    }
}
