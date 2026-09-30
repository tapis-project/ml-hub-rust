use super::ListDeploymentOptionsInput;

#[test]
fn defaults_pagination_options() {
    let input = ListDeploymentOptionsInput::new(None, None, None);

    assert_eq!(input.limit(), 100);
    assert_eq!(input.cursor(), None);
    assert!(!input.include_count());
}

#[test]
fn clamps_limit_to_supported_range() {
    let minimum = ListDeploymentOptionsInput::new(Some(0), None, None);
    let maximum = ListDeploymentOptionsInput::new(Some(101), None, None);

    assert_eq!(minimum.limit(), 1);
    assert_eq!(maximum.limit(), 100);
}
