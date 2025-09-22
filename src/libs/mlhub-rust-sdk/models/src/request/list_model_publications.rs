use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::list_model_publications`].

On request success, this will return a [`ListModelPublicationsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelPublicationsRequest {}
impl FluentRequest<'_, ListModelPublicationsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ListModelPublicationsRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ListModelPublicationsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/models-api/publications";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///List all model publications
    pub fn list_model_publications(
        &self,
    ) -> FluentRequest<'_, ListModelPublicationsRequest> {
        FluentRequest {
            client: self,
            params: ListModelPublicationsRequest {},
        }
    }
}
