use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, HttpMessage as _};
use futures_util::future::{ready, Ready};
use crate::domain::entities::principal::Principal;

impl FromRequest for Principal {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<Principal>() {
            Some(principal) => ready(Ok(principal.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "Missing Principal",
            ))),
        }
    }
}