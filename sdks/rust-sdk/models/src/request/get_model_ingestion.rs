use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::get_model_ingestion`].

On request success, this will return a [`GetModelIngestionResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelIngestionRequest {
    pub ingestion_id: String,
}
impl FluentRequest<'_, GetModelIngestionRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetModelIngestionRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetModelIngestionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/ingestions/{ingestion_id}", ingestion_id = self.params
                .ingestion_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Fetch an ingestion by id
    pub fn get_model_ingestion(
        &self,
        ingestion_id: &str,
    ) -> FluentRequest<'_, GetModelIngestionRequest> {
        FluentRequest {
            client: self,
            params: GetModelIngestionRequest {
                ingestion_id: ingestion_id.to_owned(),
            },
        }
    }
}
