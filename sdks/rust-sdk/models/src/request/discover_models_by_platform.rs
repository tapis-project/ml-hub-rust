use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{ModelMetadata, Platform};
/**You should use this struct via [`MlHubModelsClient::discover_models_by_platform`].

On request success, this will return a [`DiscoverModelsByPlatformResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverModelsByPlatformRequest {
    pub confidence_threshold: Option<Vec<String>>,
    pub criteria: Vec<ModelMetadata>,
    pub platform: Platform,
}
impl FluentRequest<'_, DiscoverModelsByPlatformRequest> {
    ///Set the value of the confidence_threshold field.
    pub fn confidence_threshold(
        mut self,
        confidence_threshold: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.params.confidence_threshold = Some(
            confidence_threshold.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DiscoverModelsByPlatformRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::DiscoverModelsByPlatformResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/platforms/{platform}/models", platform = self.params
                .platform
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.confidence_threshold {
                r = r.json(serde_json::json!({ "confidence_threshold" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "criteria" : self.params.criteria }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Discover models from external platforms
    pub fn discover_models_by_platform(
        &self,
        criteria: Vec<ModelMetadata>,
        platform: Platform,
    ) -> FluentRequest<'_, DiscoverModelsByPlatformRequest> {
        FluentRequest {
            client: self,
            params: DiscoverModelsByPlatformRequest {
                confidence_threshold: None,
                criteria,
                platform,
            },
        }
    }
}
