use std::{collections::HashMap, pin::Pin, sync::Arc};

use envoy_types::pb::envoy::{
    config::core::v3::{HeaderValue, HeaderValueOption, header_value_option::HeaderAppendAction},
    service::ext_proc::v3::{
        CommonResponse, HeaderMutation, HeadersResponse, ImmediateResponse, ProcessingRequest,
        ProcessingResponse, common_response::ResponseStatus,
        external_processor_server::ExternalProcessor, processing_request, processing_response,
    },
    r#type::v3::HttpStatus,
};
use log::{debug, error, warn};
use moka::future::Cache;
use shared::application::services::{
    authentication_service::{AuthenticationServiceError, authenticate_token},
    endpoint_routing_service::{EndpointRoutingService, EndpointRoutingServiceError},
    federated_identity_service::FederatedIdentityService,
    federated_idp_registrar::FederatedIdpRegistrar,
    principal_service::{PrincipalService, PrincipalServiceError},
};
use shared::domain::entities::endpoint::NetworkProtocol;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};

use crate::{
    route::{CachedRoute, EndpointInvocation},
    target_validation::validate_public_https_target,
};

const TOKEN_HEADER: &str = "x-tapis-token";
const UPSTREAM_HOST_HEADER: &str = "x-mlhub-upstream-host";
const UPSTREAM_PORT_HEADER: &str = "x-mlhub-upstream-port";

#[derive(Clone)]
pub struct AuthenticationDependencies {
    pub federated_identity_service: Arc<FederatedIdentityService>,
    pub idp_registrar: Arc<FederatedIdpRegistrar>,
    pub principal_service: Arc<PrincipalService>,
}

pub struct EndpointExternalProcessor {
    authentication: AuthenticationDependencies,
    routing_service: Arc<EndpointRoutingService>,
    route_cache: Cache<String, CachedRoute>,
}

impl EndpointExternalProcessor {
    pub fn new(
        authentication: AuthenticationDependencies,
        routing_service: Arc<EndpointRoutingService>,
        route_cache: Cache<String, CachedRoute>,
    ) -> Self {
        Self {
            authentication,
            routing_service,
            route_cache,
        }
    }

    async fn process_request_headers(&self, request: ProcessingRequest) -> ProcessingResponse {
        let headers = match request.request {
            Some(processing_request::Request::RequestHeaders(headers)) => headers,
            _ => return immediate_response(503, "endpoint_router_invalid_request"),
        };

        let headers = headers
            .headers
            .map(|headers| header_map(headers.headers))
            .unwrap_or_default();

        if unsupported_request(&headers) {
            return immediate_response(502, "endpoint_router_unsupported_protocol");
        }

        let token = match headers.get(TOKEN_HEADER) {
            Some(token) if !token.is_empty() => token.clone(),
            _ => return immediate_response(401, "endpoint_router_missing_credentials"),
        };

        let authority = match headers.get(":authority").or_else(|| headers.get("host")) {
            Some(authority) => authority,
            None => return immediate_response(404, "endpoint_router_missing_tenant_host"),
        };

        let tenant_id = match tenant_from_authority(authority) {
            Some(tenant_id) => tenant_id,
            None => return immediate_response(404, "endpoint_router_invalid_tenant_host"),
        };

        let path = match headers.get(":path") {
            Some(path) => path,
            None => return immediate_response(404, "endpoint_router_missing_path"),
        };

        let invocation = match EndpointInvocation::parse(path) {
            Ok(invocation) => invocation,
            Err(_) => return immediate_response(404, "endpoint_router_invalid_path"),
        };

        debug!(
            "Processing endpoint proxy request; tenant_id='{}'; slug='{}'; method='{}'",
            tenant_id,
            invocation.slug,
            headers
                .get(":method")
                .map(String::as_str)
                .unwrap_or("unknown")
        );

        let context = match authenticate_token(
            token,
            &tenant_id,
            self.authentication.federated_identity_service.as_ref(),
            self.authentication.idp_registrar.as_ref(),
            self.authentication.principal_service.as_ref(),
        ).await {
            Ok(context) => context,
            Err(error) => {
                return authentication_error_response(error, &tenant_id, &invocation.slug);
            }
        };

        debug!(
            "Authenticated endpoint proxy request; tenant_id='{}'; slug='{}'; principal_id='{}'",
            tenant_id,
            invocation.slug,
            context.actor_principal_id()
        );

        let cache_key = format!("{tenant_id}:{}", invocation.slug);

        let route = match self.route_cache.get(&cache_key).await {
            Some(route) => {
                debug!(
                    "Resolved endpoint proxy route from cache; tenant_id='{}'; slug='{}'; endpoint_id='{}'",
                    tenant_id, invocation.slug, route.endpoint_id
                );

                if route.owner != *context.actor_principal_id() {
                    return immediate_response(403, "endpoint_router_forbidden");
                }

                route
            }
            None => {
                debug!(
                    "Resolving endpoint proxy route from persistence; tenant_id='{}'; slug='{}'",
                    tenant_id, invocation.slug
                );

                let resolved = match self
                    .routing_service
                    .resolve(&context, &invocation.slug)
                    .await
                {
                    Ok(resolved) => resolved,
                    Err(error) => {
                        return routing_error_response(error, &tenant_id, &invocation.slug);
                    }
                };

                let route = CachedRoute {
                    endpoint_id: *resolved.endpoint_id(),
                    owner: resolved.owner().into(),
                    protocol: resolved.target().protocol().clone(),
                    target_url: resolved.target().base_url().clone(),
                };

                if route.protocol != NetworkProtocol::RestHttp {
                    return immediate_response(502, "endpoint_router_unsupported_protocol");
                }

                if let Err(error) = validate_public_https_target(&route.target_url).await {
                    warn!(
                        "Rejected unsafe endpoint proxy target; tenant_id='{}'; slug='{}'; endpoint_id='{}'; reason='{}'",
                        tenant_id, invocation.slug, route.endpoint_id, error
                    );

                    return immediate_response(502, "endpoint_router_invalid_target");
                }

                debug!(
                    "Resolved and validated endpoint proxy route; tenant_id='{}'; slug='{}'; endpoint_id='{}'; target_host='{}'; target_port={}",
                    tenant_id,
                    invocation.slug,
                    route.endpoint_id,
                    route.target_url.host_str().unwrap_or("unknown"),
                    route.target_url.port_or_known_default().unwrap_or_default()
                );

                self.route_cache.insert(cache_key, route.clone()).await;

                route
            }
        };

        let target_host = route.target_url.host_str().unwrap_or_default();
        let target_port = route.target_url.port_or_known_default().unwrap_or(443);

        let authority = match route.target_url.port() {
            Some(port) => format!("{target_host}:{port}"),
            None => target_host.into(),
        };

        let upstream_path = invocation.upstream_path(&route.target_url);

        debug!(
            "Forwarding endpoint proxy request; tenant_id='{}'; slug='{}'; endpoint_id='{}'; target_host='{}'; target_port={}",
            tenant_id,
            invocation.slug,
            route.endpoint_id,
            route.target_url.host_str().unwrap_or("unknown"),
            route.target_url.port_or_known_default().unwrap_or_default()
        );

        continue_response(vec![
            replacement_header(":authority", &authority),
            replacement_header(":scheme", "https"),
            replacement_header(":path", &upstream_path),
            replacement_header(UPSTREAM_HOST_HEADER, target_host),
            replacement_header(UPSTREAM_PORT_HEADER, &target_port.to_string()),
        ])
    }
}

