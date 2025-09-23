use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::list_model_artifacts`].

On request success, this will return a [`ListModelArtifactResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelArtifactsRequest {}
impl FluentRequest<'_, ListModelArtifactsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ListModelArtifactsRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ListModelArtifactResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/models-api/artifacts";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///List all model artifacts
    pub fn list_model_artifacts(&self) -> FluentRequest<'_, ListModelArtifactsRequest> {
        FluentRequest {
            client: self,
            params: ListModelArtifactsRequest {},
        }
    }
}
