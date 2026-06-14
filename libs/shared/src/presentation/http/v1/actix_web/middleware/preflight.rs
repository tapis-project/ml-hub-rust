use actix_web::{
    HttpResponse,
    middleware::Next,
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    Error,
};

pub async fn preflight_short_circuit(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, Error> {
    if req.method() == Method::OPTIONS {
        return Ok(
            req
                .into_response(HttpResponse::Ok())
                .map_into_right_body()
        );
    }

    Ok(next.call(req).await?.map_into_left_body())
}