#[tonic::async_trait]
impl ExternalProcessor for EndpointExternalProcessor {
    type ProcessStream =
        Pin<Box<dyn Stream<Item = Result<ProcessingResponse, Status>> + Send + 'static>>;

    async fn process(
        &self,
        request: Request<Streaming<ProcessingRequest>>,
    ) -> Result<Response<Self::ProcessStream>, Status> {
        let mut stream = request.into_inner();

        let (sender, receiver) = mpsc::channel(1);

        let response = match stream.next().await {
            Some(Ok(request)) => self.process_request_headers(request).await,
            Some(Err(error)) => return Err(error),
            None => return Err(Status::invalid_argument("empty external processing stream")),
        };

        sender
            .send(Ok(response))
            .await
            .map_err(|_| Status::unavailable("external processing response channel closed"))?;

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
    }
}

fn header_map(headers: Vec<HeaderValue>) -> HashMap<String, String> {
    headers
        .into_iter()
        .filter_map(|header| {
            let value = if header.raw_value.is_empty() {
                header.value
            } else {
                String::from_utf8(header.raw_value).ok()?
            };

            Some((header.key.to_ascii_lowercase(), value))
        })
        .collect()
}

fn tenant_from_authority(authority: &str) -> Option<String> {
    let hostname = authority
        .trim_matches(['[', ']'])
        .split(':')
        .next()?
        .trim_end_matches('.');

    let tenant_id = hostname.split('.').next()?;

    (!tenant_id.is_empty()).then(|| tenant_id.into())
}

fn unsupported_request(headers: &HashMap<String, String>) -> bool {
    headers
        .get(":method")
        .is_some_and(|method| method.eq_ignore_ascii_case("CONNECT"))
        || headers
            .get("upgrade")
            .is_some_and(|value| !value.is_empty())
        || headers
            .get("content-type")
            .is_some_and(|value| value.to_ascii_lowercase().starts_with("application/grpc"))
}

#[allow(deprecated)]
fn replacement_header(name: &str, value: &str) -> HeaderValueOption {
    HeaderValueOption {
        header: Some(HeaderValue {
            key: name.into(),
            value: String::new(),
            raw_value: value.as_bytes().to_vec(),
        }),
        append: None,
        append_action: HeaderAppendAction::OverwriteIfExistsOrAdd as i32,
        keep_empty_value: false,
    }
}

