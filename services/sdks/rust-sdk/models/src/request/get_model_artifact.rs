use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::get_model_artifact`].

On request success, this will return a [`GetModelArtifactResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelArtifactRequest {
    pub artifact_id: String,
}
impl FluentRequest<'_, GetModelArtifactRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetModelArtifactRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetModelArtifactResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/artifacts/{artifact_id}", artifact_id = self.params
                .artifact_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///Fetches the model artifact by the provided id
    pub fn get_model_artifact(
        &self,
        artifact_id: &str,
    ) -> FluentRequest<'_, GetModelArtifactRequest> {
        FluentRequest {
            client: self,
            params: GetModelArtifactRequest {
                artifact_id: artifact_id.to_owned(),
            },
        }
    }
}
