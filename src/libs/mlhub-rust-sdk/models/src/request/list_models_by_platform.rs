use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::list_models_by_platform`].

On request success, this will return a [`ListModelsByPlatformResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsByPlatformRequest {
    pub platform: String,
}
impl FluentRequest<'_, ListModelsByPlatformRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ListModelsByPlatformRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ListModelsByPlatformResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/platforms/{platform}/models", platform = self.params
                .platform
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///List models from an external platform
    pub fn list_models_by_platform(
        &self,
        platform: &str,
    ) -> FluentRequest<'_, ListModelsByPlatformRequest> {
        FluentRequest {
            client: self,
            params: ListModelsByPlatformRequest {
                platform: platform.to_owned(),
            },
        }
    }
}
