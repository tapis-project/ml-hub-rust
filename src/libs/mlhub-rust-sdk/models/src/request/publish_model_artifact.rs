use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::publish_model_artifact`].

On request success, this will return a [`PublishModelArtifactResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishModelArtifactRequest {
    pub artifact_id: String,
    pub target_platform: String,
    pub webhook_url: Option<String>,
}
impl FluentRequest<'_, PublishModelArtifactRequest> {
    ///Set the value of the webhook_url field.
    pub fn webhook_url(mut self, webhook_url: &str) -> Self {
        self.params.webhook_url = Some(webhook_url.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PublishModelArtifactRequest> {
    type Output = httpclient::InMemoryResult<crate::model::PublishModelArtifactResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/artifacts/{artifact_id}/publications", artifact_id = self
                .params.artifact_id
            );
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "target_platform" : self.params.target_platform }
                    ),
                );
            if let Some(ref unwrapped) = self.params.webhook_url {
                r = r.json(serde_json::json!({ "webhook_url" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Publish a model artifact to an external platform
    pub fn publish_model_artifact(
        &self,
        artifact_id: &str,
        target_platform: &str,
    ) -> FluentRequest<'_, PublishModelArtifactRequest> {
        FluentRequest {
            client: self,
            params: PublishModelArtifactRequest {
                artifact_id: artifact_id.to_owned(),
                target_platform: target_platform.to_owned(),
                webhook_url: None,
            },
        }
    }
}
