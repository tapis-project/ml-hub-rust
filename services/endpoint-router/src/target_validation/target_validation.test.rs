use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use url::Url;

use super::{TargetValidationError, is_public_address, validate_public_https_target};

#[test]
fn rejects_non_public_addresses() {
    let addresses = [
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(169, 254, 1, 1)),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V6("fd00::1".parse().expect("test IPv6 address")),
    ];

    for address in addresses {
        assert!(!is_public_address(address));
    }
}

#[test]
fn accepts_public_addresses() {
    assert!(is_public_address(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
    assert!(is_public_address(IpAddr::V6(
        "2606:4700:4700::1111".parse().expect("test IPv6 address")
    )));
}

#[tokio::test]
async fn rejects_non_https_ip_literal_and_cluster_internal_targets()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        ("http://example.org", TargetValidationError::HttpsRequired),
        (
            "https://127.0.0.1",
            TargetValidationError::IpLiteralNotAllowed,
        ),
        (
            "https://service.namespace.svc",
            TargetValidationError::ClusterInternalHostname,
        ),
    ];

    for (target, expected_error) in cases {
        let target = Url::parse(target)?;
        let error = validate_public_https_target(&target)
            .await
            .expect_err("target should be rejected before DNS resolution");

        assert_eq!(error, expected_error);
    }

    Ok(())
}
