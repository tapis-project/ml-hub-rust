use crate::domain::entities::agent as entities;
use crate::presentation::http::v1::responses::agents as responses;
use crate::presentation::http::v1::responses::endpoints::Endpoint;

impl From<(entities::Agent, Vec<Endpoint>)> for responses::Agent {
    fn from(value: (entities::Agent, Vec<Endpoint>)) -> Self {
        let mut response = Self::from(value.0);
        response.endpoints = value.1;

        response
    }
}
