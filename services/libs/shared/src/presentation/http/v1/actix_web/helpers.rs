use std::sync::Arc;
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::presentation::http::v1::requests::headers::AuthToken;
use crate::{domain::entities::identity::FederatedIdentity, presentation::http::v1::responses::JsonResponse};
use crate::presentation::http::v1::requests::Parameters;
use crate::errors::Error;
use actix_web::HttpMessage;
use serde_json::Value;
use actix_web::{HttpRequest, HttpResponse, http::StatusCode};

pub fn param_to_string(params: Option<Parameters>, prop: &str) -> Result<Option<String>, Error> {
    let val = match params.and_then(|mut m| m.remove(prop)) {
        Some(Value::String(s)) => Some(s),
        Some(v) => Some(v.to_string()), // fallback if it's not a string
        None => None,
    };
    Ok(val)
}

pub fn get_header_value(header_key: &str, request: &HttpRequest) -> Option<String> {
    request
        .headers()
        .get(header_key)
        .and_then(|value| value.to_str().ok())
        .map(|value| String::from(value))
}


pub fn build_error_response(status: u16, message: String, version: Option<String>, metadata: Option<Value>) -> HttpResponse {
    match StatusCode::from_u16(status) {
        Ok(code) => {
            return HttpResponse::build(code)
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(status),
                    message: Some(message),
                    result: None,
                    metadata,
                    version,
                })
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(format!("Invalid http status code found: {}", err.to_string())),
                    result: None,
                    metadata,
                    version,
                })
        }
    }
}

pub fn build_success_response(result: Option<Value>, message: Option<String>, version: Option<String>, metadata: Option<Value>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(JsonResponse {
            status: Some(200),
            message,
            result,
            metadata,
            version,
        })
}

pub async fn authenticate(req: HttpRequest, version: String) -> Result<FederatedIdentity, HttpResponse> {
    let ext = req.extensions();
    
    let (token, idp) = match ext.get::<(AuthToken, Arc<dyn FederatedIdentityProvider>)>() {
        Some((t, i)) => (t, i),
        None => return Err(build_error_response(401, "Missing valid credentials for this request".into(), Some(version), None))
    };

    let maybe_identity = match idp.authenticate(token.into_inner()).await {
        Ok(i) => i,
        Err(err) => return Err(build_error_response(500, format!("Error during authentication: {}", err.to_string()), Some(version), None))
    };

    match maybe_identity {
        Some(i) => Ok(i),
        None => return Err(build_error_response(401, "Unauthenticated".into(), Some(version), None))
    }
}