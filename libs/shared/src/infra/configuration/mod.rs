pub mod site_configuration_loader;

use serde::Deserialize;
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
    pub endpoint_router: EndpointRouterConfiguration,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndpointRouterConfiguration {
    pub public_url_template: String,
    #[serde(default = "default_route_cache_ttl_seconds")]
    pub route_cache_ttl_seconds: u64,
    #[serde(default = "default_route_cache_capacity")]
    pub route_cache_capacity: u64,
}

impl EndpointRouterConfiguration {
    pub fn build_public_url(&self, tenant_id: &str, slug: &str) -> String {
        self.public_url_template
            .replace("{tenant}", tenant_id)
            .replace("{slug}", slug)
    }
}

fn default_route_cache_ttl_seconds() -> u64 {
    30
}

fn default_route_cache_capacity() -> u64 {
    10_000
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TenancyResolutionMode {
    Subdomain,
}
