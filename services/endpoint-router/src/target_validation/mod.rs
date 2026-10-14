use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use thiserror::Error;
use tokio::net::lookup_host;
use url::{Host, Url};

pub async fn validate_public_https_target(url: &Url) -> Result<(), TargetValidationError> {
    if url.scheme() != "https" {
        return Err(TargetValidationError::HttpsRequired);
    }

    if !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() {
        return Err(TargetValidationError::UnsupportedUrlComponents);
    }

    let hostname = match url.host() {
        Some(Host::Domain(hostname)) => hostname,
        Some(Host::Ipv4(_)) | Some(Host::Ipv6(_)) => {
            return Err(TargetValidationError::IpLiteralNotAllowed);
        }
        None => return Err(TargetValidationError::MissingHostname),
    };

    let hostname_lowercase = hostname.to_ascii_lowercase();

    if hostname_lowercase == "localhost"
        || hostname_lowercase.ends_with(".localhost")
        || hostname_lowercase.ends_with(".local")
        || hostname_lowercase.ends_with(".cluster.local")
        || hostname_lowercase.ends_with(".svc")
    {
        return Err(TargetValidationError::ClusterInternalHostname);
    }

    let port = url
        .port_or_known_default()
        .ok_or(TargetValidationError::MissingPort)?;
    let addresses = lookup_host((hostname, port))
        .await
        .map_err(|_| TargetValidationError::DnsResolutionFailed)?
        .collect::<Vec<_>>();

    if addresses.is_empty() {
        return Err(TargetValidationError::DnsResolutionFailed);
    }

    if addresses
        .iter()
        .any(|address| !is_public_address(address.ip()))
    {
        return Err(TargetValidationError::UnsafeAddress);
    }

    Ok(())
}

fn is_public_address(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => is_public_ipv4(address),
        IpAddr::V6(address) => is_public_ipv6(address),
    }
}

fn is_public_ipv4(address: Ipv4Addr) -> bool {
    let octets = address.octets();

    !address.is_private()
        && !address.is_loopback()
        && !address.is_link_local()
        && !address.is_multicast()
        && !address.is_unspecified()
        && !address.is_broadcast()
        && octets[0] != 0
        && !(octets[0] == 100 && (64..=127).contains(&octets[1]))
        && !(octets[0] == 192 && octets[1] == 0 && octets[2] == 0)
        && !(octets[0] == 192 && octets[1] == 0 && octets[2] == 2)
        && !(octets[0] == 198 && (18..=19).contains(&octets[1]))
        && !(octets[0] == 198 && octets[1] == 51 && octets[2] == 100)
        && !(octets[0] == 203 && octets[1] == 0 && octets[2] == 113)
        && octets[0] < 224
}

fn is_public_ipv6(address: Ipv6Addr) -> bool {
    let segments = address.segments();

    if let Some(mapped) = address.to_ipv4_mapped() {
        return is_public_ipv4(mapped);
    }

    !address.is_loopback()
        && !address.is_multicast()
        && !address.is_unspecified()
        && (segments[0] & 0xfe00) != 0xfc00
        && (segments[0] & 0xffc0) != 0xfe80
        && !(segments[0] == 0x2001 && segments[1] == 0x0db8)
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TargetValidationError {
    #[error("Endpoint target must use HTTPS")]
    HttpsRequired,

    #[error("Endpoint target must have a DNS hostname")]
    MissingHostname,

    #[error("Endpoint target IP literals are not allowed")]
    IpLiteralNotAllowed,

    #[error("Endpoint target includes unsupported URL components")]
    UnsupportedUrlComponents,

    #[error("Endpoint target uses a cluster-internal hostname")]
    ClusterInternalHostname,

    #[error("Endpoint target port could not be resolved")]
    MissingPort,

    #[error("Endpoint target DNS lookup failed")]
    DnsResolutionFailed,

    #[error("Endpoint target resolves to a non-public address")]
    UnsafeAddress,
}

#[cfg(test)]
#[path = "target_validation.test.rs"]
mod target_validation_test;
