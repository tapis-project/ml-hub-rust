use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::Platform;
/**You should use this struct via [`MlHubModelsClient::get_model_by_platform`].

On request success, this will return a [`GetModelByPlatformResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelByPlatformRequest {
    pub model_id: String,
    pub platform: Platform,
}
impl FluentRequest<'_, GetModelByPlatformRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetModelByPlatformRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetModelByPlatformResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/platforms/{platform}/models/{model_id}", model_id = self
                .params.model_id, platform = self.params.platform
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Fetch a model from an external platform by id
    pub fn get_model_by_platform(
        &self,
        model_id: &str,
        platform: Platform,
    ) -> FluentRequest<'_, GetModelByPlatformRequest> {
        FluentRequest {
            client: self,
            params: GetModelByPlatformRequest {
                model_id: model_id.to_owned(),
                platform,
            },
        }
    }
}
