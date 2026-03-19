use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, HttpMessage as _};
use futures_util::future::{ready, Ready};
use crate::domain::entities::identity::FederatedIdentity;

impl FromRequest for FederatedIdentity {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<FederatedIdentity>() {
            Some(identity) => ready(Ok(identity.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "Missing FederatedIdentity",
            ))),
        }
    }
}