use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::ingest_external_model`].

On request success, this will return a [`IngestModelArtifactResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestExternalModelRequest {
    pub exclude_paths: Option<Vec<String>>,
    pub include_paths: Option<Vec<String>>,
    pub model_id: String,
    pub params: serde_json::Value,
    pub platform: String,
    pub webhook_url: Option<String>,
}
impl FluentRequest<'_, IngestExternalModelRequest> {
    ///Set the value of the exclude_paths field.
    pub fn exclude_paths(
        mut self,
        exclude_paths: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.exclude_paths = Some(
            exclude_paths.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the include_paths field.
    pub fn include_paths(
        mut self,
        include_paths: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.include_paths = Some(
            include_paths.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the webhook_url field.
    pub fn webhook_url(mut self, webhook_url: &str) -> Self {
        self.params.webhook_url = Some(webhook_url.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IngestExternalModelRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IngestModelArtifactResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/platforms/{platform}/models/{model_id}", model_id = self
                .params.model_id, platform = self.params.platform
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.exclude_paths {
                r = r.json(serde_json::json!({ "exclude_paths" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.include_paths {
                r = r.json(serde_json::json!({ "include_paths" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "params" : self.params.params }));
            if let Some(ref unwrapped) = self.params.webhook_url {
                r = r.json(serde_json::json!({ "webhook_url" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Ingest a model from an external platform
    pub fn ingest_external_model(
        &self,
        model_id: &str,
        params: serde_json::Value,
        platform: &str,
    ) -> FluentRequest<'_, IngestExternalModelRequest> {
        FluentRequest {
            client: self,
            params: IngestExternalModelRequest {
                exclude_paths: None,
                include_paths: None,
                model_id: model_id.to_owned(),
                params,
                platform: platform.to_owned(),
                webhook_url: None,
            },
        }
    }
}
