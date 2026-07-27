#[cfg(test)]
mod client_strategy_test {
    use crate::domain::entities::{deployment_strategy::{
            client_strategy::ClientStrategyError, rule_set::{Rule, RuleSet}, test_fixtures::ReconstitutedClientStrategyBuilder
        }, operator::Operator};

    #[test]
    fn test_valid_client_strategy_reconsititution() -> Result<(), ClientStrategyError>{
        let client_strategy = ReconstitutedClientStrategyBuilder::new()
            .with_name("client-strategy-name".into())
            .with_enabled(false)
            .build_reconstituted()?;

        println!("{:#?}", client_strategy);

        assert!(client_strategy.name == String::from("client-strategy-name"));
        assert!(client_strategy.enabled == Some(false));

        Ok(())
    }

    #[test]
    fn test_invalid_reconstitution_with_empty_rule_sets() -> Result<(), ClientStrategyError>{
        let result = ReconstitutedClientStrategyBuilder::new()
            .with_rule_sets(Some(vec![]))
            .build_reconstituted();

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_reconstitution_with_non_empty_rule_set() -> Result<(), ClientStrategyError>{
        let result = ReconstitutedClientStrategyBuilder::new()
            .with_rule_set(RuleSet {
                name: "rule-set-1".into(),
                rules: vec![
                    // Rule number 1
                    Rule {
                        field_path: vec!["im".into()],
                        operator: Operator::Eq,
                        value: "#1".into(),
                    },
                    // Rule number 2
                    Rule {
                        field_path: vec!["crocs".into()],
                        operator: Operator::Eq,
                        value: "#2".into(),
                    },
                    // What's rule 3?
                    // ...
                    // So ya don't know rule 3?
                ]

            })
            .build_reconstituted();

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_invalid_reconstitution_with_empty_rule_set_references() -> Result<(), ClientStrategyError>{
        let result = ReconstitutedClientStrategyBuilder::new()
            .with_rule_set_references(vec![])
            .build_reconstituted();

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_invalid_reconstitution_with_non_empty_rule_set_references() -> Result<(), ClientStrategyError>{
        let result = ReconstitutedClientStrategyBuilder::new()
            .with_rule_set_references(vec!["rule-set-1".into()])
            .build_reconstituted();

        assert!(result.is_ok());

        Ok(())
    }
}