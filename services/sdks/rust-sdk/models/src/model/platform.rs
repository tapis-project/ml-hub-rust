use serde::{Serialize, Deserialize};
/**Represents a platform for which there are clients registered for one or more of
the following APIs: Models, Datasets, Inference, Training. The strum(serialize="")
attribute corresponds to the desired "platform" path parameter passed to the
`get_client` method of a registrar.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Platform {
    #[serde(rename = "huggingface")]
    Huggingface,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "patra")]
    Patra,
}
