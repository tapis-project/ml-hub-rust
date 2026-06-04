use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::list_platforms`].

On request success, this will return a [`ListPlatformsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPlatformsRequest {}
impl FluentRequest<'_, ListPlatformsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ListPlatformsRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ListPlatformsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/models-api/platforms";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///List all external platforms integrated with this deployment of MLHub
    pub fn list_platforms(&self) -> FluentRequest<'_, ListPlatformsRequest> {
        FluentRequest {
            client: self,
            params: ListPlatformsRequest {},
        }
    }
}
