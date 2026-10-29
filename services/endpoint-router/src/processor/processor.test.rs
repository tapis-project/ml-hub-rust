use envoy_types::pb::envoy::service::ext_proc::v3::processing_response;

use super::{
    UPSTREAM_HOST_HEADER, UPSTREAM_PORT_HEADER, continue_response, replacement_header,
    tenant_from_authority,
};

#[test]
fn derives_tenant_from_tenant_host() {
    assert_eq!(
        tenant_from_authority("tenant-a.develop.tapis.io:443"),
        Some("tenant-a".into())
    );
}

#[test]
fn routing_response_rewrites_authority_and_strips_sensitive_headers() {
    let response = continue_response(vec![
        replacement_header(":authority", "example.org"),
        replacement_header(":path", "/chat"),
        replacement_header(UPSTREAM_HOST_HEADER, "example.org"),
        replacement_header(UPSTREAM_PORT_HEADER, "443"),
    ]);

    let processing_response::Response::RequestHeaders(headers) =
        response.response.expect("processing response")
    else {
        panic!("expected request headers response");
    };

    let response = headers.response.expect("common response");

    assert!(!response.clear_route_cache);

    let mutation = response.header_mutation.expect("header mutation");

    assert!(mutation.remove_headers.contains(&"authorization".into()));
    assert!(!mutation.remove_headers.contains(&"x-tapis-token".into()));
    assert!(
        !mutation
            .remove_headers
            .contains(&UPSTREAM_HOST_HEADER.into())
    );
    assert!(mutation.set_headers.iter().any(|header| {
        header
            .header
            .as_ref()
            .is_some_and(|header| header.key == ":authority" && header.raw_value == b"example.org")
    }));
    assert!(mutation.set_headers.iter().any(|header| {
        header.header.as_ref().is_some_and(|header| {
            header.key == UPSTREAM_HOST_HEADER && header.raw_value == b"example.org"
        })
    }));
    assert!(mutation.set_headers.iter().any(|header| {
        header
            .header
            .as_ref()
            .is_some_and(|header| header.key == UPSTREAM_PORT_HEADER && header.raw_value == b"443")
    }));
}
