use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, HttpMessage as _};
use futures_util::future::{ready, Ready};
use crate::application::actor::Actor;

impl FromRequest for Actor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<Actor>() {
            Some(actor) => ready(Ok(actor.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "Missing Actor",
            ))),
        }
    }
}