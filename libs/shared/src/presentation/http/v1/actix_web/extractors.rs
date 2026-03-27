use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, HttpMessage as _};
use futures_util::future::{ready, Ready};
use crate::domain::entities::identity::FederatedIdentity;
use crate::domain::entities::principal::{NewUserPrincipalProps, Principal};

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


impl FromRequest for Principal {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = match req.extensions().get::<Option<FederatedIdentity>>() {
            Some(Some(identity)) => identity.clone(),
            _ => {
                return ready(Err(actix_web::error::ErrorInternalServerError(
                    "Missing FederatedIdentity",
                )))
            }
        };

        let props = NewUserPrincipalProps {
            id: identity.subject.clone(),
            tenant_id: identity.tenant_id.clone(),
            identities: vec![identity],
        };

        match Principal::new_user(props) {
            Ok(principal) => ready(Ok(principal)),
            Err(err) => ready(Err(actix_web::error::ErrorInternalServerError(err.to_string()))),
        }
    }
}