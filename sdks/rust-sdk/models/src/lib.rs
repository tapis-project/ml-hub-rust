use std::sync::OnceLock;
use std::borrow::Cow;
use httpclient::Client;
pub mod request;
pub mod model;
pub fn default_http_client() -> Client {
    Client::new()
        .base_url(
            std::env::var("ML_HUB_MODELS_BASE_URL")
                .expect("Missing environment variable ML_HUB_MODELS_BASE_URL")
                .as_str(),
        )
}
static SHARED_HTTPCLIENT: OnceLock<Client> = OnceLock::new();
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}
fn shared_http_client() -> Cow<'static, Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a MlHubModelsClient,
    pub params: T,
}
pub struct MlHubModelsClient {
    client: Cow<'static, Client>,
}
impl MlHubModelsClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
        }
    }
    pub fn new() -> Self {
        Self {
            client: shared_http_client(),
        }
    }
}
impl MlHubModelsClient {}
