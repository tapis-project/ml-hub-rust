#[cfg(test)]
mod list_agent_records_query_test {
    use super::super::{ListAgentRecordsQueryParams, Scope};

    #[test]
    fn defaults_scope_to_owned() -> Result<(), serde_json::Error> {
        let query: ListAgentRecordsQueryParams = serde_json::from_str("{}")?;

        assert!(matches!(query.scope, Scope::Owned));
        Ok(())
    }

    #[test]
    fn accepts_supported_scopes() -> Result<(), serde_json::Error> {
        let owned: ListAgentRecordsQueryParams = serde_json::from_str(r#"{"scope":"Owned"}"#)?;
        let shared: ListAgentRecordsQueryParams = serde_json::from_str(r#"{"scope":"Shared"}"#)?;

        assert!(matches!(owned.scope, Scope::Owned));
        assert!(matches!(shared.scope, Scope::Shared));
        Ok(())
    }

    #[test]
    fn rejects_removed_and_unknown_scopes() {
        let removed =
            serde_json::from_str::<ListAgentRecordsQueryParams>(r#"{"scope":"SharedPublic"}"#);
        let unknown = serde_json::from_str::<ListAgentRecordsQueryParams>(r#"{"scope":"Global"}"#);

        assert!(removed.is_err());
        assert!(unknown.is_err());
    }
}
