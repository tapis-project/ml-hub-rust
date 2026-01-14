pub mod parameter_set;
pub mod rule_set;
pub mod strategy;
pub mod client_strategy_set;

use uuid::Uuid;

pub struct ModelDeployment {
    pub model_id: String,
    pub model_author: String,
    pub deployment_id: Uuid
}