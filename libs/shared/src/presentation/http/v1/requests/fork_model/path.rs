use platforms::Platform;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Clone, Debug, Deserialize, IntoParams)]
pub struct ForkModel {
    pub platform: Platform,
    pub author: String,
    pub name: String,
}