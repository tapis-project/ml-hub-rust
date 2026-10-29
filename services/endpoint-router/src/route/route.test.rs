use url::Url;

use super::EndpointInvocation;

#[test]
fn parses_slug_path_and_query() -> Result<(), Box<dyn std::error::Error>> {
    let invocation = EndpointInvocation::parse("/v3/mlhub/endpoints/abc/chat?a=1")?;

    assert_eq!(invocation.slug, "abc");
    assert_eq!(invocation.remaining_path_and_query, "/chat?a=1");

    Ok(())
}

#[test]
fn joins_target_base_path() -> Result<(), Box<dyn std::error::Error>> {
    let invocation = EndpointInvocation::parse("/v3/mlhub/endpoints/abc/chat?a=1")?;
    let base_url = Url::parse("https://example.org/api/")?;

    assert_eq!(invocation.upstream_path(&base_url), "/api/chat?a=1");

    Ok(())
}
