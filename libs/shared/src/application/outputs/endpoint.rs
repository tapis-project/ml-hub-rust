use crate::domain::entities::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub struct EndpointListOutput {
    pub endpoints: Vec<Endpoint>,
    pub cursor: Option<String>,
    pub count: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct ResolvedEndpointOutput {
    pub endpoint: Endpoint,
    pub target_base_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ResolvedEndpointListOutput {
    pub endpoints: Vec<ResolvedEndpointOutput>,
    pub cursor: Option<String>,
    pub count: Option<u64>,
}
