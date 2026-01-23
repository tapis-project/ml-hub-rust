use serde_json::Value;
use platforms::Platform;

pub struct DeployWithStrategyInput {
    pub owner: String,
    pub platform: Platform,
    pub model_name: String,
    pub model_author: String,
    pub strategy_name: String,
    pub params: Value,
}