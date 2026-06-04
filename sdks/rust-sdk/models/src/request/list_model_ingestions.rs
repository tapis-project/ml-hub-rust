use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::list_model_ingestions`].

On request success, this will return a [`ListModelIngestionsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelIngestionsRequest {}
impl FluentRequest<'_, ListModelIngestionsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ListModelIngestionsRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ListModelIngestionsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/models-api/ingestions";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///List all model ingestions
    pub fn list_model_ingestions(
        &self,
    ) -> FluentRequest<'_, ListModelIngestionsRequest> {
        FluentRequest {
            client: self,
            params: ListModelIngestionsRequest {},
        }
    }
}
