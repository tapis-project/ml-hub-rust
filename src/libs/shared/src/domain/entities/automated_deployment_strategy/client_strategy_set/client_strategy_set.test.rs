#[cfg(test)]
mod client_strategy_set_test {
    use serde_json::Value;

    use crate::domain::entities::automated_deployment_strategy::{client_strategy::ClientStrategy, client_strategy_set::ClientStrategySet, parameter_set::{Parameter, ParameterSet}, rule_set::{Rule, RuleSet}};

    #[test]
    fn test_init() {
        let client_strategy_set = ClientStrategySet::new(
            "test-client".into(),
            Some("Test client description".into()),
            vec![
                ClientStrategy::new(
                    "foo".into(),
                    Some("Test Client Strategy".into()),
                    Some(vec![
                        RuleSet {
                            name: "foo".into(),
                            rules: vec![
                                Rule {
                                    field_path: vec!["name".into()],
                                    operator: crate::domain::entities::operator::Operator::Eq,
                                    value:  Value::String("foo".into()),
                                }
                            ]
                        }
                    ]),
                    Some(ParameterSet {
                        name: "foo-params".into(),
                        parameters: vec![
                            Parameter {
                                name: "foo-param".into()
                            }
                        ]
                    }),
                    None,
                    Some("client-parmater-set".into()),
                ).unwrap()
            ],
            None,
            None,
        );

        assert!(client_strategy_set.is_ok());

    }
}