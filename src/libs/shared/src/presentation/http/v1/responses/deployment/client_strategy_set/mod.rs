pub mod entity_to_response;

use super::rule_set::RuleSet;
use super::strategy::Strategy;
use super::parameter_set::ParameterSet;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ClientStrategySet {
    pub client: String,
    pub description: Option<String>,
    pub rule_sets: Option<Vec<RuleSet>>,
    pub parameter_sets: Option<Vec<ParameterSet>>,
    pub strategies: Vec<Strategy>
}