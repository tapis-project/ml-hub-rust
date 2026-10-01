pub mod site_configuration_loader;

use serde::Deserialize;
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

// Infra
use crate::infra::identity::Idp;

pub const DEFAULT_SITE_CONFGIURATION_PATH: &'static str = "/etc/mlhub/site.json";

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfiguration {
    pub site_id: String,
    pub base_url: String,
    pub idps: Vec<Idp>,
    pub tenancy_resolution_mode: TenancyResolutionMode,
    pub model_deployment: ModelDeploymentConfiguration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelDeploymentConfiguration {
    pub tapis_jobs: TapisJobsConfiguration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TapisJobsConfiguration {
    pub hpc_systems: HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TenancyResolutionMode {
    Subdomain,
}
