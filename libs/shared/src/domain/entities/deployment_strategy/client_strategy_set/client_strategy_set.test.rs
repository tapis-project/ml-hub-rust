#[cfg(test)]
mod client_strategy_set_test {
    use serde_json::Value;

    use crate::domain::entities::deployment_strategy::{client_strategy::ClientStrategy, client_strategy_set::ClientStrategySet, parameter_set::{Parameter, ParameterSet, ParameterType}, rule_set::{Rule, RuleSet}};

    #[test]
    fn test_init() {
        let client_strategy_set = ClientStrategySet::new(
            platforms::Platform::TapisPods,
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
                                name: "foo-param".into(),
                                required: true,
                                secret: false,
                                description: Some("bar".into()),
                                r#type: ParameterType::String,
                                choices: None,
                                default: Some("foo".into())
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