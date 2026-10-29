use super::*;

fn flexserv_version_parameter() -> Parameter {
    Parameter {
        name: "flexserv_version".into(),
        description: None,
        required: false,
        secret: false,
        r#type: ParameterType::String,
        choices: Some(vec![
            Choice {
                value: "1.4".into(),
                description: None,
                enabled: true,
            },
            Choice {
                value: "1.5".into(),
                description: None,
                enabled: false,
            },
        ]),
        default: Some("1.4".into()),
    }
}

#[test]
fn resolves_defaults_and_preserves_secret_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter = flexserv_version_parameter();

    parameter.secret = true;

    let parameters = DeploymentParameters::new(vec![parameter])?;

    let resolved = parameters.resolve(&[])?;

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].name(), "flexserv_version");
    assert_eq!(resolved[0].value(), "1.4");
    assert!(resolved[0].secret());

    Ok(())
}

#[test]
fn rejects_disabled_choices() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = DeploymentParameters::new(vec![flexserv_version_parameter()])?;

    let values = vec![("flexserv_version".into(), "1.5".into())];

    let result = parameters.resolve(&values);

    assert!(matches!(
        result,
        Err(DeploymentParameterError::InvalidArgument(_))
    ));

    Ok(())
}

#[test]
fn rejects_duplicate_parameter_definitions() {
    let result = DeploymentParameters::new(vec![
        flexserv_version_parameter(),
        flexserv_version_parameter(),
    ]);

    assert!(matches!(
        result,
        Err(DeploymentParameterError::DuplicateParameter(name))
            if name == "flexserv_version"
    ));
}
