use crate::shared_kernel::context::RequestContext;
use actix_web::{dev::Payload, Error, FromRequest, HttpMessage as _, HttpRequest};
use futures_util::future::{ready, Ready};

impl FromRequest for RequestContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<RequestContext>() {
            Some(identity_context) => ready(Ok(identity_context.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "Missing Identity Context",
            ))),
        }
    }
}
