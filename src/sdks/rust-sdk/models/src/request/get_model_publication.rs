use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::get_model_publication`].

On request success, this will return a [`GetModelPublicationResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelPublicationRequest {
    pub publication_id: String,
}
impl FluentRequest<'_, GetModelPublicationRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetModelPublicationRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetModelPublicationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/publications/{publication_id}", publication_id = self.params
                .publication_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Fetch a publication by id
    pub fn get_model_publication(
        &self,
        publication_id: &str,
    ) -> FluentRequest<'_, GetModelPublicationRequest> {
        FluentRequest {
            client: self,
            params: GetModelPublicationRequest {
                publication_id: publication_id.to_owned(),
            },
        }
    }
}
