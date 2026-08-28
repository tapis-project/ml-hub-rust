use super::{ListAgentsQueryParams, Scope};

#[test]
fn list_agents_query_defaults_to_owned_without_endpoint_enrichment() -> Result<(), serde_json::Error>
{
    let query: ListAgentsQueryParams = serde_json::from_str("{}")?;

    assert!(matches!(query.scope, Scope::Owned));
    assert!(!query.include_endpoints);

    Ok(())
}

#[test]
fn list_agents_query_accepts_endpoint_enrichment() -> Result<(), serde_json::Error> {
    let query: ListAgentsQueryParams = serde_json::from_str(r#"{"include_endpoints":true}"#)?;

    assert!(query.include_endpoints);

    Ok(())
}
