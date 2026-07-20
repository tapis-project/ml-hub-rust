#[cfg(test)]
mod client_strategy_test {
    use serde_json::Value;

    use crate::{domain::entities::deployment_strategy::{client_strategy::ClientStrategy, parameter_set::{Parameter, ParameterSet, ParameterType}, rule_set::{Rule, RuleSet}}, shared_kernel::enums::DeploymentModality};

    #[test]
    fn test_init() {
        let valid = ClientStrategy::new(
            "foo".into(),
            Some("Test Client Strategy".into()),
            DeploymentModality::Service,
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
        );

        assert!(valid.is_ok());

        let missing_refs = ClientStrategy::new(
            "foo".into(),
            Some("Test Client Strategy".into()),
            DeploymentModality::Service,
            None,
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
        );

        assert!(missing_refs.is_err())
    }
}