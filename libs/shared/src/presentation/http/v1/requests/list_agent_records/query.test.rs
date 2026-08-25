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
        let shared_public: ListAgentRecordsQueryParams =
            serde_json::from_str(r#"{"scope":"SharedPublic"}"#)?;

        assert!(matches!(owned.scope, Scope::Owned));
        assert!(matches!(shared_public.scope, Scope::SharedPublic));
        Ok(())
    }

    #[test]
    fn rejects_unknown_scope() {
        let result = serde_json::from_str::<ListAgentRecordsQueryParams>(r#"{"scope":"Global"}"#);

        assert!(result.is_err());
    }
}
