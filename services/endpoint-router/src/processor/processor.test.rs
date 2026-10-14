use envoy_types::pb::envoy::service::ext_proc::v3::processing_response;

use super::{continue_response, replacement_header, tenant_from_authority};

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
    ]);

    let processing_response::Response::RequestHeaders(headers) =
        response.response.expect("processing response")
    else {
        panic!("expected request headers response");
    };

    let mutation = headers
        .response
        .expect("common response")
        .header_mutation
        .expect("header mutation");

    assert!(mutation.remove_headers.contains(&"authorization".into()));
    assert!(!mutation.remove_headers.contains(&"x-tapis-token".into()));
    assert!(mutation.set_headers.iter().any(|header| {
        header
            .header
            .as_ref()
            .is_some_and(|header| header.key == ":authority" && header.value == "example.org")
    }));
}
