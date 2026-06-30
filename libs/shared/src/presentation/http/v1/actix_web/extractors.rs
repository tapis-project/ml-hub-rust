use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, HttpMessage as _};
use futures_util::future::{ready, Ready};
use crate::shared_kernal::identity::IdentityContext;

impl FromRequest for IdentityContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<IdentityContext>() {
            Some(identity_context) => ready(Ok(identity_context.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "Missing Identity Context",
            ))),
        }
    }
}