fn continue_response(headers: Vec<HeaderValueOption>) -> ProcessingResponse {
    ProcessingResponse {
        response: Some(processing_response::Response::RequestHeaders(
            HeadersResponse {
                response: Some(CommonResponse {
                    status: ResponseStatus::Continue as i32,
                    header_mutation: Some(HeaderMutation {
                        set_headers: headers,
                        remove_headers: vec![
                            "authorization".into(),
                            "connection".into(),
                            "keep-alive".into(),
                            "proxy-authenticate".into(),
                            "proxy-authorization".into(),
                            "te".into(),
                            "trailer".into(),
                            "transfer-encoding".into(),
                            "upgrade".into(),
                        ],
                    }),
                    body_mutation: None,
                    trailers: None,
                    // The path now points at the downstream resource. Re-evaluating
                    // routes would make it miss the MLHub endpoint proxy prefix.
                    clear_route_cache: false,
                }),
            },
        )),
        ..Default::default()
    }
}

fn immediate_response(status: u16, details: &str) -> ProcessingResponse {
    match status {
        500..=599 => error!(
            "Endpoint proxy request rejected; status={}; reason='{}'",
            status, details
        ),
        400..=499 => warn!(
            "Endpoint proxy request rejected; status={}; reason='{}'",
            status, details
        ),
        _ => debug!(
            "Endpoint proxy request completed immediately; status={}; reason='{}'",
            status, details
        ),
    }

    ProcessingResponse {
        response: Some(processing_response::Response::ImmediateResponse(
            ImmediateResponse {
                status: Some(HttpStatus {
                    code: i32::from(status),
                }),
                headers: None,
                body: Vec::new(),
                grpc_status: None,
                details: details.into(),
            },
        )),
        ..Default::default()
    }
}

fn authentication_error_response(
    error: AuthenticationServiceError,
    tenant_id: &str,
    slug: &str,
) -> ProcessingResponse {
    let error_message = error.to_string();

    match error {
        AuthenticationServiceError::TenantMismatch { .. } => {
            warn!(
                "Endpoint proxy authentication rejected for tenant mismatch; tenant_id='{}'; slug='{}'; error='{}'",
                tenant_id, slug, error_message
            );

            immediate_response(403, "endpoint_router_tenant_mismatch")
        }
        AuthenticationServiceError::IdentityProvider(
            shared::application::ports::identity::FederatedIdentityProviderError::InternalIdpError(
                _,
            )
            | shared::application::ports::identity::FederatedIdentityProviderError::InitializationError(
                _,
                _,
            ),
        )
        | AuthenticationServiceError::Principal(PrincipalServiceError::InternalError(_)) => {
            error!(
                "Endpoint proxy authentication dependency failed; tenant_id='{}'; slug='{}'; error='{}'",
                tenant_id, slug, error_message
            );

            immediate_response(503, "endpoint_router_authentication_unavailable")
        }
        _ => {
            warn!(
                "Endpoint proxy authentication rejected; tenant_id='{}'; slug='{}'; error='{}'",
                tenant_id, slug, error_message
            );

            immediate_response(401, "endpoint_router_invalid_credentials")
        }
    }
}

fn routing_error_response(
    error: EndpointRoutingServiceError,
    tenant_id: &str,
    slug: &str,
) -> ProcessingResponse {
    let error_message = error.to_string();

    match error {
        EndpointRoutingServiceError::EndpointNotFound => {
            warn!(
                "Endpoint proxy route was not found; tenant_id='{}'; slug='{}'",
                tenant_id, slug
            );

            immediate_response(404, "endpoint_router_not_found")
        }
        EndpointRoutingServiceError::Forbidden => {
            warn!(
                "Endpoint proxy route access was forbidden; tenant_id='{}'; slug='{}'",
                tenant_id, slug
            );

            immediate_response(403, "endpoint_router_forbidden")
        }
        EndpointRoutingServiceError::TargetUnavailable
        | EndpointRoutingServiceError::ReferencedResourceNotFound => {
            warn!(
                "Endpoint proxy target is unavailable; tenant_id='{}'; slug='{}'; error='{}'",
                tenant_id, slug, error_message
            );

            immediate_response(503, "endpoint_router_target_unavailable")
        }
        EndpointRoutingServiceError::DataIntegrity(_)
        | EndpointRoutingServiceError::InvalidUrn(_)
        | EndpointRoutingServiceError::InvalidTarget(_) => {
            error!(
                "Endpoint proxy target data is invalid; tenant_id='{}'; slug='{}'; error='{}'",
                tenant_id, slug, error_message
            );

            immediate_response(502, "endpoint_router_invalid_target")
        }
        EndpointRoutingServiceError::EndpointRepository(_)
        | EndpointRoutingServiceError::AgentRepository(_)
        | EndpointRoutingServiceError::DeploymentRepository(_) => {
            error!(
                "Endpoint proxy persistence lookup failed; tenant_id='{}'; slug='{}'; error='{}'",
                tenant_id, slug, error_message
            );

            immediate_response(503, "endpoint_router_persistence_unavailable")
        }
    }
}

#[cfg(test)]
#[path = "processor.test.rs"]
mod processor_test;
