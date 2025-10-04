use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`MlHubModelsClient::list_publications_for_artifact`].

On request success, this will return a [`ListModelPublicationsForArtifactResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPublicationsForArtifactRequest {
    pub artifact_id: String,
}
impl FluentRequest<'_, ListPublicationsForArtifactRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ListPublicationsForArtifactRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ListModelPublicationsForArtifactResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/models-api/artifacts/{artifact_id}/publications", artifact_id = self
                .params.artifact_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::MlHubModelsClient {
    ///List all publications for an artifact
    pub fn list_publications_for_artifact(
        &self,
        artifact_id: &str,
    ) -> FluentRequest<'_, ListPublicationsForArtifactRequest> {
        FluentRequest {
            client: self,
            params: ListPublicationsForArtifactRequest {
                artifact_id: artifact_id.to_owned(),
            },
        }
    }
}
