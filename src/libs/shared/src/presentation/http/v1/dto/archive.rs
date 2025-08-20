use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Archive {
    #[strum(serialize = "zip")]
    Zip,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    #[strum(serialize = "deflated")]
    Deflated,
}
